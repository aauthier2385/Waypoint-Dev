import shutil
from pathlib import Path

import pytest

from waypoint import course

ROOT = Path(__file__).resolve().parent.parent
SOURCE = ROOT / "config/includes.chroot_after_packages/usr/lib/waypoint/courses/rust"
SOLUTIONS = ROOT / "tests/course_solutions/rust"
LESSONS = [lesson.name for lesson in course.lessons(SOURCE)]
needs_cargo = pytest.mark.skipif(not shutil.which("cargo"), reason="cargo not installed")


def apply_solution(lesson, solution):
    remove = solution / "REMOVE"
    if remove.exists():
        for line in remove.read_text().split():
            (lesson / line).unlink()
    for path in solution.rglob("*"):
        if path.is_file() and path.name != "REMOVE":
            target = lesson / path.relative_to(solution)
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(path, target)


def test_every_lesson_is_complete():
    assert len(LESSONS) == 33
    for name in LESSONS:
        lesson = SOURCE / name
        meta = course.read_meta(lesson)
        assert meta["title"] and meta["hint"]
        assert meta["check"] in ("output", "tests")
        assert (lesson / "README.md").is_file()
        assert (lesson / "Cargo.toml").is_file()
        assert (lesson / meta["open"]).is_file()
        assert (SOLUTIONS / name).is_dir(), f"{name} needs a known-good solution"


@needs_cargo
@pytest.mark.parametrize("name", LESSONS)
def test_lesson_starts_unfinished_and_the_solution_passes(tmp_path, name):
    lesson = tmp_path / name
    shutil.copytree(SOURCE / name, lesson)
    first = course.check_lesson(lesson)
    assert not first.passed, f"{name} should need work before it passes"
    assert first.summary and (first.details or first.summary)
    apply_solution(lesson, SOLUTIONS / name)
    result = course.check_lesson(lesson)
    assert result.passed, f"{result.summary}\n{result.details}"


@needs_cargo
def test_first_failures_explain_themselves(tmp_path):
    messages = {}
    for name in ("02-inside-a-project", "03-variables", "04-types", "05-functions"):
        shutil.copytree(SOURCE / name, tmp_path / name)
        result = course.check_lesson(tmp_path / name)
        messages[name] = result.summary + "\n" + result.details
    assert "main.rs inside a folder called src" in messages["02-inside-a-project"]
    assert "add `mut`" in messages["03-variables"]
    assert "wrong type" in messages["04-types"]
    assert "todo!()" in messages["05-functions"]


@needs_cargo
def test_endless_loops_are_stopped(tmp_path):
    lesson = tmp_path / "07-loops"
    shutil.copytree(SOURCE / "07-loops", lesson)
    code = (lesson / "src/main.rs").read_text()
    code = code.replace("fn doublings_to_reach(start: u32, goal: u32) -> u32 {\n    todo!()",
                        "fn doublings_to_reach(start: u32, goal: u32) -> u32 {\n    loop {}")
    (lesson / "src/main.rs").write_text(code)
    course._cargo(["test", "--no-run"], lesson, timeout=300)
    original = course._check_tests

    def quick(lesson_dir, meta):
        real = course._cargo
        course._cargo = lambda args, folder, timeout: real(args, folder, timeout=5)
        try:
            return original(lesson_dir, meta)
        finally:
            course._cargo = real

    result = quick(lesson, course.read_meta(lesson))
    assert not result.passed
    assert "never finished" in result.summary


def test_sync_adds_new_lessons_without_touching_work(tmp_path):
    source = tmp_path / "source"
    shutil.copytree(SOURCE, source)
    target = tmp_path / "learn"
    _, added = course.sync_course(source=source, target=target)
    assert added == LESSONS
    work = target / "01-first-program/src/main.rs"
    work.write_text("my work")
    shutil.copytree(source / "01-first-program", source / "09-new-lesson")
    _, added = course.sync_course(source=source, target=target)
    assert added == ["09-new-lesson"]
    assert work.read_text() == "my work"
    assert (target / "FILES.md").is_file() and (target / ".vscode/tasks.json").is_file()


def test_find_lesson_from_inside_a_lesson(tmp_path):
    shutil.copytree(SOURCE / "05-functions", tmp_path / "05-functions")
    assert course.find_lesson(tmp_path / "05-functions/src/main.rs") == tmp_path / "05-functions"
    assert course.find_lesson(tmp_path / "05-functions/src") == tmp_path / "05-functions"
    assert course.find_lesson(tmp_path) is None


def test_progress_and_next_lesson(tmp_path, monkeypatch):
    monkeypatch.setenv("XDG_CONFIG_HOME", str(tmp_path))
    assert course.next_lesson(SOURCE, course.progress()).name == "01-first-program"
    course.mark_done("rust", "01-first-program")
    course.mark_done("rust", "01-first-program")
    assert course.progress() == {"01-first-program"}
    assert course.next_lesson(SOURCE, course.progress()).name == "02-inside-a-project"
