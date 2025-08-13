# zenoh-ffi-workspace

A technical crate that provides the path to the workspace's Cargo.lock, which is necessary for correctly building the zenoh-ffi crate.

## Usage

This crate allows zenoh-ffi to determine the path to the workspace's `Cargo.lock` file. zenoh-ffi needs this path to correctly
determine the size and alignment of "opaque types": blind repr(C) structures equivalent to their corresponding Rust counterparts.
Unfortunately, Cargo doesn't allow dependent crates to know the path to the workspace where they're being built,
so additional configuration is necessary to pass this information to the zenoh-ffi crate.

### Solution 1: Environment Variable

Set the `CARGO_LOCK` environment variable to the absolute path of your `Cargo.lock`:

```bash
CARGO_LOCK=$PWD/Cargo.lock cargo build
```

zenoh-ffi-workspace simply passes this value to the zenoh-ffi crate.

### Solution 2: Workspace Integration

Add the crate to your workspace and substitute the original "zenoh-ffi-workspace" with a local version:

1. Clone the repository and make it part of your source tree. The "git submodule" approach can also be used,
   but for such a small project it probably isn't worth it

```bash
git clone https://github.com/eclipse-zenoh/zenoh-ffi-workspace
rm -rf zenoh-ffi-workspace/.git
```

2. Add to your `Cargo.toml`:

```toml
[workspace]
members = [
    "zenoh-ffi-workspace",
    # ... other members
]

[patch.crates-io]
"zenoh-ffi-workspace" = { path = "zenoh-ffi-workspace" }

[patch.'https://github.com/eclipse-zenoh/zenoh-ffi-workspace']
"zenoh-ffi-workspace" = { path = "zenoh-ffi-workspace" }
```

With this setup, the zenoh-ffi-workspace project can automatically pass the Cargo.lock
path to zenoh-ffi, and `cargo build` works without additional configuration.
