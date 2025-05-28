#!/usr/bin/env bash

set -xeo pipefail

readonly live_run=${LIVE_RUN:-false}
# Release number
readonly version=${VERSION:?input VERSION is required}
# Dependencies' pattern
readonly bump_deps_pattern=${BUMP_DEPS_PATTERN:-''}
# Dependencies' version
readonly bump_deps_version=${BUMP_DEPS_VERSION:-''}
# Dependencies' git branch
readonly bump_deps_branch=${BUMP_DEPS_BRANCH:-''}
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

# Converts cmake version into a debian package version
function to_debian_version() {
  v="${1}"
  deb_rev="${2:-1}"
  # check if version has tweak component
  if [ $(echo ${v//[^.]} | wc -c) == 4 ]; then
    if [ ${v:0-2} == ".0" ]; then
      deb_v=$(echo "${v}" | sed 's/\(.*\)\.0/\1~dev/')
    else
      deb_v=$(echo "${v}" | sed 's/\(.*\)\./\1~pre\./')
    fi
    echo "${deb_v}-${deb_rev}"
  else
    echo "${v}"
  fi
}

export GIT_AUTHOR_NAME=$git_user_name
export GIT_AUTHOR_EMAIL=$git_user_email
export GIT_COMMITTER_NAME=$git_user_name
export GIT_COMMITTER_EMAIL=$git_user_email

# Bump CMake project version
printf '%s' "$version" > version.txt
# Propagate version change to Cargo.toml and Cargo.toml.in
cmake . -DZENOHC_BUILD_IN_SOURCE_TREE=TRUE -DCMAKE_BUILD_TYPE=Release
# Update Debian dependency of libzenohc-dev
debian_version=$(to_debian_version $version)
toml_set_in_place Cargo.toml "package.metadata.deb.variants.libzenohc-dev.depends" "libzenohc (=$debian_version)"
toml_set_in_place Cargo.toml.in "package.metadata.deb.variants.libzenohc-dev.depends" "libzenohc (=$debian_version)"

git commit version.txt Cargo.toml Cargo.toml.in Cargo.lock -m "chore: Bump version to $version"

# Select all package dependencies that match $bump_deps_pattern and bump them to $bump_deps_version
if [[ "$bump_deps_pattern" != '' ]]; then
  for deps_key in "dependencies" "build-dependencies"; do
    deps=$(toml get Cargo.toml $deps_key | jq -r "keys[] | select(test(\"$bump_deps_pattern\"))")
    for dep in $deps; do
      if [[ -n $bump_deps_version ]]; then
        toml_set_in_place Cargo.toml "$deps_key.$dep.version" "$bump_deps_version"
        toml_set_in_place Cargo.toml.in "$deps_key.$dep.version" "$bump_deps_version"
      fi

      if [[ -n $bump_deps_branch ]]; then
        toml_set_in_place Cargo.toml "$deps_key.$dep.branch" "$bump_deps_branch"
        toml_set_in_place Cargo.toml.in "$deps_key.$dep.branch" "$bump_deps_branch"
      fi
    done
  done
  opaque_types_deps=$(toml get build-resources/opaque-types/Cargo.toml dependencies | jq -r "keys[] | select(test(\"$bump_deps_pattern\"))")
  for dep in $opaque_types_deps; do
    if [[ -n $bump_deps_version ]]; then
      toml_set_in_place build-resources/opaque-types/Cargo.toml "dependencies.$dep.version" "$bump_deps_version"
    fi

    if [[ -n $bump_deps_branch ]]; then
      toml_set_in_place build-resources/opaque-types/Cargo.toml "dependencies.$dep.branch" "$bump_deps_branch"
    fi
  done

  # Update lockfile using 1.75.0 to avoid updating it to version 4
  cargo +1.75.0 check
  cargo +1.75.0 check --manifest-path build-resources/opaque-types/Cargo.toml

  if [[ -n $bump_deps_version || -n $bump_deps_branch ]]; then
    git commit Cargo.toml Cargo.toml.in Cargo.lock build-resources/opaque-types/Cargo.toml build-resources/opaque-types/Cargo.lock -m "chore: Bump $bump_deps_pattern version to $bump_deps_version"
  else
    echo "warn: no changes have been made to any dependencies matching $bump_deps_pattern"
  fi
fi

if [[ ${live_run} ]]; then
  git tag --force "$version" -m "v$version"
fi
git log -10
git show-ref --tags
git push origin
git push --force origin "$version"