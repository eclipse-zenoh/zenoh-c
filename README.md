<img src="https://raw.githubusercontent.com/eclipse-zenoh/zenoh/main/zenoh-dragon.png" height="150">

[![CI](https://github.com/eclipse-zenoh/zenoh-c/workflows/CI/badge.svg)](https://github.com/eclipse-zenoh/zenoh-c/actions?query=workflow%3A%22CI%22)
[![Documentation Status](https://readthedocs.org/projects/zenoh-c/badge/?version=latest)](https://zenoh-c.readthedocs.io/en/latest/?badge=latest)
[![Discussion](https://img.shields.io/badge/discussion-on%20github-blue)](https://github.com/eclipse-zenoh/roadmap/discussions)
[![Discord](https://img.shields.io/badge/chat-on%20discord-blue)](https://discord.gg/2GJ958VuHs)
[![License](https://img.shields.io/badge/License-EPL%202.0-blue)](https://choosealicense.com/licenses/epl-2.0/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

# Eclipse Zenoh

The Eclipse Zenoh: Zero Overhead Pub/sub, Store/Query and Compute.

Zenoh (pronounce _/zeno/_) unifies data in motion, data at rest and computations. It carefully blends traditional pub/sub with geo-distributed storages, queries and computations, while retaining a level of time and space efficiency that is well beyond any of the mainstream stacks.

Check the website [zenoh.io](http://zenoh.io) and the [roadmap](https://github.com/eclipse-zenoh/roadmap) for more detailed information.

-------------------------------

# C API

This repository provides a C binding based on the main [Zenoh implementation written in Rust](https://github.com/eclipse-zenoh/zenoh).

-------------------------------

## How to build it

1. Make sure that [Rust](https://www.rust-lang.org) is available on your platform.
   Please check [here](https://www.rust-lang.org/tools/install) to learn how to install it.
   If you already have the Rust toolchain installed, make sure it is up-to-date with:

   ```bash
   rustup update
   ```

2. Clone the [source] with `git`:

   ```bash
   git clone https://github.com/eclipse-zenoh/zenoh-c.git
   ```

   [source]: https://github.com/eclipse-zenoh/zenoh-c

3. Build:

   Good CMake practice is to perform build outside of source directory, leaving source tree untouched. The examples below demonstrates this mode of building. On the other hand VScode by default creates build directory named 'build' inside source tree. In this case build script slightly changes its behavior. See more about it in section 'VScode'.

   By default build configuration is set to `Release`, it's not necessary to add `-DCMAKE_BUILD_TYPE=Release` option on configuration step. But if your platform uses multi-config generator by default (this is the case on Windows), you may need to add option `--config Release` on build step. See more in CMake [build-configurations] documentation. Option`--config Release` is skipped in further examples for brewity. It's actually necessary for [Visual Studio generators] only. For [Ninja Multi-Config] the build script is able to select `Release` as the default configuration.

   ```bash
   mkdir -p build && cd build 
   cmake ../zenoh-c
   cmake --build . --config Release
   ```

   The generator to use is selected with option `-G`. If Ninja is installed on your system, adding `-GNinja` to `cmake` command can greatly speed up the build time:

   ```bash
   cmake ../zenoh-c -GNinja
   cmake --build .
   ```

   Unstable api and/or shared memory support can be enabled by setting repectively `ZENOHC_BUILD_WITH_UNSTABLE_API` and `ZENOHC_BUILD_WITH_SHARED_MEMORY` Cmake flags to `true` during configuration step.

   ```bash
   cmake -DZENOHC_BUILD_WITH_UNSTABLE_API=true -DZENOHC_BUILD_WITH_SHARED_MEMORY=true ../zenoh-c
   cmake --build . --config Release
   ```

   [build-configurations]: https://cmake.org/cmake/help/latest/manual/cmake-buildsystem.7.html#build-configurations
   [Visual Studio generators]: https://cmake.org/cmake/help/latest/manual/cmake-generators.7.html#id14
   [Ninja Multi-Config]: https://cmake.org/cmake/help/latest/generator/Ninja%20Multi-Config.html

4. Install:

   To install zenoh-c library into system just build target `install`. You need root privileges to do it, as the default install location is `/usr/local`.

   ```bash
   cmake --build . --target install
   ```  

   If you want to install zenoh-c libraries locally, you can set the installation directory with `CMAKE_INSTALL_PREFIX`

   ```bash
   cmake ../zenoh-c -DCMAKE_INSTALL_PREFIX=~/.local
   cmake --build . --target install
   ```

   By default only dynamic library is built and installed. Set `BUILD_SHARED_LIBS` variable to false to build and install static library:

   ```bash
   cmake ../zenoh-c -DCMAKE_INSTALL_PREFIX=~/.local -DBUILD_SHARED_LIBS=FALSE
   cmake --build . --target install
   ```

   The result of installation is the header files in `include` directory, the library files in `lib` directory and cmake package configuration files for package `zenohc` in `lib/cmake` directory. The library later can be loaded with CMake command `find_package(zenohc)`.
   Add dependency in CMakeLists.txt on target

   - `zenohc::shared` for linking dynamic library
   - `zenohc::static` for linking static library
   - `zenohc::lib` for linking static or dynamic library depending on boolean variable `BUILD_SHARED_LIBS`

5. VScode

   When zenoh-c project is opened in VSCode the build directory is set to `build` inside source tree (this is default behavior of Microsoft [CMake Tools]). The project build script detects this situation. In this case it places build files in `target` directory and `Cargo.toml` file (which is generated from `Cargo.toml.in`) into the root of source tree, as the rust developers used to and as the rust build tools expects by default. This behavior also can be explicitly enabled by setting `ZENOHC_BUILD_IN_SOURCE_TREE` variable to `TRUE`.

   [CMake Tools]: https://marketplace.visualstudio.com/items?itemName=ms-vscode.cmake-tools

## Building the Examples

The examples can be built in two ways. One is to select `examples` as a build target of zenoh-c project (assuming here that the current directory is side-by-side with zenoh-c directory):

```bash
cmake ../zenoh-c
cmake --build . --target examples
```

You may also use `--target <example_name>` if you wish to only build a specific example.

All build artifacts will be in the `target/release/examples` directory in this case.

The second way is to directly build `examples` as a root project:

```bash
cmake ../zenoh-c/examples
cmake --build .
```

Link with `zenoh-c` installed into default location in the system (with [find_package]):

```bash
cmake ../zenoh-c/examples
```

Link with `zenoh-c` installed in `~/.local` directory:

```bash
cmake ../zenoh-c/examples -DCMAKE_INSTALL_PREFIX=~/.local
```

## Running the Examples

See information about running examples [here](./examples/README.md).

## Documentation

Zenoh-c API documentation is available on [Read the Docs](https://zenoh-c.readthedocs.io/en/latest/index.html).

It can be built manually by performing the following steps:

```bash
cd docs
doxygen
sphinx-build -b html . _build/html
```

## Cross-Compilation

Cross-compilation can be performed using standard cmake approach as described in [[cmake-toolchains](https://cmake.org/cmake/help/latest/manual/cmake-toolchains.7.html)].

In addition the following project-specific options might need to be set for cross-compilation:

- `-DZENOHC_CARGO_CHANNEL="+nightly"|"+beta"|"+stable"`: refers to a specific rust toolchain release [[rust-channels](https://rust-lang.github.io/rustup/concepts/channels.html)]
- `-DZENOHC_CARGO_FLAGS`: several optional flags can be used for compilation. [[cargo flags](https://doc.rust-lang.org/cargo/commands/cargo-build.html)]
- `-DZENOHC_CUSTOM_TARGET`: specifies a crosscompilation target. Currently rust support several Tier-1, Tier-2 and Tier-3 targets [[targets](https://doc.rust-lang.org/nightly/rustc/platform-support.html)].

Let's put all together in an example:
Assuming you want to cross-compile for x86_64-pc-windows-gnu from Ubuntu environment.

1. Install required packages
   - `sudo apt-get install -y mingw-w64`: cross-compilation toolchain for c/c++.
   - `rustup toolchain install x86_64-pc-windows-gnu`: cross-compilation toolchain for rust.
2. *(Only if you're using `nightly`)
   - `rustup component add rust-src --toolchain nightly`
3. Compile Zenoh-C. Assume that it's in `zenoh-c` directory. Notice that build in this sample is performed outside of source directory

   ```bash
   export RUSTFLAGS="-Clinker=x86_64-w64-mingw32-gcc -Car=x86_64-w64-mingw32-ar"
   mkdir -p build && cd build
   cmake ../zenoh-c  -DCMAKE_SYSTEM_NAME="Windows" -DCMAKE_C_COMPILER="x86_64-w64-mingw32-gcc" -DCMAKE_CXX_COMPILER="x86_64-w64-mingw32-g++" -DCMAKE_SYSTEM_PROCESSOR="x86_64" -DZENOHC_CARGO_CHANNEL="+nightly" -DZENOHC_CARGO_FLAGS="-Zbuild-std=std,panic_abort" -DZENOHC_CUSTOM_TARGET="x86_64-pc-windows-gnu" -DCMAKE_INSTALL_PREFIX="../x86_64-pc-windows-gnu/stage"
   cmake --build . --target install
   ```

If all goes right the building files will be located at:
`/path/to/zenoh-c/target/x86_64-pc-windows-gnu/release`
and release files will be located at
`/path/to/zenoh-c/target/x86_64-pc-windows-gnu/release`

> :warning: **WARNING** :warning: : Perhaps additional efforts are necessary, that will depend of your environment.

## Rust Version

The minimal supported Rust version (MSRV) is 1.75, as specified in [Cargo.toml](Cargo.toml).
Builds and tests are run using the version defined in [rust-toolchain.toml](rust-toolchain.toml).
The rust version can be specified with CMake variable `ZENOHC_CARGO_CHANNEL`:

```bash
cmake ../zenoh-c -DZENOHC_CARGO_CHANNEL="+1.75.0"
```

or

```bash
cmake ../zenoh-c -DZENOHC_CARGO_CHANNEL="+nightly"
```

Special efforts are made to keep Rust 1.75 compatibility. The base `zenoh` project provides crate [zenoh-pinned-deps-1-75](https://crates.io/crates/zenoh-pinned-deps-1-75) which
pins crate dependencies to latest versions compatible with rust 1.75. This crate is separate from the [zenoh](https://crates.io/crates/zenoh) itself to avoid staying on obsolete versions and crate version conflicts.
If some project needs compatibility with Rust 1.75, it adds a dependency on `zenoh-pinned-deps-1-75` and if necessary adds some additional version pins (like `crate_name="=X.Y.Z"`). 
On the `zenoh-c` level there is a special check in `CMakeLists.txt` which uncomments pinning in the generated `Cargo.toml` when `ZENOHC_CARGO_CHANNEL` == `+1.75.0`.

## Zenoh features support (enabling/disabling protocols, etc)

It's necessary sometimes to build zenoh-c library with set of features different from default. For example: enable TCP and UDP only. This can be done by changing `ZENOHC_CARGO_FLAGS` parameter for cmake (notice ";" instead of space due to cmake peculiarities)

Available features can be found in [Cargo.toml](./Cargo.toml)

```bash
cmake ../zenoh-c -DZENOHC_CARGO_FLAGS="--no-default-features;--features=transport_tcp,transport_udp"
```

## Versioning

Being a CMake project, zenoh-c is limited to the `MAJOR.MINOR.PATCH.TWEAK` version scheme [inherent
to CMake](https://gitlab.kitware.com/cmake/cmake/-/issues/16716). However, zenoh-c also incorporates
a Cargo package which cannot be versionned with the `MAJOR.MINOR.PATCH.TWEAK` version scheme (not
SemVer compatible). Hence zenoh-c uses a one-to-one mapping between CMake versions and SemVer versions:

| CMake version           | SemVer equivalent | Meaning              |
|-------------------------|-------------------|----------------------|
| `1.2.3`                 | `1.2.3`           | Release version      |
| `1.2.3.0`               | `1.2.3-dev`       | Developement version |
| `1.2.3.x if x >= 1`     | `1.2.3-pre.x`     | Pre-release version  |
