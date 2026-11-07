"""The built-in courses: copying lessons, checking work in plain English, and tracking progress."""
import json
import os
import re
import shutil
import signal
import subprocess
import tomllib
from dataclasses import dataclass
from pathlib import Path

from .projects import load_state, save_state

COURSES = Path(__file__).resolve().parent.parent / "courses"
REFERENCE_DOCS = ("COURSE.md", "FILES.md")

# Errors without a code number, matched by their wording.
MESSAGE_HELP = {
    "cannot find macro": "Rust can't find that macro. A macro written with macro_rules! has to come before the code that uses it in the file.",
    "no rules expected": "The macro was used in a way none of its rules match. Compare what you typed with each rule's pattern.",
    "expected `;`": "A semicolon is missing. Most lines that do something end with `;`.",
    "unclosed delimiter": "A bracket was opened but never closed. Check that every ( [ { has a matching ) ] }.",
}

# What common Rust compiler errors mean, in everyday words.
ERROR_HELP = {
    "E0384": "You changed a variable that isn't allowed to change. In Rust, variables are locked "
             "unless you add `mut` after `let`, like `let mut score = 0;`.",
    "E0308": "Something is the wrong type. Rust expected one kind of value (like a whole number) "
             "but got another (like text or a decimal). Look at the line it points to and compare "
             "the \"expected\" and \"found\" types.",
    "E0425": "Rust doesn't know that name. Check the spelling, and make sure the variable or "
             "function was created before you use it.",
    "E0433": "Rust can't find that module, crate, or type. If it's one of your own files, check for a `mod` line. "
             "If it's a crate, check Cargo.toml. Otherwise you may need a `use` line.",
    "E0382": "You used a value after giving it away. Once a value moves to a new owner, the old "
             "name can't use it anymore. Use `.clone()` for a copy, or lend it with `&`. Lessons 09 and 10 cover this.",
    "E0499": "Two parts of your code are trying to change the same thing at once. Rust only allows one at a time.",
    "E0502": "You're reading something while also changing it. Rust doesn't allow both at the same time.",
    "E0106": "Rust needs to know how long a borrowed value lives. Add a lifetime like `'a`. Lesson 24 covers this.",
    "E0061": "You called a function with the wrong number of inputs. Compare your call with the "
             "line that starts with `fn`.",
    "E0369": "You used an operator like + or == on values that don't support it.",
    "E0599": "That method doesn't exist for this type. Check the spelling, or the value may be a different type than you think.",
    "E0277": "A type is missing an ability (a trait) that this code needs. The message names the type and the trait. "
             "You may need to write `impl Trait for Type`, or the types may not be what you expect.",
    "E0603": "That item is private. Things in another file can only be used if they're marked `pub`.",
    "E0583": "You wrote `mod` for a module, but Rust can't find its file. Check the file's name and that it's in the src folder.",
    "E0432": "Rust can't find what you're trying to `use`. Check the spelling and path. For your own files, check for "
             "`mod` lines. For a crate, check that it's listed under [dependencies] in Cargo.toml.",
    "E0412": "Rust doesn't know that type. Check the spelling, or bring it in with a `use` line.",
    "E0046": "A trait's required method is missing. Every method the trait lists without a body has to be written in your `impl`.",
    "E0373": "A new thread might outlive the values it uses. Put `move` before the closure so the thread takes ownership of them.",
    "E0072": "This type contains itself, so Rust can't work out how big it is. Put the inner copy in a Box, like `Box<List>`.",
    "E0726": "A type that borrows something needs its lifetime written out, like `Highlight<'a>`.",
    "E0133": "This needs an `unsafe` block. Rust can't check it for you, so you have to mark it to show you've checked it yourself.",
    "E0423": "You used a name in the wrong way, like using a type as if it were a value.",
}


@dataclass
class CheckResult:
    passed: bool
    summary: str
    details: str = ""


def course_source(name="rust"):
    return COURSES / name


def learn_dir(name="rust"):
    return Path.home() / "Learn" / name


def lessons(root):
    root = Path(root)
    if not root.is_dir():
        return []
    return sorted(p for p in root.iterdir() if (p / "lesson.toml").is_file())


def read_meta(lesson):
    with open(Path(lesson) / "lesson.toml", "rb") as handle:
        return tomllib.load(handle)


def find_lesson(start):
    """Find the lesson folder that contains a file or folder."""
    path = Path(start).resolve()
    if path.is_file():
        path = path.parent
    for folder in (path, *path.parents):
        if (folder / "lesson.toml").is_file():
            return folder
    return None


def sync_course(name="rust", source=None, target=None):
    """Copy the course into the learner's folder. Never overwrites a lesson they've started."""
    source = Path(source) if source else course_source(name)
    target = Path(target) if target else learn_dir(name)
    target.mkdir(parents=True, exist_ok=True)
    for doc in REFERENCE_DOCS:
        if (source / doc).is_file():
            shutil.copyfile(source / doc, target / doc)
    if (source / ".vscode").is_dir() and not (target / ".vscode").exists():
        shutil.copytree(source / ".vscode", target / ".vscode")
    added = []
    for lesson in lessons(source):
        destination = target / lesson.name
        if not destination.exists():
            shutil.copytree(lesson, destination)
            added.append(lesson.name)
    return target, added


def progress(name="rust"):
    return set(load_state().get("courses", {}).get(name, {}).get("done", []))


def mark_done(name, lesson_name):
    courses = load_state().get("courses", {})
    course = courses.get(name, {})
    course["done"] = sorted(set(course.get("done", [])) | {lesson_name})
    courses[name] = course
    save_state(courses=courses)


def next_lesson(root, done):
    for lesson in lessons(root):
        if lesson.name not in done:
            return lesson
    return None


# Checking work ----------------------------------------------------------------

def _cargo(args, lesson, timeout):
    """Run cargo in its own process group so a stuck program can be stopped completely."""
    env = {**os.environ, "CARGO_TERM_COLOR": "never", "RUST_BACKTRACE": "0"}
    process = subprocess.Popen(["cargo", *args], cwd=lesson, stdin=subprocess.DEVNULL,
                               stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
                               env=env, start_new_session=True)
    try:
        stdout, stderr = process.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGKILL)
        process.communicate()
        raise
    return process.returncode, stdout, stderr


def _compile_errors(stdout):
    errors = []
    for line in stdout.splitlines():
        try:
            item = json.loads(line)
        except ValueError:
            continue
        message = item.get("message") or {}
        if item.get("reason") == "compiler-message" and message.get("level") == "error":
            code = (message.get("code") or {}).get("code")
            if message.get("message", "").startswith("aborting due to"):
                continue
            spans = [span for span in message.get("spans", []) if span.get("is_primary")] or [{}]
            place = (spans[0].get("file_name", ""), spans[0].get("line_start", 0))
            errors.append((place, code, message.get("rendered", "").strip()))
    # Show problems in the order they appear in each file. The checker sits at the bottom,
    # so the learner's own code comes first.
    errors.sort(key=lambda error: error[0])
    return [(code, rendered) for _, code, rendered in errors]


def _explain_build_failure(errors, stderr, hint):
    if "no targets specified" in stderr:
        return CheckResult(False, "Cargo can't find your program.",
                           "Cargo looks for your code in a file called main.rs inside a folder called "
                           "src. Right now it isn't there, so there's nothing to build."
                           + (f"\n\nHint: {hint}" if hint else ""))
    if not errors:
        return CheckResult(False, "Your code didn't build.", stderr.strip())
    parts = []
    for code, rendered in errors[:3]:
        explanation = ERROR_HELP.get(code) or next(
            (text for words, text in MESSAGE_HELP.items() if words in rendered.splitlines()[0]), None)
        if explanation:
            parts.append(f"In plain English: {explanation}\n\nWhat Rust said:\n{rendered}")
        else:
            parts.append(f"What Rust said:\n{rendered}")
    if len(errors) > 3:
        parts.append(f"...and {len(errors) - 3} more. Fix these first. Others often disappear with them.")
    if hint:
        parts.append(f"Hint: {hint}")
    count = len(errors)
    summary = (f"Rust found {count} problem{'s' if count != 1 else ''} before it could run your code. "
               "That's normal. Reading these messages is a big part of programming.")
    return CheckResult(False, summary, "\n\n".join(parts))


def _normalize(text):
    return "\n".join(line.rstrip() for line in text.strip().splitlines())


def _check_output(lesson, meta):
    hint = meta.get("hint", "")
    try:
        code, stdout, stderr = _cargo(["run", "--quiet"], lesson, timeout=30)
    except subprocess.TimeoutExpired:
        return CheckResult(False, "Your program never finished.",
                           "It may have a loop that never ends, or it's waiting for you to type something.")
    if code != 0:
        return CheckResult(False, "Your program crashed while it was running.", stderr.strip())
    expected = _normalize(meta["expected_output"])
    actual = _normalize(stdout)
    if actual == expected:
        return CheckResult(True, "Your program printed exactly the right thing.")
    details = (f"The lesson expected your program to print:\n\n{expected}\n\n"
               f"Your program printed:\n\n{actual or '(nothing)'}\n\n"
               "Check spelling, spaces, and punctuation. They have to match exactly.")
    if hint:
        details += f"\n\nHint: {hint}"
    return CheckResult(False, "Your program runs, but it doesn't print what the lesson asked for yet.", details)


def _check_tests(lesson, meta):
    hint = meta.get("hint", "")
    try:
        code, stdout, stderr = _cargo(["test", "--quiet"], lesson, timeout=180)
    except subprocess.TimeoutExpired:
        return CheckResult(False, "The checks never finished.",
                           "One of your functions may have a loop that never ends. Look at any `while` or `loop` you wrote.")
    output = stdout + "\n" + stderr
    if code == 0:
        wanted = meta.get("min_tests", 0)
        ran = sum(int(n) for n in re.findall(r"test result: ok\. (\d+) passed", output))
        if ran < wanted:
            return CheckResult(False, f"Your code passes, but this lesson needs at least {wanted} tests and found {ran}.",
                               "Write more tests of your own. Each one is a function marked #[test]"
                               + (" or a code example in a /// comment." if meta.get("doc_tests") else ".")
                               + (f"\n\nHint: {hint}" if hint else ""))
        return CheckResult(True, "Every check passed.")
    blocks = re.findall(r"---- (\S+) stdout ----\n(.*?)(?=\n---- |\nfailures:|\Z)", output, re.S)
    failed = [name.split("::")[-1] for name, _ in blocks]
    if "not yet implemented" in output:
        summary = "Some functions still say todo!(). Replace each todo!() with your own code."
    else:
        summary = f"{len(failed) or 'Some'} check{'s' if len(failed) != 1 else ''} didn't pass yet."
    parts = [_describe_failure(name, body) for name, body in blocks[:4]]
    if any("left:" in body for _, body in blocks):
        parts.append("How to read this: \"left\" is what your code gave back. \"right\" is what the lesson expected.")
    if hint:
        parts.append(f"Hint: {hint}")
    return CheckResult(False, summary, "\n\n".join(parts) or output.strip()[-2000:])


PANIC_LINE = re.compile(r"panicked at (?P<file>[^:\n]+):(?P<line>\d+):\d+:")


def _describe_failure(name, body):
    """Turn one failed test's output into a short, readable note."""
    keep = []
    for line in body.strip().splitlines():
        if line.startswith(("note:", "stack backtrace", "   ")) or re.match(r"\s+\d+: ", line):
            continue
        keep.append(line)
    text = "\n".join(keep)
    match = PANIC_LINE.search(text)
    where = ""
    if match:
        where = f" (stopped at {match['file']}, line {match['line']})"
        text = text[match.end():].strip()
    if text == "not yet implemented":
        text = "This function still says todo!()."
    return f"Check \"{name.split('::')[-1]}\"{where}:\n{text}"


def check_lesson(lesson):
    lesson = Path(lesson)
    meta = read_meta(lesson)
    mode = meta.get("check", "tests")
    build_args = ["test", "--no-run"] if mode == "tests" else ["build"]
    try:
        code, stdout, stderr = _cargo([*build_args, "--message-format=json"], lesson, timeout=300)
    except subprocess.TimeoutExpired:
        return CheckResult(False, "Building took too long.", "Try again. If it keeps happening, restart your PC.")
    if code != 0:
        return _explain_build_failure(_compile_errors(stdout), stderr, meta.get("hint", ""))
    return _check_tests(lesson, meta) if mode == "tests" else _check_output(lesson, meta)


# Commands -----------------------------------------------------------------------

def _title(lesson):
    return read_meta(lesson).get("title", lesson.name)


def open_course(name="rust"):
    if not shutil.which("cargo"):
        print("The Rust tools aren't installed yet. Run: dev install rust")
        return 1
    root, added = sync_course(name)
    done = progress(name)
    lesson = next_lesson(root, done)
    files = [] if done else [root / "COURSE.md"]
    if lesson:
        files.append(lesson / "README.md")
        code_file = lesson / read_meta(lesson).get("open", "src/main.rs")
        if code_file.exists():
            files.append(code_file)
    if added and done:
        print(f"New lessons were added: {', '.join(added)}")
    if shutil.which("code"):
        subprocess.Popen(["code", str(root), *map(str, files)])
        print(f"The course is open in VS Code. Your lessons are in {root}")
    else:
        print(f"Your lessons are in {root}. Open them in a code editor to begin.")
    if lesson:
        print(f"Up next: {lesson.name} - {_title(lesson)}")
    else:
        print("You've finished every lesson that's available. New lessons arrive with updates.")
    return 0


def check_command(path, name="rust"):
    lesson = find_lesson(path)
    if not lesson:
        print("Open a lesson file first, or run this from inside a lesson folder.")
        return 2
    if not shutil.which("cargo"):
        print("The Rust tools aren't installed yet. Run: dev install rust")
        return 1
    print(f"Checking {lesson.name}: {_title(lesson)}...\n")
    result = check_lesson(lesson)
    print(("✓ " if result.passed else "✗ ") + result.summary)
    if result.details:
        print("\n" + result.details)
    if not result.passed:
        return 1
    mark_done(name, lesson.name)
    following = next_lesson(lesson.parent, progress(name))
    print("\nLesson complete!")
    if following:
        print(f"Next: {following.name} - {_title(following)}\n"
              f"Open {following.name}/README.md in VS Code, or run: dev learn")
    else:
        print("That's every lesson available right now. Great work!")
    return 0


def status_command(name="rust"):
    root = learn_dir(name)
    source = root if lessons(root) else course_source(name)
    done = progress(name)
    for lesson in lessons(source):
        mark = "✓" if lesson.name in done else " "
        print(f"[{mark}] {lesson.name}  {_title(lesson)}")
    total = len(lessons(source))
    print(f"\n{len(done & {l.name for l in lessons(source)})} of {total} lessons done.")
    return 0
