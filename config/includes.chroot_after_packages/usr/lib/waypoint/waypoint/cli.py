"""The `dev` command."""
import argparse
import os
import sys

from . import course, doctor
from .projects import create_project, load_state, save_state
from .tracks import TRACKS

HELPER = "/usr/libexec/waypoint/waypoint-helper"


def _helper(*args):
    os.execvp("pkexec", ["pkexec", HELPER, *args])


def main(argv=None):
    parser = argparse.ArgumentParser(prog="dev", description="Waypoint developer tools")
    commands = parser.add_subparsers(dest="command", required=True)
    commands.add_parser("doctor", help="check for common problems")
    commands.add_parser("tracks", help="list what you can build")
    new = commands.add_parser("new", help="create a project from a starter")
    new.add_argument("track", choices=sorted(TRACKS))
    new.add_argument("name", nargs="?")
    install = commands.add_parser("install", help="install the tools for a track")
    install.add_argument("tracks", nargs="+", choices=sorted(TRACKS))
    commands.add_parser("snapshot", help="take an undo snapshot now")
    learn = commands.add_parser("learn", help="open the built-in Rust course")
    learn_commands = learn.add_subparsers(dest="learn_command")
    check = learn_commands.add_parser("check", help="check the lesson you're working on")
    check.add_argument("path", nargs="?", default=".")
    learn_commands.add_parser("status", help="show which lessons you've finished")
    commands.add_parser("claude", help="install the Claude desktop app")
    commands.add_parser("docker-setup", help="let your account use Docker")
    args = parser.parse_args(argv)

    if args.command == "doctor":
        results = doctor.run_all()
        print(doctor.render(results))
        return 1 if any(result.status == doctor.FAIL for result in results) else 0
    if args.command == "learn":
        if args.learn_command == "check":
            return course.check_command(args.path)
        if args.learn_command == "status":
            return course.status_command()
        return course.open_course()
    if args.command == "tracks":
        for track in TRACKS.values():
            print(f"{track.id:8} {track.title}: {track.blurb}")
        return 0
    if args.command == "new":
        try:
            path = create_project(args.track, args.name)
        except ValueError as error:
            print(error, file=sys.stderr)
            return 2
        print(f"Created {path}\nOpen it with: code {path}\nThen run it with: {TRACKS[args.track].run}")
        return 0
    if args.command == "install":
        chosen = sorted(set(load_state().get("tracks", [])) | set(args.tracks))
        save_state(tracks=chosen)
        _helper("install-tracks", *args.tracks)
    if args.command == "snapshot":
        _helper("snapshot")
    if args.command == "claude":
        _helper("install-claude")
    if args.command == "docker-setup":
        _helper("enable-docker")
    return 0
