# Waypoint Dev

**A free Linux system for learning to code.** It's built for complete beginners and runs on newer and older PCs.

- **Everything set up from the first start:** a code editor (VS Code), Git, GitHub sign-in, and the tools for the kind of programming you pick.
- **A complete Rust course built in:** 33 step-by-step lessons, from your first program to a working web server. Every lesson explains things in plain English and checks your work for you.
- **Help when things break:** `dev doctor` finds common problems and tells you how to fix them, and you can undo big system changes.
- **Updates through the normal system updater**, including new lessons.

**[⬇ Download the latest version](https://github.com/aauthier2385/Waypoint-Dev/releases/latest)**. On that page, download the `.iso` file under **Assets**.

Waypoint Dev is based on [Debian 13](https://www.debian.org/), a long-running, widely used Linux system. Waypoint is an independent project and isn't made by Debian.

---

## What you need

- **A PC** with a 64-bit processor. Almost any PC from the last 15 years has one.
- **At least 4 GB of memory**, though 8 GB or more is better, and **40 GB of free disk space**.
- **A USB drive of 8 GB or larger.** Everything on it will be erased.
- **Internet**, for downloading and for setting up programming tools after you install.
- **About an hour.**

## Step 1: Download

1. Go to the [latest release](https://github.com/aauthier2385/Waypoint-Dev/releases/latest).
2. Under **Assets**, click the file ending in `.iso`. It's about 2 GB.

**Optional but recommended: check the download.** This confirms the file arrived complete and unchanged. The release page lists a long code called the **SHA-256**.

- **Windows:** open **PowerShell** and run `Get-FileHash "$HOME\Downloads\waypoint-dev-1.0.0-amd64.iso"`, using your file's actual name.
- **Mac:** open **Terminal** and run `shasum -a 256 ~/Downloads/waypoint-dev-1.0.0-amd64.iso`.
- **Linux:** run `sha256sum ~/Downloads/waypoint-dev-1.0.0-amd64.iso`.

The code it prints must exactly match the one on the release page. If it doesn't, download the file again.

## Step 2: Put it on a USB drive

You can't just copy the file onto the drive. It has to be *written* to it, using a free program.

**Using balenaEtcher (easiest, on Windows, Mac, or Linux):**

1. Download and install [balenaEtcher](https://etcher.balena.io/).
2. Click **Flash from file** and choose the `.iso`.
3. Click **Select target** and choose your USB drive. Double-check it's the right one, because it will be erased.
4. Click **Flash!** and wait for it to finish.

**Using Rufus (Windows only):** open [Rufus](https://rufus.ie/), pick your USB drive and the `.iso`, and click **Start**. If it asks how to write the image, choose **DD Image mode**.

## Step 3: Start your PC from the USB drive

1. Plug the USB drive into the PC where you want Waypoint.
2. Turn the PC on, and **right away** start tapping its boot menu key. It's different on different brands:

   | Brand | Boot menu key |
   | --- | --- |
   | Dell, Lenovo, Acer, Gigabyte | F12 |
   | HP | Esc, then F9 |
   | ASUS | F8 or Esc |
   | MSI, ASRock | F11 |
   | Apple (Intel Macs) | Hold Option |

   If none of these work, search for your PC's model plus "boot menu key."
3. In the menu, choose your USB drive. It's usually listed by its brand, like "SanDisk." Don't choose options with "Network" or "Boot Agent."
4. On the Waypoint start screen, press **Enter**.

After a minute, you'll be at the Waypoint desktop, running from the USB drive. Nothing on your PC has changed yet, so this is a good time to check that Wi-Fi, sound, and the screen work.

**If it won't start:**
- **"Secure Boot violation"** or a similar message: Waypoint supports Secure Boot on most PCs. On ones where it doesn't, enter your PC's setup screen (usually F2 or Delete as it starts), turn off **Secure Boot**, and try again.
- **It goes straight back to your usual system:** you probably missed the boot menu. Restart and tap the key sooner.

## Step 4: Install

> **Back up anything important before installing.** Installing changes your hard drive. Waypoint can share a drive with Windows or another system, but any mistake while changing a drive can lose data.

1. On the desktop, click **Install Waypoint** in the welcome window.
2. Choose your language, location, and keyboard.
3. On the **Partitions** screen, choose what to do with the drive:
   - **Erase disk:** replaces everything on that drive with Waypoint. Choose this for a spare PC.
   - **Install alongside:** keeps your current system, like Windows, and lets you choose which one to use each time the PC starts. Drag the divider to give Waypoint at least 40 GB.
   - **Manual partitioning:** for people who already know how drives are set up.
4. At the bottom of that screen, make sure **Boot loader location** names your hard drive, such as `/dev/sda` or `/dev/nvme0n1`.
5. Create your username and password. **Remember the password.** You'll need it to install things.
6. On the **Summary** screen, read what's about to happen. Click **Install** only if it looks right.
7. When it finishes, restart, and remove the USB drive when the screen goes dark.

## Step 5: First start

When you log in for the first time, **Waypoint Welcome** opens. Connect to the internet first, using the network icon in the top panel, then:

1. Click **Get started**.
2. **Pick what you want to build**: websites, Python, games, phone-friendly apps, or Rust. You can choose more than one, and add others later.
3. **Set up Git** with your name and email, so your work is saved with your name on it. You can also sign in to GitHub to back up and share your projects.
4. Click **Set up my PC** and enter your password. Waypoint installs the tools you chose.
5. You'll get a **starter project** for each choice, with a short lesson. Click **Open in VS Code** to begin.

To open the welcome app again later, find **Waypoint Welcome** in the applications menu.

## Step 6: Take the Rust course

If you chose Rust, open **Rust Course** from the applications menu, or type `dev learn` in a terminal. Each lesson opens in VS Code, with the guide on one side and the code on the other.

- **Read the lesson** and follow its steps.
- **Do the challenge** in the code.
- **Press Ctrl+Shift+B to check your work.** The checker explains what's right and what still needs fixing.

The first time, VS Code asks whether you trust the folder. Click **Yes, I trust the authors**, because the check button won't work otherwise.

`dev learn` always picks up where you left off, and `dev learn status` shows your progress.

## Handy commands

Open a terminal from the applications menu and type:

| Command | What it does |
| --- | --- |
| `dev doctor` | Checks for common problems and explains how to fix them |
| `dev learn` | Opens the Rust course where you left off |
| `dev new python` | Makes a new practice project. Also works with `web`, `game`, `mobile`, and `rust`. |
| `dev install rust` | Installs the tools for another kind of programming |
| `dev claude` | Installs the Claude desktop app, an AI assistant that can help explain code. Needs a Claude account. |
| `dev docker-setup` | Installs Docker, for projects that need it |
| `dev snapshot` | Saves a restore point you can go back to |

To undo a big system change, open **Undo a System Change** from the applications menu.

## Updates

Waypoint updates the same way as the rest of the system. Open a terminal and run:

```
sudo apt update && sudo apt upgrade
```

That brings fixes and new lessons for Waypoint, and security updates for everything else. Lessons you've already started are never changed.

## Common problems

- **Wi-Fi doesn't appear:** try a network cable. Some Wi-Fi cards need an extra driver, and `dev doctor` can help work out which. If your laptop has a Wi-Fi switch or key, make sure it's on.
- **The installer shows an error:** take a photo of it and [open an issue](https://github.com/aauthier2385/Waypoint-Dev/issues). Your old system hasn't been changed unless the error came after partitioning.
- **Your other system is missing from the start menu after installing:** open a terminal in Waypoint and run `sudo update-grub`, then restart.
- **Claude Code says "Illegal instruction":** your processor is too old for it, roughly from before 2013. Use Claude at [claude.ai](https://claude.ai) in Firefox instead.
- **Anything else:** run `dev doctor` first. If that doesn't help, [open an issue](https://github.com/aauthier2385/Waypoint-Dev/issues) and include what you were doing, what you expected, and what happened.

## About this project

Waypoint Dev is free and open source. It's an early release, tested on a small number of PCs so far, so reports of what does and doesn't work on yours are especially welcome.

To build it yourself or change it, see [docs/BUILDING.md](docs/BUILDING.md).
