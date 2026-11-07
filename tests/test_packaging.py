import os
import shutil
import subprocess
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent
needs_tools = pytest.mark.skipif(
    not all(shutil.which(tool) for tool in ("dpkg-deb", "gpg", "apt-ftparchive")),
    reason="packaging tools not installed",
)


@pytest.fixture
def signing(tmp_path):
    home = tmp_path / "gnupg"
    home.mkdir(mode=0o700)
    env = {**os.environ, "GNUPGHOME": str(home)}
    subprocess.run(["gpg", "--batch", "--passphrase", "", "--quick-generate-key", "Test Key", "ed25519", "sign", "never"],
                   env=env, check=True, capture_output=True)
    public = tmp_path / "key.asc"
    with public.open("w") as handle:
        subprocess.run(["gpg", "--batch", "--armor", "--export", "Test Key"], env=env, check=True, stdout=handle)
    return env, public


@needs_tools
def test_package_and_signed_repo(tmp_path, signing):
    env, public = signing
    out = tmp_path / "deb"
    subprocess.run([str(ROOT / "scripts/build-deb.sh"), "1.0.7", str(public), str(out)], check=True, capture_output=True)
    deb = out / "waypoint-tools_1.0.7_all.deb"
    contents = subprocess.run(["dpkg-deb", "--contents", str(deb)], capture_output=True, text=True, check=True).stdout

    assert "/usr/lib/waypoint/installer" not in contents
    assert "__pycache__" not in contents
    for path in ("./usr/bin/dev", "./usr/libexec/waypoint/waypoint-helper", "./etc/apt/sources.list.d/waypoint.list",
                 "./usr/share/keyrings/waypoint-archive-keyring.asc", "./usr/lib/waypoint/starters/web/server.js",
                 "./usr/lib/waypoint/courses/rust/08-guessing-game/src/main.rs", "./usr/lib/waypoint/courses/rust/FILES.md"):
        assert path in contents
    assert "-rwxr-xr-x root/root" in next(line for line in contents.splitlines() if line.endswith("waypoint-helper"))

    site = tmp_path / "site"
    subprocess.run([str(ROOT / "scripts/build-apt-repo.sh"), str(deb), "Test Key", str(public), str(site)],
                   env=env, check=True, capture_output=True)
    assert "waypoint-tools" in (site / "dists/stable/main/binary-amd64/Packages").read_text()
    subprocess.run(["gpg", "--batch", "--verify", str(site / "dists/stable/InRelease")], env=env, check=True, capture_output=True)
