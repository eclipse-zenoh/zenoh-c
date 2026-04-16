#!/usr/bin/env bash

set -xeo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=lib/cargo-toml.bash
source "$SCRIPT_DIR/lib/cargo-toml.bash"

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

ensure_toml_cli

export GIT_AUTHOR_NAME=$git_user_name
export GIT_AUTHOR_EMAIL=$git_user_email
export GIT_COMMITTER_NAME=$git_user_name
export GIT_COMMITTER_EMAIL=$git_user_email

# Bump CMake project version
printf '%s' "$version" > version.txt
# Propagate version change to Cargo.toml and Cargo.toml.in
cmake . -DZENOHC_BUILD_IN_SOURCE_TREE=TRUE -DCMAKE_BUILD_TYPE=Release
# Update Debian dependency of libzenohc-dev
debian_version=$(to_debian_version "$version")
toml_set_in_place Cargo.toml "package.metadata.deb.variants.libzenohc-dev.depends" "libzenohc (=$debian_version)"
cargo_toml_in_set_debian_depends "libzenohc (=$debian_version)"

# Show the changes to be committed
git diff version.txt Cargo.toml Cargo.toml.in Cargo.lock
git commit version.txt Cargo.toml Cargo.toml.in Cargo.lock -m "chore: Bump version to $version"

# Select all package dependencies that match $bump_deps_pattern and bump them to $bump_deps_version
if [[ "$bump_deps_pattern" != '' ]]; then
  for deps_key in "dependencies" "build-dependencies"; do
    deps=$(toml get Cargo.toml "$deps_key" | jq -r "keys[] | select(test(\"$bump_deps_pattern\"))")
    for dep in $deps; do
      if [[ -n $bump_deps_version ]]; then
        toml_set_in_place Cargo.toml "$deps_key.$dep.version" "$bump_deps_version"
        cargo_toml_in_set_dep "$deps_key" "$dep" "version" "$bump_deps_version"
      fi

      if [[ -n $bump_deps_branch ]]; then
        toml_set_in_place Cargo.toml "$deps_key.$dep.branch" "$bump_deps_branch"
        cargo_toml_in_set_dep "$deps_key" "$dep" "branch" "$bump_deps_branch"
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

  # Regenerate Cargo.lock for both Cargo.toml files
  cargo generate-lockfile
  cargo generate-lockfile --manifest-path build-resources/opaque-types/Cargo.toml

  if [[ -n $bump_deps_version || -n $bump_deps_branch ]]; then
    # Show the changes to be committed
    git diff Cargo.toml Cargo.toml.in Cargo.lock build-resources/opaque-types/Cargo.lock
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
