"""The first-run welcome app."""
import argparse
import os
import shutil
import subprocess
import sys
import threading
from pathlib import Path

import gi

gi.require_version("Gtk", "3.0")
from gi.repository import GLib, Gtk, Gdk, Pango  # noqa: E402

from . import hardware  # noqa: E402
from .projects import create_project, load_state, save_state  # noqa: E402
from .tracks import TRACKS  # noqa: E402

HELPER = "/usr/libexec/waypoint/waypoint-helper"
PAGES = ("hello", "tracks", "git", "progress", "done")
CSS = b"""
.wp-root { background: #f6f4ef; color: #22302c; }
.wp-title { font-size: 26px; font-weight: 700; }
.wp-muted { color: #5b6863; }
.wp-card { background: #ffffff; border: 1px solid #ddd8cc; border-radius: 8px; padding: 12px 14px; }
.wp-note { background: #fbf1d9; border: 1px solid #e6d29d; border-radius: 8px; padding: 10px 14px; }
.wp-primary { background-image: none; background-color: #1f6f5c; color: #ffffff; border-color: #185846; }
.wp-primary:disabled { background-color: #a9bdb6; color: #f6f4ef; }
.wp-log { font-family: monospace; font-size: 11px; }
"""


def label(text, *classes, wrap=True):
    widget = Gtk.Label(label=text, xalign=0)
    widget.set_line_wrap(wrap)
    widget.set_line_wrap_mode(Pango.WrapMode.WORD_CHAR)
    for name in classes:
        widget.get_style_context().add_class(name)
    return widget


def open_terminal(directory, command):
    """Run a shell command in a terminal window that stays open afterward."""
    subprocess.Popen(["xfce4-terminal", "--hold", f"--working-directory={directory}",
                      "-x", "sh", "-c", command])


class Welcome(Gtk.Window):
    def __init__(self, hw, live, dry_run=False, page="hello", selected=None):
        super().__init__(title="Welcome to Waypoint")
        self.hw = hw
        self.live = live
        self.dry_run = dry_run
        self.projects = {}
        self.retry = lambda: None
        self.back_page = "tracks"
        self.track_checks = {}
        self.set_default_size(720, 560)
        self.set_position(Gtk.WindowPosition.CENTER)
        self.get_style_context().add_class("wp-root")
        self.connect("destroy", Gtk.main_quit)

        self.stack = Gtk.Stack(transition_type=Gtk.StackTransitionType.CROSSFADE, transition_duration=120)
        outer = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, margin=28, spacing=16)
        outer.pack_start(self.stack, True, True, 0)
        self.add(outer)

        self.stack.add_named(self.build_hello(), "hello")
        self.stack.add_named(self.build_tracks(), "tracks")
        self.stack.add_named(self.build_git(), "git")
        self.stack.add_named(self.build_progress(), "progress")
        self.stack.add_named(self.build_done(), "done")

        for track_id in selected or ():
            self.track_checks[track_id].set_active(True)
        self.show_all()
        self.stack.set_visible_child_name(page)

    def page(self, title, subtitle):
        box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=14)
        box.pack_start(label(title, "wp-title"), False, False, 0)
        box.pack_start(label(subtitle, "wp-muted"), False, False, 0)
        return box

    def footer(self, box, back=None, forward=None, forward_label="Continue"):
        row = Gtk.Box(spacing=8)
        if back:
            button = Gtk.Button(label="Back")
            button.connect("clicked", lambda _: self.stack.set_visible_child_name(back))
            row.pack_start(button, False, False, 0)
        next_button = None
        if forward:
            next_button = Gtk.Button(label=forward_label)
            next_button.get_style_context().add_class("wp-primary")
            next_button.connect("clicked", forward)
            row.pack_end(next_button, False, False, 0)
        box.pack_end(row, False, False, 0)
        return next_button

    # Pages -----------------------------------------------------------------
    def build_hello(self):
        box = self.page("Welcome to Waypoint", "A few quick questions and your PC will be ready for your first project.")
        box.pack_start(label(f"This PC: {self.hw.summary}", "wp-card"), False, False, 0)

        if self.live:
            box.pack_start(label("You're trying Waypoint from a USB drive. Install it before setting up "
                                 "tools, so your work is saved.", "wp-note"), False, False, 0)
            install = Gtk.Button(label="Install Waypoint")
            install.get_style_context().add_class("wp-primary")
            install.connect("clicked", lambda _: subprocess.Popen(["calamares-install-debian"]))
            install.set_halign(Gtk.Align.START)
            box.pack_start(install, False, False, 0)
            return box

        if self.hw.low_memory:
            box.pack_start(label("This PC has limited memory, so Waypoint turned on compressed memory. "
                                 "Websites and Python will run best.", "wp-note"), False, False, 0)
        if "nvidia" in self.hw.vendors:
            box.pack_start(label("NVIDIA graphics found. The built-in driver works for coding. Install "
                                 "NVIDIA's driver only if you plan to make 3D games or use AI tools.",
                                 "wp-note"), False, False, 0)
            driver = Gtk.Button(label="Install NVIDIA driver")
            driver.set_halign(Gtk.Align.START)
            driver.connect("clicked", lambda _: self.run_action(
                ["gpu-driver"], "Done. Restart your PC to start using the driver.",
                "The driver wasn't installed. Your PC will keep using the built-in one.", "hello"))
            box.pack_start(driver, False, False, 0)

        box.pack_start(label("Want help while you learn? Claude can explain errors and work through "
                             "code with you. You'll need a Claude account.", "wp-muted"), False, False, 0)
        box.pack_start(self.claude_button("hello"), False, False, 0)

        self.footer(box, forward=lambda _: self.stack.set_visible_child_name("tracks"), forward_label="Get started")
        return box

    def claude_button(self, back_page):
        button = Gtk.Button(label="Set up Claude")
        button.set_halign(Gtk.Align.START)
        button.connect("clicked", lambda _: self.run_action(
            ["install-claude"], "Done. Open Claude from the applications menu and sign in.",
            "Claude wasn't installed. Check your internet connection and try again.", back_page))
        return button

    def build_tracks(self):
        box = self.page("What do you want to build?", "Pick one or more. You can add others later.")
        for track in TRACKS.values():
            check = Gtk.CheckButton()
            text = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=2)
            text.pack_start(label(track.title), False, False, 0)
            blurb = track.blurb
            # A PC sold with 8 GB usually reports about 7.6 GB.
            if self.hw.ram_gb < track.recommended_ram_gb - 1:
                blurb += " This PC may be slow at it."
            text.pack_start(label(blurb, "wp-muted"), False, False, 0)
            check.add(text)
            check.get_style_context().add_class("wp-card")
            check.connect("toggled", self.on_track_toggled)
            self.track_checks[track.id] = check
            box.pack_start(check, False, False, 0)
        self.tracks_next = self.footer(box, back="hello", forward=lambda _: self.stack.set_visible_child_name("git"))
        self.tracks_next.set_sensitive(False)
        return box

    def on_track_toggled(self, _check):
        self.tracks_next.set_sensitive(bool(self.selected_tracks()))

    def selected_tracks(self):
        return [track_id for track_id, check in self.track_checks.items() if check.get_active()]

    def build_git(self):
        box = self.page("Save your work with Git", "Git keeps a history of your projects and labels your changes with this name. You can skip this for now.")
        grid = Gtk.Grid(row_spacing=8, column_spacing=12)
        self.git_name = Gtk.Entry(hexpand=True, text=self.git_config("user.name"))
        self.git_email = Gtk.Entry(hexpand=True, text=self.git_config("user.email"))
        grid.attach(label("Your name"), 0, 0, 1, 1)
        grid.attach(self.git_name, 1, 0, 1, 1)
        grid.attach(label("Your email"), 0, 1, 1, 1)
        grid.attach(self.git_email, 1, 1, 1, 1)
        box.pack_start(grid, False, False, 0)
        box.pack_start(label("Using a GitHub account? Sign in to back up and share your projects. "
                             "You'll need the internet and a web browser.", "wp-muted"), False, False, 0)
        github = Gtk.Button(label="Sign in to GitHub")
        github.set_halign(Gtk.Align.START)
        github.connect("clicked", self.on_github)
        box.pack_start(github, False, False, 0)
        self.footer(box, back="tracks", forward=self.on_install, forward_label="Set up my PC")
        return box

    def build_progress(self):
        box = self.page("Setting up your PC", "This can take a few minutes, depending on your internet connection.")
        self.bar = Gtk.ProgressBar()
        box.pack_start(self.bar, False, False, 0)
        self.log = Gtk.TextView(editable=False, cursor_visible=False, wrap_mode=Gtk.WrapMode.WORD_CHAR)
        self.log.get_style_context().add_class("wp-log")
        scroll = Gtk.ScrolledWindow(vexpand=True)
        scroll.add(self.log)
        box.pack_start(scroll, True, True, 0)
        self.retry_row = Gtk.Box(spacing=8, no_show_all=True)
        self.retry_button = Gtk.Button(label="Try again")
        self.retry_button.connect("clicked", lambda _: self.retry())
        self.retry_row.pack_start(self.retry_button, False, False, 0)
        back = Gtk.Button(label="Back")
        back.connect("clicked", lambda _: self.stack.set_visible_child_name(self.back_page))
        self.retry_row.pack_start(back, False, False, 0)
        back.show()
        box.pack_start(self.retry_row, False, False, 0)
        return box

    def build_done(self):
        box = self.page("You're ready", "Your starter projects are in the Projects folder. Each one includes a short lesson.")
        self.project_list = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=8)
        scroll = Gtk.ScrolledWindow(vexpand=True, hscrollbar_policy=Gtk.PolicyType.NEVER)
        scroll.add(self.project_list)
        box.pack_start(scroll, True, True, 0)
        tools = Gtk.Box(spacing=8)
        doctor = Gtk.Button(label="Check my PC")
        doctor.connect("clicked", lambda _: open_terminal(os.path.expanduser("~"), "dev doctor"))
        undo = Gtk.Button(label="Undo a system change")
        undo.connect("clicked", lambda _: subprocess.Popen(["timeshift-launcher"]))
        close = Gtk.Button(label="Finish")
        close.get_style_context().add_class("wp-primary")
        close.connect("clicked", lambda _: self.destroy())
        tools.pack_start(doctor, False, False, 0)
        tools.pack_start(self.claude_button("done"), False, False, 0)
        tools.pack_start(undo, False, False, 0)
        tools.pack_end(close, False, False, 0)
        box.pack_end(tools, False, False, 0)
        return box

    def add_project_row(self, track_id, path):
        track = TRACKS[track_id]
        row = Gtk.Box(spacing=8)
        row.get_style_context().add_class("wp-card")
        text = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=2)
        text.pack_start(label(track.title), False, False, 0)
        text.pack_start(label(str(path), "wp-muted"), False, False, 0)
        row.pack_start(text, True, True, 0)
        run = Gtk.Button(label="Run it")
        run.connect("clicked", lambda _: open_terminal(str(path), track.run))
        edit = Gtk.Button(label="Open in VS Code")
        edit.get_style_context().add_class("wp-primary")
        edit.connect("clicked", lambda _: subprocess.Popen(["code", str(path), str(path / "LESSON.md")]))
        row.pack_end(edit, False, False, 0)
        row.pack_end(run, False, False, 0)
        self.project_list.pack_start(row, False, False, 0)
        row.show_all()

    # Actions ---------------------------------------------------------------
    @staticmethod
    def git_config(key):
        if not shutil.which("git"):
            return ""
        result = subprocess.run(["git", "config", "--global", key], capture_output=True, text=True, check=False)
        return result.stdout.strip()

    def save_git_identity(self):
        for key, entry in (("user.name", self.git_name), ("user.email", self.git_email)):
            value = entry.get_text().strip()
            if value and not self.dry_run:
                subprocess.run(["git", "config", "--global", key, value], check=False)

    def on_github(self, _button):
        open_terminal(os.path.expanduser("~"), "gh auth login --web && gh auth setup-git")

    def append_log(self, text):
        buffer = self.log.get_buffer()
        buffer.insert(buffer.get_end_iter(), text)
        self.log.scroll_to_iter(buffer.get_end_iter(), 0, False, 0, 0)
        self.bar.pulse()
        return False

    def on_install(self, _button):
        self.save_git_identity()
        self.back_page = "tracks"
        self.retry = lambda: self.on_install(None)
        self.show_progress()
        tracks = self.selected_tracks()
        threading.Thread(target=self.install_worker, args=(tracks,), daemon=True).start()

    def run_action(self, helper_args, success, failure, back_page):
        self.back_page = back_page
        self.retry = lambda: self.run_action(helper_args, success, failure, back_page)
        self.show_progress()
        threading.Thread(target=self.action_worker, args=(helper_args, success, failure), daemon=True).start()

    def show_progress(self):
        self.retry_row.hide()
        self.stack.set_visible_child_name("progress")

    def show_result(self, succeeded):
        self.retry_button.set_visible(not succeeded)
        self.retry_row.show()
        return False

    def stream(self, args):
        if self.dry_run:
            GLib.idle_add(self.append_log, "Dry run: " + " ".join(args) + "\n")
            return 0
        try:
            process = subprocess.Popen(args, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        except OSError as error:
            GLib.idle_add(self.append_log, f"{error}\n")
            return 1
        for line in process.stdout:
            GLib.idle_add(self.append_log, line)
        return process.wait()

    def action_worker(self, helper_args, success, failure):
        code = self.stream(["pkexec", HELPER, *helper_args])
        GLib.idle_add(self.append_log, f"\n{success if code == 0 else failure}\n")
        GLib.idle_add(self.show_result, code == 0)

    def install_worker(self, tracks):
        code = self.stream(["pkexec", HELPER, "install-tracks", *tracks])
        if code != 0:
            GLib.idle_add(self.append_log, "\nSetup didn't finish. Check your internet connection and try again.\n")
            GLib.idle_add(self.show_result, False)
            return
        GLib.idle_add(self.finish_install, tracks)

    def finish_install(self, tracks):
        previous = load_state()
        known = dict(previous.get("projects", {}))
        for track_id in tracks:
            if track_id not in known or not os.path.isdir(known[track_id]):
                known[track_id] = str(create_project(track_id))
        for child in self.project_list.get_children():
            self.project_list.remove(child)
        for track_id in tracks:
            self.add_project_row(track_id, Path(known[track_id]))
        save_state(tracks=sorted(set(previous.get("tracks", [])) | set(tracks)), projects=known, finished=True)
        self.stack.set_visible_child_name("done")
        return False


def main(argv=None):
    parser = argparse.ArgumentParser(prog="waypoint-welcome")
    parser.add_argument("--first-run", action="store_true", help="only open if setup hasn't finished")
    parser.add_argument("--dry-run", action="store_true", help="don't change anything on this PC")
    parser.add_argument("--page", choices=PAGES, default="hello")
    parser.add_argument("--live", action="store_true", help="show the USB-drive version of the first page")
    parser.add_argument("--select", nargs="*", default=[], choices=sorted(TRACKS))
    args = parser.parse_args(argv)
    if args.first_run and load_state().get("finished"):
        return 0
    if not Gtk.init_check()[0]:
        print("No display found. Run this from the desktop.", file=sys.stderr)
        return 1
    provider = Gtk.CssProvider()
    provider.load_from_data(CSS)
    Gtk.StyleContext.add_provider_for_screen(Gdk.Screen.get_default(), provider,
                                             Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION)
    Welcome(hardware.detect(), args.live or hardware.is_live_session(), args.dry_run, args.page, args.select)
    Gtk.main()
    return 0
