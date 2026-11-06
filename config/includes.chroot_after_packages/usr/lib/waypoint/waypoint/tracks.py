"""The things a beginner can choose to build, and what each one installs."""
from dataclasses import dataclass


@dataclass(frozen=True)
class Track:
    id: str
    title: str
    blurb: str
    packages: tuple
    commands: tuple
    project: str
    run: str
    recommended_ram_gb: int = 4


TRACKS = {
    t.id: t
    for t in (
        Track("web", "Websites", "Build pages and web apps with HTML, CSS and JavaScript.",
              ("nodejs", "npm"), ("node", "npm"), "my-website", "node server.js"),
        Track("python", "Python", "A friendly first language for scripts, data and automation.",
              ("python3", "python3-venv", "python3-pip"), ("python3",), "my-python-app", "python3 main.py"),
        Track("game", "Games", "Make a small 2D game you can play right away.",
              ("python3", "python3-pygame"), ("python3",), "my-game", "python3 main.py"),
        Track("mobile", "Phone-friendly apps", "Web apps people can add to a phone's home screen.",
              ("nodejs", "npm"), ("node", "npm"), "my-phone-app", "node server.js"),
        Track("rust", "Rust", "A fast, careful language. Compiling works best with 8 GB of RAM or more.",
              ("rustc", "cargo"), ("rustc", "cargo"), "my-rust-app", "cargo run", 8),
    )
}


def packages_for(track_ids):
    """Return the sorted packages for known tracks. Unknown IDs raise ValueError."""
    packages = set()
    for track_id in track_ids:
        if track_id not in TRACKS:
            raise ValueError(f"Unknown track: {track_id}")
        packages.update(TRACKS[track_id].packages)
    return sorted(packages)
