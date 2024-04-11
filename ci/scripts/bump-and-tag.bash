#!/usr/bin/env bash

set -eo pipefail

# Repository
readonly repo=${REPO:?input REPO is required}
# Release number
readonly version=${VERSION:-''}
# Release branch
readonly branch=${BRANCH:?input BRANCH is required}
# Dependencies' pattern
readonly bump_deps_pattern=${BUMP_DEPS_PATTERN:-input BUMP_DEPS_PATTERN is required}
# Dependencies' version
readonly bump_deps_version=${BUMP_DEPS_VERSION:-''}
# Dependencies' git branch
readonly bump_deps_branch=${BUMP_DEPS_BRANCH:-''}
# GitHub token
readonly github_token=${GITHUB_TOKEN:?input GITHUB_TOKEN is required}
# Git actor name
readonly git_user_name=${GIT_USER_NAME:?input GIT_USER_NAME is required}
# Git actor email
readonly git_user_email=${GIT_USER_EMAIL:?input GIT_USER_EMAIL is required}

cargo +stable install toml-cli

# NOTE(fuzzypixelz): toml-cli doesn't yet support in-place modification
# See: https://github.com/gnprice/toml-cli?tab=readme-ov-file#writing-ish-toml-set
function toml_set_in_place() {
  local tmp=$(mktemp)
  toml set "$1" "$2" "$3" > "$tmp"
  mv "$tmp" "$1"
}

export GIT_AUTHOR_NAME=$git_user_name
export GIT_AUTHOR_EMAIL=$git_user_email
export GIT_COMMITTER_NAME=$git_user_name
export GIT_COMMITTER_EMAIL=$git_user_email

git clone --recursive --single-branch --branch "$branch" "https://github.com/$repo"

# Bump CMake project version
if [[ "$version" == '' ]]; then
  # If no version has been specified, infer it using git-describe
  printf '%s' "$(git describe)" > version.txt
else
  printf '%s' "$version" > version.txt
fi
# Propagate version change to Cargo.toml and Cargo.toml.in
cmake . -DZENOHC_BUILD_IN_SOURCE_TREE=TRUE -DCMAKE_BUILD_TYPE=Release
# Update Debian dependency of libzenohc-dev
toml_set_in_place Cargo.toml "package.metadata.deb.variants.libzenohc-dev.depends" "libzenohc (=$version)"
toml_set_in_place Cargo.toml.in "package.metadata.deb.variants.libzenohc-dev.depends" "libzenohc (=$version)"

git commit version.txt Cargo.toml Cargo.toml.in Cargo.lock -m "chore: Bump version to $version"

# Select all package dependencies that match $bump_deps_pattern and bump them to $bump_deps_version
deps=$(toml get Cargo.toml dependencies | jq -r "keys.[] | select(test(\"$bump_deps_pattern\"))")
for dep in $deps; do
  if [[ -n $bump_deps_version ]]; then
    toml_set_in_place Cargo.toml "dependencies.$dep.version" "$bump_deps_version"
    toml_set_in_place Cargo.toml.in "dependencies.$dep.version" "$bump_deps_version"
  fi

  if [[ -n $bump_deps_branch ]]; then
    toml_set_in_place Cargo.toml "dependencies.$dep.branch" "$bump_deps_branch"
    toml_set_in_place Cargo.toml.in "dependencies.$dep.branch" "$bump_deps_branch"
  fi
done
# Update lockfile
cargo check

if [[ -n $bump_deps_version || -n $bump_deps_branch ]]; then
  git commit Cargo.toml Cargo.toml.in Cargo.lock -m "chore: Bump $bump_deps_pattern version to $bump_deps_version"
else
  echo "info: no changes have been made to any dependencies"
fi

git tag "$version" -m "v$version"
git log -10
git show-ref --tags
git push "https://$github_token@github.com/$repo" "$branch" "$version"
