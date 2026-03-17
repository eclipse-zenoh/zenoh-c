#!/usr/bin/env bash
#
# Sync Cargo.toml.in with Cargo.toml
#
# This script reads dependency information from Cargo.toml and updates
# the corresponding values in Cargo.toml.in. This is necessary because
# Cargo.toml.in contains CMake template variables (e.g., @CARGO_PROJECT_VERSION@)
# which make it invalid TOML that cannot be parsed by toml-cli.
#
# The script syncs:
#   - zenoh dependency versions, branches, and git URLs
#   - debian package depends field
#
# Usage: ./ci/scripts/sync-cargo-toml-in.bash
#

set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=lib/cargo-toml.bash
source "$SCRIPT_DIR/lib/cargo-toml.bash"

# Ensure we're in the repository root
cd "$SCRIPT_DIR/../.."

# Check if Cargo.toml.in exists
if [[ ! -f "Cargo.toml.in" ]]; then
    echo "No Cargo.toml.in found, nothing to sync"
    exit 0
fi

ensure_toml_cli

echo "Syncing Cargo.toml.in with Cargo.toml..."

# Sync zenoh dependencies' version, branch, and git fields
for deps_key in "dependencies" "build-dependencies"; do
    # Get all zenoh-related dependencies from this section
    deps=$(toml get Cargo.toml "$deps_key" 2>/dev/null | jq -r 'keys[] | select(test("^zenoh"))' || true)

    for dep in $deps; do
        # Get values from Cargo.toml (strip quotes from JSON output)
        version=$(toml get Cargo.toml "$deps_key.$dep.version" 2>/dev/null | tr -d '"' || true)
        branch=$(toml get Cargo.toml "$deps_key.$dep.branch" 2>/dev/null | tr -d '"' || true)
        git_url=$(toml get Cargo.toml "$deps_key.$dep.git" 2>/dev/null | tr -d '"' || true)

        if [[ -n "$version" ]]; then
            echo "  [$deps_key] $dep: version = $version"
            cargo_toml_in_set_dep "$deps_key" "$dep" "version" "$version"
        fi

        if [[ -n "$branch" ]]; then
            echo "  [$deps_key] $dep: branch = $branch"
            cargo_toml_in_set_dep "$deps_key" "$dep" "branch" "$branch"
        fi

        if [[ -n "$git_url" ]]; then
            echo "  [$deps_key] $dep: git = $git_url"
            cargo_toml_in_set_dep "$deps_key" "$dep" "git" "$git_url"
        fi
    done
done

# Sync the debian package depends field
debian_depends=$(toml get Cargo.toml "package.metadata.deb.variants.libzenohc-dev.depends" 2>/dev/null | tr -d '"' || true)
if [[ -n "$debian_depends" ]]; then
    echo "  [package.metadata.deb] depends = $debian_depends"
    cargo_toml_in_set_debian_depends "$debian_depends"
fi

# Sync commented zenoh dependencies (like zenoh-pinned-deps-1-75)
# These are commented out in Cargo.toml but use @COMMENT_PINNED_DEPS@ placeholder in Cargo.toml.in
# We use the version/branch/git values from an active zenoh dependency as the reference
echo "Syncing commented zenoh dependencies..."

ref_version=$(toml get Cargo.toml "dependencies.zenoh.version" 2>/dev/null | tr -d '"' || true)
ref_branch=$(toml get Cargo.toml "dependencies.zenoh.branch" 2>/dev/null | tr -d '"' || true)
ref_git=$(toml get Cargo.toml "dependencies.zenoh.git" 2>/dev/null | tr -d '"' || true)

if [[ -n "$ref_version" ]]; then
    echo "  [commented] version = $ref_version"
    # Update version in lines starting with @COMMENT... followed by zenoh-
    sed_in_place 's#^\(@.*zenoh-[^=]*=.*version = "\)[^"]*"#\1'"$ref_version"'"#g' Cargo.toml.in
fi

if [[ -n "$ref_branch" ]]; then
    echo "  [commented] branch = $ref_branch"
    # Update branch in lines starting with @COMMENT... followed by zenoh-
    sed_in_place 's#^\(@.*zenoh-[^=]*=.*branch = "\)[^"]*"#\1'"$ref_branch"'"#g' Cargo.toml.in
fi

if [[ -n "$ref_git" ]]; then
    echo "  [commented] git = $ref_git"
    # Update git URL in lines starting with @COMMENT... followed by zenoh-
    sed_in_place 's#^\(@.*zenoh-[^=]*=.*git = "\)[^"]*"#\1'"$ref_git"'"#g' Cargo.toml.in
fi

echo "Done."
