import json
import shutil
import subprocess
import xml.etree.ElementTree as ET
from pathlib import Path

import pytest

from waypoint.tracks import TRACKS

ROOT = Path(__file__).resolve().parent.parent
INCLUDES = ROOT / "config/includes.chroot_after_packages"
STARTERS = INCLUDES / "usr/lib/waypoint/starters"
SHELL_SCRIPTS = [
    ROOT / "build.sh", ROOT / "auto/config", ROOT / "auto/clean",
    ROOT / "scripts/build-inside.sh", ROOT / "scripts/check-packages.sh",
    ROOT / "scripts/build-deb.sh", ROOT / "scripts/build-apt-repo.sh",
    ROOT / "config/hooks/normal/0100-waypoint.hook.chroot",
    INCLUDES / "usr/lib/waypoint/installer/bootloader-config", INCLUDES / "usr/lib/waypoint/installer/sources-final",
]


@pytest.mark.parametrize("script", SHELL_SCRIPTS, ids=lambda p: p.name)
def test_shell_scripts_pass_shellcheck(script):
    if not shutil.which("shellcheck"):
        pytest.skip("shellcheck not installed")
    result = subprocess.run(["shellcheck", "-S", "warning", str(script)], capture_output=True, text=True)
    assert result.returncode == 0, result.stdout


@pytest.mark.parametrize("path", [
    ROOT / "build.sh", ROOT / "auto/config", ROOT / "config/hooks/normal/0100-waypoint.hook.chroot",
    INCLUDES / "usr/bin/dev", INCLUDES / "usr/bin/waypoint-welcome",
    INCLUDES / "usr/libexec/waypoint/waypoint-helper", INCLUDES / "usr/lib/waypoint/installer/bootloader-config",
    INCLUDES / "usr/lib/waypoint/installer/sources-final",
], ids=lambda p: p.name)
def test_entry_points_are_executable(path):
    assert path.stat().st_mode & 0o111


def test_desktop_files_are_valid():
    if not shutil.which("desktop-file-validate"):
        pytest.skip("desktop-file-validate not installed")
    files = list(INCLUDES.glob("usr/share/applications/*.desktop")) + list(INCLUDES.glob("etc/xdg/autostart/*.desktop"))
    assert files
    for path in files:
        result = subprocess.run(["desktop-file-validate", str(path)], capture_output=True, text=True)
        assert result.returncode == 0, result.stdout


def test_polkit_policy_points_at_the_helper():
    root = ET.parse(INCLUDES / "usr/share/polkit-1/actions/org.waypoint.helper.policy").getroot()
    annotate = root.find("./action/annotate[@key='org.freedesktop.policykit.exec.path']")
    assert annotate.text == "/usr/libexec/waypoint/waypoint-helper"
    assert (INCLUDES / annotate.text.lstrip("/")).exists()


@pytest.mark.parametrize("track_id", sorted(TRACKS))
def test_every_track_has_a_complete_starter(track_id):
    folder = STARTERS / track_id
    assert (folder / "LESSON.md").is_file()
    tasks = json.loads((folder / ".vscode/tasks.json").read_text())
    assert tasks["tasks"][0]["command"] == TRACKS[track_id].run
    assert tasks["tasks"][0]["group"]["isDefault"] is True
