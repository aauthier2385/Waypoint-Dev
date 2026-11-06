#!/bin/sh
# Builds the ISO inside a Debian container. Needs Docker, about 30 GB free and
# roughly an hour. The finished ISO lands in ./dist.
set -eu
cd "$(dirname "$0")"
mkdir -p dist
docker volume create waypoint-build >/dev/null
docker run --rm --privileged \
  -v "$PWD":/src:ro \
  -v "$PWD/dist":/dist \
  -v waypoint-build:/build \
  debian:trixie sh /src/scripts/build-inside.sh
echo "Done. ISO is in $PWD/dist"
