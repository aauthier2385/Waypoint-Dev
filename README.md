# Waypoint

A beginner-friendly Linux developer workstation based on Debian 13. "Waypoint" is a working name.

## What it includes

- XFCE desktop, with one ISO for older BIOS PCs and newer UEFI PCs
- Newer kernel and firmware from Debian Backports
- Graphical installer that works without internet
- Welcome app for choosing Websites, Python, Games, Phone-friendly apps, or Rust
- Git and GitHub setup, plus a starter project and lesson for each choice
- `dev doctor`, which explains common problems and their fixes
- Undo snapshots, compressed memory, VS Code, and Docker that starts only when needed

## Build the ISO

Option 1: On a PC with Docker, 30 GB free, and a good connection, run `./build.sh`. On Windows, run it from WSL with Docker Desktop. The ISO lands in `dist/`.

Option 2: Push the repo to GitHub, then open **Actions > Build ISO > Run workflow**. Download the ISO from the finished run.

## Test it

Try the ISO in a VM first, then on real hardware:

- [ ] Boots on an older BIOS PC
- [ ] Boots on a newer UEFI PC, with Secure Boot on and off
- [ ] Installer finishes without internet, then restarts into the installed system
- [ ] Wi-Fi, sound, and graphics work
- [ ] Welcome app asks for a password and installs a track
- [ ] Each starter project runs from **Run it** and **Ctrl+Shift+B**
- [ ] `dev doctor` matches what's actually wrong
- [ ] **Undo a system change** restores a snapshot
- [ ] NVIDIA driver installs on the GTX 970

## Change it

| To change | Edit |
| --- | --- |
| Preinstalled apps | `config/package-lists/` |
| Tracks | `.../usr/lib/waypoint/waypoint/tracks.py` |
| Starter projects | `.../usr/lib/waypoint/starters/` |
| Welcome app | `.../usr/lib/waypoint/waypoint/welcome.py` |

`...` is `config/includes.chroot_after_packages`.

## Run the checks

`pip install pytest pygame && pytest -q`

## Before releasing

- Pick a final name and check that it's available.
- Replace the remaining Debian artwork in the installer and boot menu.
- Review the licenses for VS Code and the non-free firmware, since the ISO redistributes them.
