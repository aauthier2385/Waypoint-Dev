"""Works out what PC we're on so setup can suggest sensible choices."""
import os
import re
import shutil
import subprocess
from dataclasses import dataclass

VENDORS = {"10de": "nvidia", "1002": "amd", "8086": "intel"}
GPU_LINE = re.compile(r"\[03[0-9a-f]{2}\]: (?P<name>.*?) \[(?P<vendor>[0-9a-f]{4}):[0-9a-f]{4}\]")


@dataclass(frozen=True)
class Hardware:
    ram_gb: float
    cores: int
    gpus: tuple

    @property
    def vendors(self):
        return {vendor for vendor, _ in self.gpus}

    @property
    def low_memory(self):
        return self.ram_gb < 6

    @property
    def summary(self):
        gpu = self.gpus[0][1] if self.gpus else "Unknown graphics"
        return f"{self.ram_gb:.0f} GB memory · {self.cores} CPU threads · {gpu}"


def parse_ram_gb(meminfo):
    match = re.search(r"^MemTotal:\s+(\d+) kB", meminfo, re.M)
    return round(int(match.group(1)) / 1024 / 1024, 1) if match else 0.0


def parse_gpus(lspci_output):
    gpus = []
    for line in lspci_output.splitlines():
        match = GPU_LINE.search(line)
        if match:
            vendor = VENDORS.get(match["vendor"], "other")
            gpus.append((vendor, match["name"]))
    return tuple(gpus)


def detect():
    try:
        with open("/proc/meminfo", encoding="ascii") as handle:
            ram = parse_ram_gb(handle.read())
    except OSError:
        ram = 0.0
    output = ""
    if shutil.which("lspci"):
        output = subprocess.run(["lspci", "-nn"], capture_output=True, text=True, check=False).stdout
    return Hardware(ram, os.cpu_count() or 1, parse_gpus(output))


def is_live_session():
    return os.path.isdir("/run/live/medium")


def secure_boot_enabled():
    if not shutil.which("mokutil"):
        return False
    result = subprocess.run(["mokutil", "--sb-state"], capture_output=True, text=True, check=False)
    return "enabled" in result.stdout.lower()
