import importlib.machinery
import importlib.util
import json
import socket
import subprocess
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path

import pytest

from waypoint import doctor, hardware, projects
from waypoint.tracks import TRACKS, packages_for

ROOT = Path(__file__).resolve().parent.parent
INCLUDES = ROOT / "config/includes.chroot_after_packages"
STARTERS = INCLUDES / "usr/lib/waypoint/starters"


def load_helper():
    path = INCLUDES / "usr/libexec/waypoint/waypoint-helper"
    loader = importlib.machinery.SourceFileLoader("waypoint_helper", str(path))
    spec = importlib.util.spec_from_loader("waypoint_helper", loader)
    module = importlib.util.module_from_spec(spec)
    loader.exec_module(module)
    return module


def test_packages_combine_without_duplicates():
    assert packages_for(["web", "mobile"]) == ["nodejs", "npm"]


@pytest.mark.parametrize("bad", ["nope", "web; rm -rf /", "--allow-downgrades", ""])
def test_unknown_tracks_are_rejected(bad):
    with pytest.raises(ValueError):
        packages_for([bad])


def test_ram_and_gpu_parsing():
    assert hardware.parse_ram_gb("MemTotal:       16309860 kB\n") == 15.6
    output = (
        "00:02.0 VGA compatible controller [0300]: Intel Corporation UHD [8086:9bc5]\n"
        "01:00.0 VGA compatible controller [0300]: NVIDIA Corporation GM204 [GeForce GTX 970] [10de:13c2] (rev a1)\n"
        "02:00.0 Display controller [0380]: Advanced Micro Devices, Inc. [AMD/ATI] Navi 22 [1002:73df]\n"
        "03:00.0 Ethernet controller [0200]: Realtek RTL8111 [10ec:8168]\n"
    )
    assert [vendor for vendor, _ in hardware.parse_gpus(output)] == ["intel", "nvidia", "amd"]


def test_project_names_are_validated_and_never_overwrite(tmp_path):
    first = projects.create_project("python", base=tmp_path, starters=STARTERS / "python")
    (first / "main.py").write_text("mine")
    second = projects.create_project("python", base=tmp_path, starters=STARTERS / "python")
    assert second.name == "my-python-app-2"
    assert (first / "main.py").read_text() == "mine"
    for bad in ["../evil", "/etc", ".hidden", "a b"]:
        with pytest.raises(ValueError):
            projects.create_project("python", name=bad, base=tmp_path, starters=STARTERS / "python")


def test_doctor_gives_a_fix_for_every_problem():
    assert doctor.check_packages(lambda _: (0, "half-installed")).status == doctor.FAIL
    assert doctor.check_packages(lambda _: (0, "half-installed")).fix
    assert doctor.check_packages(lambda _: (0, "")).status == doctor.OK
    assert doctor.check_memory(4).status == doctor.WARN
    assert doctor.check_memory(16).status == doctor.OK
    assert doctor.check_tracks(["web", "not-a-track"])[0].name == "Websites"


def test_doctor_summary_wording():
    good = doctor.Result(doctor.OK, "A", "fine")
    warn = doctor.Result(doctor.WARN, "B", "hm", "do x")
    bad = doctor.Result(doctor.FAIL, "C", "no", "do y")
    assert "Everything looks good" in doctor.render([good])
    assert "Nothing is broken" in doctor.render([good, warn])
    assert "1 problem(s)" in doctor.render([bad, warn])


def test_nvidia_choice_accepts_only_driver_packages():
    helper = load_helper()
    text = "Detected NVIDIA GPUs:\n01:00.0 GTX 970\nIt is recommended to install the\n    nvidia-driver\npackage.\n"
    assert helper.choose_nvidia_package(text) == "nvidia-driver"
    assert helper.choose_nvidia_package("    nvidia-tesla-470-driver\n") == "nvidia-tesla-470-driver"
    assert helper.choose_nvidia_package("Uh oh. Your card is not supported.\n") is None
    assert helper.choose_nvidia_package("evil-package\n") is None


def free_port():
    with socket.socket() as sock:
        sock.bind(("127.0.0.1", 0))
        return sock.getsockname()[1]


@pytest.mark.parametrize("track", ["web", "mobile"])
def test_web_starters_serve_files_and_block_escapes(track):
    port = free_port()
    process = subprocess.Popen(["node", "server.js"], cwd=STARTERS / track,
                               env={"PORT": str(port), "PATH": "/usr/bin:/bin:/usr/local/bin"},
                               stdout=subprocess.DEVNULL)
    try:
        for _ in range(50):
            try:
                body = urllib.request.urlopen(f"http://127.0.0.1:{port}/").read()
                break
            except urllib.error.URLError:
                time.sleep(0.1)
        assert b"<!doctype html>" in body
        with socket.create_connection(("127.0.0.1", port)) as sock:
            sock.sendall(b"GET /../server.js HTTP/1.0\r\n\r\n")
            reply = sock.recv(200)
        assert b" 200 " not in reply.split(b"\r\n")[0]
    finally:
        process.kill()


def test_python_starter_plays_to_the_end():
    answers = "abc\n" + "".join(f"{n}\n" for n in range(1, 21))
    result = subprocess.run([sys.executable, "main.py"], cwd=STARTERS / "python", input=answers,
                            capture_output=True, text=True, timeout=10)
    assert result.returncode == 0
    assert "whole number" in result.stdout and "You got it" in result.stdout


def test_game_starter_runs_without_crashing():
    pytest.importorskip("pygame")
    process = subprocess.Popen([sys.executable, "main.py"], cwd=STARTERS / "game",
                               env={"SDL_VIDEODRIVER": "dummy", "SDL_AUDIODRIVER": "dummy", "PATH": "/usr/bin:/bin"})
    time.sleep(2)
    assert process.poll() is None
    process.kill()
