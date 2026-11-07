#!/bin/sh
# Builds the waypoint-tools package from the same files the ISO uses.
# Usage: build-deb.sh VERSION PUBLIC_KEY_FILE OUTPUT_DIR
set -eu
version="$1"
key="$2"
out="$3"
root="$(cd "$(dirname "$0")/.." && pwd)"
stage="$(mktemp -d)"
trap 'rm -rf "$stage"' EXIT

cp -r "$root/config/includes.chroot_after_packages/." "$stage/"
# These only matter while the installer is running.
rm -rf "$stage/usr/lib/waypoint/installer"
find "$stage" -name __pycache__ -type d -prune -exec rm -rf {} +

install -d "$stage/usr/share/keyrings" "$stage/etc/apt/sources.list.d" "$stage/DEBIAN"
install -m 0644 "$key" "$stage/usr/share/keyrings/waypoint-archive-keyring.asc"
install -m 0644 "$root/packaging/waypoint.list" "$stage/etc/apt/sources.list.d/waypoint.list"

find "$stage" -type d -exec chmod 0755 {} +
find "$stage" -type f -exec chmod 0644 {} +
chmod 0755 "$stage/usr/bin/dev" "$stage/usr/bin/waypoint-welcome" "$stage/usr/libexec/waypoint/waypoint-helper"

sed "s/@VERSION@/$version/" "$root/packaging/control" > "$stage/DEBIAN/control"
printf '/etc/systemd/zram-generator.conf\n' > "$stage/DEBIAN/conffiles"

mkdir -p "$out"
dpkg-deb --root-owner-group --build "$stage" "$out/waypoint-tools_${version}_all.deb"
