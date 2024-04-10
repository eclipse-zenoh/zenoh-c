#!/usr/bin/env bash

set -eo pipefail

# Repository
readonly REPO
# Release number
readonly VERSION
# Release branch
readonly BRANCH
# Dependencies' pattern
readonly BUMP_DEPS_PATTERN
# Dependencies' version
readonly BUMP_DEPS_VERSION
# Dependencies' git branch
readonly BUMP_DEPS_BRANCH
# GitHub token
readonly GH_TOKEN
# Git actor name
readonly GIT_USER_NAME
# Git actor email
readonly GIT_USER_EMAIL

cargo +stable install toml-cli

# NOTE(fuzzypixelz): toml-cli doesn't yet support in-place modification
# See: https://github.com/gnprice/toml-cli?tab=readme-ov-file#writing-ish-toml-set
function toml_set_in_place() {
  local tmp=$(mktemp)
  toml set "$1" "$2" "$3" > "$tmp"
  mv "$tmp" "$1"
}

export GIT_AUTHOR_NAME=$GIT_USER_NAME
export GIT_AUTHOR_EMAIL=$GIT_USER_EMAIL
export GIT_COMMITTER_NAME=$GIT_USER_NAME
export GIT_COMMITTER_EMAIL=$GIT_USER_EMAIL

git clone --recursive --single-branch --branch "$BRANCH" "https://github.com/$REPO"

# Bump CMake project version
sed -i "s;^set(ZENOHC_VERSION .*)$;set(ZENOHC_VERSION $VERSION);" CMakeLists.txt
# Propagate version change to Cargo.toml and Cargo.toml.in
cmake . -DZENOHC_BUILD_IN_SOURCE_TREE=TRUE -DCMAKE_BUILD_TYPE=Release
# Update Read the Docs version
sed -i "s;^release = .*$;release = '$VERSION';" docs/conf.py
# Update Debian dependency of libzenohc-dev
toml_set_in_place Cargo.toml "package.metadata.deb.variants.libzenohc-dev.depends" "libzenohc (=$VERSION)"
toml_set_in_place Cargo.toml.in "package.metadata.deb.variants.libzenohc-dev.depends" "libzenohc (=$VERSION)"

git commit CMakeLists.txt Cargo.toml Cargo.toml.in Cargo.lock -m "chore: Bump version to $VERSION"

# Select all package dependencies that match $BUMP_DEPS_PATTERN and bump them to $BUMP_DEPS_VERSION
deps=$(toml get Cargo.toml dependencies | jq -r "keys.[] | select(test(\"$BUMP_DEPS_PATTERN\"))")
for dep in $deps; do
  if [[ -n $BUMP_DEPS_VERSION ]]; then
    toml_set_in_place Cargo.toml "dependencies.$dep.version" "$BUMP_DEPS_VERSION"
    toml_set_in_place Cargo.toml.in "dependencies.$dep.version" "$BUMP_DEPS_VERSION"
  fi

  if [[ -n $BUMP_DEPS_BRANCH ]]; then
    toml_set_in_place Cargo.toml "dependencies.$dep.branch" "$BUMP_DEPS_BRANCH"
    toml_set_in_place Cargo.toml.in "dependencies.$dep.branch" "$BUMP_DEPS_BRANCH"
  fi
done
# Update lockfile
cargo check

if [[ -n $BUMP_DEPS_VERSION || -n $BUMP_DEPS_BRANCH ]]; then
  git commit Cargo.toml Cargo.toml.in Cargo.lock -m "chore: Bump $BUMP_DEPS_PATTERN
 version to $BUMP_DEPS_VERSION"
else
  echo "info: no changes have been made to any dependencies"
fi

git tag "$VERSION" -m "v$VERSION"
git log -10
git show-ref --tags
git push "https://$GH_TOKEN@github.com/$REPO" "$BRANCH" "$VERSION"
