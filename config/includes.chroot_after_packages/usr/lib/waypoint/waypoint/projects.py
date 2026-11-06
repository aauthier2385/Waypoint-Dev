"""Creates a beginner's first project from a starter."""
import json
import os
import re
import shutil
import subprocess
from pathlib import Path

from .tracks import TRACKS

STARTERS = Path(__file__).resolve().parent.parent / "starters"
VALID_NAME = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$")


def projects_dir():
    return Path.home() / "Projects"


def state_file():
    base = Path(os.environ.get("XDG_CONFIG_HOME", Path.home() / ".config"))
    return base / "waypoint" / "state.json"


def load_state():
    try:
        return json.loads(state_file().read_text(encoding="utf-8"))
    except (OSError, ValueError):
        return {}


def save_state(**changes):
    state = load_state()
    state.update(changes)
    path = state_file()
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(state, indent=2), encoding="utf-8")


def create_project(track_id, name=None, base=None, starters=None):
    """Copy a starter into a new folder. Never overwrites an existing folder."""
    track = TRACKS[track_id]
    name = name or track.project
    if not VALID_NAME.match(name):
        raise ValueError("Project names can use letters, numbers, dots, dashes and underscores.")
    base = Path(base) if base else projects_dir()
    base.mkdir(parents=True, exist_ok=True)
    target = base / name
    number = 2
    while target.exists():
        target = base / f"{name}-{number}"
        number += 1
    shutil.copytree(Path(starters) if starters else STARTERS / track_id, target)
    if shutil.which("git"):
        subprocess.run(["git", "init", "-q", "-b", "main", str(target)], check=False)
    return target
