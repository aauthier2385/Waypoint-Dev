#!/bin/sh
# Fails early, before the long build, if a package name doesn't exist on Debian 13.
set -eu
sed -i 's/^Components:.*/Components: main contrib non-free non-free-firmware/' /etc/apt/sources.list.d/debian.sources
apt-get update -qq
missing=0
for pkg in $(grep -hv '^#' "$1"/*.list.chroot | sed '/^$/d'); do
  if [ -z "$(apt-cache madison "$pkg" 2>/dev/null)" ]; then
    echo "Package not found: $pkg" >&2
    missing=1
  fi
done
exit "$missing"
