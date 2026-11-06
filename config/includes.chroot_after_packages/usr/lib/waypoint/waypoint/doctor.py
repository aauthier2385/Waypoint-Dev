"""`dev doctor`: finds common problems and explains the fix in plain English."""
import os
import shutil
import socket
import subprocess
from dataclasses import dataclass

from . import hardware
from .projects import load_state
from .tracks import TRACKS

OK, WARN, FAIL = "ok", "warn", "fail"
SYMBOLS = {OK: "✓", WARN: "!", FAIL: "✗"}


@dataclass(frozen=True)
class Result:
    status: str
    name: str
    message: str
    fix: str = ""


def _run(args):
    try:
        result = subprocess.run(args, capture_output=True, text=True, timeout=20, check=False)
    except (OSError, subprocess.TimeoutExpired):
        return 127, ""
    return result.returncode, (result.stdout + result.stderr).strip()


def check_disk(path=None):
    free = shutil.disk_usage(path or os.path.expanduser("~")).free / 1024**3
    if free < 2:
        return Result(FAIL, "Disk space", f"Only {free:.1f} GB is free, so installs will likely fail.",
                      "Delete large files you don't need, then empty the Trash.")
    if free < 10:
        return Result(WARN, "Disk space", f"{free:.1f} GB free. That's getting tight for projects.",
                      "Free up some space when you get a chance.")
    return Result(OK, "Disk space", f"{free:.0f} GB free.")


def check_memory(ram_gb=None):
    ram = hardware.detect().ram_gb if ram_gb is None else ram_gb
    if ram < 6:
        return Result(WARN, "Memory", f"{ram:.0f} GB. Small projects are fine, but big builds may be slow.",
                      "Close your web browser while compiling.")
    return Result(OK, "Memory", f"{ram:.0f} GB.")


def check_internet(host="deb.debian.org", port=443):
    try:
        socket.create_connection((host, port), timeout=4).close()
        return Result(OK, "Internet", "Connected.")
    except OSError:
        return Result(FAIL, "Internet", "Can't reach the internet, so tools can't be installed.",
                      "Click the network icon in the top panel and connect to a network.")


def check_packages(run=_run):
    code, output = run(["dpkg", "--audit"])
    if code == 127:
        return Result(WARN, "System packages", "Couldn't check the package system.")
    if output:
        return Result(FAIL, "System packages", "An earlier install didn't finish.",
                      "Run: sudo dpkg --configure -a && sudo apt-get -f install")
    return Result(OK, "System packages", "Healthy.")


def check_git(run=_run):
    if not shutil.which("git"):
        return Result(FAIL, "Git", "Git isn't installed.", "Run: sudo apt-get install git")
    _, name = run(["git", "config", "--global", "user.name"])
    _, email = run(["git", "config", "--global", "user.email"])
    if not name or not email:
        return Result(WARN, "Git", "Git doesn't know your name and email yet, so it can't save your work.",
                      "Open Waypoint Welcome, or run: git config --global user.name \"Your Name\" "
                      "and git config --global user.email you@example.com")
    return Result(OK, "Git", f"Saving work as {name}.")


def check_github(run=_run):
    if not shutil.which("gh"):
        return Result(WARN, "GitHub", "The GitHub tool isn't installed.", "Run: sudo apt-get install gh")
    code, _ = run(["gh", "auth", "status"])
    if code != 0:
        return Result(WARN, "GitHub", "You aren't signed in, so you can't upload projects yet.",
                      "Run: gh auth login --web && gh auth setup-git")
    return Result(OK, "GitHub", "Signed in.")


def check_editor():
    if shutil.which("code"):
        return Result(OK, "VS Code", "Installed.")
    return Result(FAIL, "VS Code", "VS Code is missing.", "Run: sudo apt-get install code")


def check_tracks(track_ids=None):
    if track_ids is None:
        track_ids = load_state().get("tracks", [])
    results = []
    for track_id in track_ids:
        track = TRACKS.get(track_id)
        if not track:
            continue
        missing = [command for command in track.commands if not shutil.which(command)]
        if missing:
            results.append(Result(FAIL, track.title, f"Missing: {', '.join(missing)}.",
                                  f"Run: dev install {track.id}"))
        else:
            results.append(Result(OK, track.title, "Tools are ready."))
    return results


def check_docker(run=_run):
    if not shutil.which("docker"):
        return Result(WARN, "Docker", "Not installed. You won't need it until a project asks for it.")
    code, _ = run(["docker", "version", "--format", "{{.Server.Version}}"])
    if code != 0:
        return Result(WARN, "Docker", "Not set up for your account. You won't need it until a project asks for it.",
                      "Run: dev docker-setup, then log out and back in.")
    return Result(OK, "Docker", "Ready.")


def check_snapshots(config="/etc/timeshift/timeshift.json"):
    if os.path.exists(config):
        return Result(OK, "Undo snapshots", "Set up.")
    return Result(WARN, "Undo snapshots", "No snapshot has been taken yet, so there's nothing to undo to.",
                  "Run: dev snapshot")


def run_all():
    results = [check_disk(), check_memory(), check_internet(), check_packages(), check_git(),
               check_github(), check_editor()]
    results.extend(check_tracks())
    results.extend([check_docker(), check_snapshots()])
    return results


def render(results):
    lines = []
    for result in results:
        lines.append(f"{SYMBOLS[result.status]} {result.name}: {result.message}")
        if result.fix:
            lines.append(f"    Fix: {result.fix}")
    failures = sum(result.status == FAIL for result in results)
    warnings = sum(result.status == WARN for result in results)
    if failures:
        lines.append(f"\n{failures} problem(s) to fix and {warnings} suggestion(s).")
    elif warnings:
        lines.append(f"\nNothing is broken. {warnings} suggestion(s) above.")
    else:
        lines.append("\nEverything looks good.")
    return "\n".join(lines)
