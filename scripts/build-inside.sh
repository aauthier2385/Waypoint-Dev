#!/bin/sh
# Runs inside the build container. Works on a container volume because live-build
# can't run from a Windows-mounted folder.
set -eu
apt-get update
apt-get install -y live-build ca-certificates curl gnupg
cd /build
lb clean --purge 2>/dev/null || true
rm -rf auto config
cp -r /src/auto /src/config .
# Executable bits can be lost when a repo is checked out on Windows.
chmod +x auto/* config/hooks/normal/*
sh /src/scripts/check-packages.sh config/package-lists
# Installed systems get updates through this package and the published signing key.
curl -fsSL https://aauthier2385.github.io/Waypoint-Dev/waypoint.asc -o /tmp/waypoint.asc
gpg --show-keys /tmp/waypoint.asc
mkdir -p config/packages.chroot
sh /src/scripts/build-deb.sh 1.0.0 /tmp/waypoint.asc config/packages.chroot
lb config
lb build
cp ./*.iso /dist/waypoint-1-amd64.iso
sha256sum /dist/waypoint-1-amd64.iso
