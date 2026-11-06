#!/bin/sh
# Runs inside the build container. Works on a container volume because live-build
# can't run from a Windows-mounted folder.
set -eu
apt-get update
apt-get install -y live-build ca-certificates
cd /build
lb clean --purge 2>/dev/null || true
rm -rf auto config
cp -r /src/auto /src/config .
# Executable bits can be lost when a repo is checked out on Windows.
chmod +x auto/* config/hooks/normal/*
sh /src/scripts/check-packages.sh config/package-lists
lb config
lb build
cp ./*.iso /dist/waypoint-1-amd64.iso
sha256sum /dist/waypoint-1-amd64.iso
