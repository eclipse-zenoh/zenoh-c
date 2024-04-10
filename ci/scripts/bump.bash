#!/usr/bin/env bash

set -eo pipefail

# Project version
readonly VERSION
# Zenoh dependencies' version
readonly ZENOH_VERSION
# Zenoh dependencies' git branch
readonly ZENOH_BRANCH

cargo +stable install toml-cli

# NOTE(fuzzypixelz): toml-cli doesn't yet support in-place modification
# See: https://github.com/gnprice/toml-cli?tab=readme-ov-file#writing-ish-toml-set
function toml_set_in_place() {
  local tmp=$(mktemp)
  toml set "$1" "$2" "$3" > "$tmp"
  mv "$tmp" "$1"
}

# Bump CMake project version
sed -i "s;^set(ZENOHC_VERSION .*)$;set(ZENOHC_VERSION $VERSION);" CMakeLists.txt
# Propagate version change to Cargo.toml and Cargo.toml.in
cmake . -DZENOHC_BUILD_IN_SOURCE_TREE=TRUE -DCMAKE_BUILD_TYPE=Release
# Update Read the Docs version
sed -i "s;^release = .*$;release = '$VERSION';" docs/conf.py

git commit CMakeLists.txt Cargo.toml Cargo.lock -m "chore: Bump version to $VERSION"

# Select all package dependencies that match 'zenoh.*' and bump them to $ZENOH_VERSION
zenoh_deps=$(toml get Cargo.toml dependencies | jq -r 'keys.[] | select(test("zenoh.*"))')
for dep in $zenoh_deps; do
  [[ -n $ZENOH_VERSION ]] && toml_set_in_place Cargo.toml "dependencies.$dep.version" "$ZENOH_VERSION"
  [[ -n $ZENOH_BRANCH ]] && toml_set_in_place Cargo.toml "dependencies.$dep.branch" "$ZENOH_BRANCH"
done
# Update lockfile
cargo check

if [[ -n $ZENOH_VERSION || -n $ZENOH_BRANCH ]]; then
  git commit Cargo.toml Cargo.lock -m "chore: Bump Zenoh version to $ZENOH_VERSION"
else
  echo "info: no changes have been made to Zenoh dependencies"
fi
