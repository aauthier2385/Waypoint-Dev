#!/bin/sh
# Builds a signed apt repository around one .deb.
# Usage: build-apt-repo.sh DEB_FILE GPG_KEY_ID PUBLIC_KEY_FILE OUTPUT_DIR
set -eu
deb="$1"
key_id="$2"
public_key="$3"
out="$4"

pool="$out/pool/main/w/waypoint-tools"
binary="$out/dists/stable/main/binary-amd64"
mkdir -p "$pool" "$binary"
cp "$deb" "$pool/"
install -m 0644 "$public_key" "$out/waypoint.asc"

cd "$out"
apt-ftparchive packages pool > "dists/stable/main/binary-amd64/Packages"
gzip -9nkf "dists/stable/main/binary-amd64/Packages"

apt-ftparchive \
  -o APT::FTPArchive::Release::Origin="Waypoint" \
  -o APT::FTPArchive::Release::Label="Waypoint" \
  -o APT::FTPArchive::Release::Suite="stable" \
  -o APT::FTPArchive::Release::Codename="stable" \
  -o APT::FTPArchive::Release::Architectures="amd64" \
  -o APT::FTPArchive::Release::Components="main" \
  release dists/stable > dists/stable/Release

gpg --batch --yes --default-key "$key_id" --clearsign -o dists/stable/InRelease dists/stable/Release
gpg --batch --yes --default-key "$key_id" --armor --detach-sign -o dists/stable/Release.gpg dists/stable/Release
