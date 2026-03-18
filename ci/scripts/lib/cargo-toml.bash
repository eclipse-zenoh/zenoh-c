#!/usr/bin/env bash
#
# Shared functions for Cargo.toml and Cargo.toml.in manipulation
#
# Usage: source this file from other scripts
#   source "$(dirname "$0")/lib/cargo-toml.bash"
#

# Cross-platform sed -i (handles both GNU sed and BSD sed on macOS)
sed_in_place() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "$@"
    else
        sed -i "$@"
    fi
}

# Install toml-cli if not present
ensure_toml_cli() {
    if ! command -v toml &> /dev/null; then
        echo "Installing toml-cli..."
        cargo +stable install toml-cli
    fi
}

# toml-cli doesn't support in-place modification
# See: https://github.com/gnprice/toml-cli?tab=readme-ov-file#writing-ish-toml-set
toml_set_in_place() {
    local file="$1"
    local key="$2"
    local value="$3"
    local tmp
    tmp=$(mktemp)
    toml set "$file" "$key" "$value" > "$tmp"
    mv "$tmp" "$file"
}

# Update a dependency field in Cargo.toml.in using sed
# Cargo.toml.in contains CMake template variables (e.g., @CARGO_PROJECT_VERSION@)
# which make it invalid TOML, so we must use sed instead of toml-cli
#
# Args:
#   $1 - deps_key: dependencies section (dependencies|build-dependencies)
#   $2 - dep: dependency name
#   $3 - field: field to update (version|branch|git)
#   $4 - value: new value
#   $5 - file: (optional) file to update, defaults to Cargo.toml.in
cargo_toml_in_set_dep() {
    local deps_key="$1"
    local dep="$2"
    local field="$3"
    local value="$4"
    local file="${5:-Cargo.toml.in}"

    # Use # as delimiter to handle / in values (URLs, branch names like release/1.8.0)
    # Scope the substitution to the requested section: from [$deps_key] to the next section header.
    sed_in_place "/^\[$deps_key\]/,/^\[/ s#^\($dep = .*$field = \"\)[^\"]*\"#\1$value\"#" "$file"
}

# Update debian depends field in Cargo.toml.in
#
# Args:
#   $1 - debian_depends: the depends value (e.g., "libzenohc (=1.2.3)")
#   $2 - file: (optional) file to update, defaults to Cargo.toml.in
cargo_toml_in_set_debian_depends() {
    local debian_depends="$1"
    local file="${2:-Cargo.toml.in}"

    sed_in_place "s#^depends = \"libzenohc (=.*)\"#depends = \"$debian_depends\"#" "$file"
}

# Converts cmake version into a debian package version
#
# Args:
#   $1 - version: the version string
#   $2 - deb_rev: (optional) debian revision number, defaults to 1
#
# Examples:
#   1.2.3     -> 1.2.3
#   1.2.3.0   -> 1.2.3~dev-1
#   1.2.3.4   -> 1.2.3~pre.4-1
to_debian_version() {
    local v="$1"
    local deb_rev="${2:-1}"

    # check if version has tweak component (4 parts: X.Y.Z.W)
    if [ "$(echo "${v//[^.]}" | wc -c)" -eq 4 ]; then
        if [ "${v: -2}" == ".0" ]; then
            # X.Y.Z.0 -> X.Y.Z~dev-REV
            local deb_v
            deb_v=$(echo "${v}" | sed 's/\(.*\)\.0/\1~dev/')
            echo "${deb_v}-${deb_rev}"
        else
            # X.Y.Z.W -> X.Y.Z~pre.W-REV
            local deb_v
            deb_v=$(echo "${v}" | sed 's/\(.*\)\./\1~pre\./')
            echo "${deb_v}-${deb_rev}"
        fi
    else
        echo "${v}"
    fi
}
