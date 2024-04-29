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

> :warning: **WARNING** :warning: : Zenoh and its ecosystem are under active development. When you build from git, make sure you also build from git any other Zenoh repository you plan to use (e.g. binding, plugin, backend, etc.). It may happen that some changes in git are not compatible with the most recent packaged Zenoh release (e.g. deb, docker, pip). We put particular effort in maintaining compatibility between the various git repositories in the Zenoh project.

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

   By default only dynamic library is installed. Set `ZENOHC_INSTALL_STATIC_LIBRARY` variable to true to install static library also:

   ```bash
   cmake ../zenoh-c -DCMAKE_INSTALL_PREFIX=~/.local -DZENOHC_INSTALL_STATIC_LIBRARY=TRUE
   cmake --build . --target install
   ```

   The result of installation is the header files in `include` directory, the library files in `lib` directory and cmake package configuration files for package `zenohc` in `lib/cmake` directory. The library later can be loaded with CMake command `find_package(zenohc)`.
   Add dependency in CMakeLists.txt on target

   - `zenohc::shared` for linking dynamic library
   - `zenohc::static` for linking static library
   - `zenohc::lib` for linking static or dynamic library depending on boolean variable `ZENOHC_LIB_STATIC`

   For `Debug` configuration the library package `zenohc_debug` is installed side-by-side with release `zenohc` library. Suffix `d` is added to names of library files (libzenohc**d**.so).

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

In this case, the examples executables will be built in the current directory.

As a root project the `examples` project links `zenoh-c` with CMake's [add_subdirectory] command by default. There are also other ways to link `zenoh-c` - with [find_package] or [FetchContent]:

[add_subdirectory]: https://cmake.org/cmake/help/latest/command/add_subdirectory.html
[find_package]: https://cmake.org/cmake/help/latest/command/find_package.html
[FetchContent]: https://cmake.org/cmake/help/latest/module/FetchContent.html

Link with `zenoh-c` installed into default location in the system (with [find_package]):

```bash
cmake ../zenoh-c/examples -DZENOHC_SOURCE=PACKAGE
```

Link with `zenoh-c` installed in `~/.local` directory:

```bash
cmake ../zenoh-c/examples -DZENOHC_SOURCE=PACKAGE -DCMAKE_INSTALL_PREFIX=~/.local
```

Download specific `zenoh-c` version from git with [FetchContent]:

```bash
cmake ../zenoh-c/examples -DZENOHC_SOURCE=GIT_URL -DZENOHC_GIT_TAG=0.11.0-rc
```

See also `configure_include_project` function in [helpers.cmake] for more information

[helpers.cmake]: cmake/helpers.cmake

## Running the Examples

### Basic Pub/Sub Example

```bash
./target/release/examples/z_sub
```

```bash
./target/release/examples/z_pub
```

### Queryable and Query Example

```bash
./target/release/examples/z_queryable
```

```bash
./target/release/examples/z_get
```

### Running the Throughput Examples

```bash
./target/release/examples/z_sub_thr
```

```bash
./target/release/examples/z_pub_thr
```

## API conventions

Most of the types exposed by the `zenoh-c` API are types for which destruction is necessary. To help you spot these types, we name them with the convention that any destructible type must start by `z_owned`.

For maximum performance, we try to make as few copies as possible. Sometimes, this implies moving data that you `z_owned`. Any function that takes a non-const pointer to a `z_owned` type will perform its destruction. To make this pattern more obvious, we encourage you to use the `z_move` macro instead of a simple `&` to create these pointers. Rest assured that all `z_owned` types are double-free safe, and that you may check whether any `z_owned_X_t` typed value is still valid by using `z_X_check(&val)`, or the `z_check(val)` macro if you're using C11.

We hope this convention will help you streamline your memory-safe usage of zenoh, as following it should make looking for leaks trivial: simply search for paths where a value of a `z_owned` type hasn't been passed to a function using `z_move`.

Functions that simply need to borrow your data will instead take values of the associated `z_X_t` type. You may construct them using `z_X_loan(&val)` (or the `z_loan(val)` generic macro with C11).

Note that some `z_X_t` typed values can be constructed without needing to `z_borrow` their owned variants. This allows you to reduce the amount of copies realized in your program.

The examples have been written with C11 in mind, using the conventions we encourage you to follow.

Finally, we strongly advise that you refrain from using structure field that starts with `_`:

- We try to maintain a common API between `zenoh-c` and [`zenoh-pico`](https://github.com/eclipse-zenoh/zenoh-pico), such that porting code from one to the other is, ideally, trivial. However, some types must have distinct representations in either library, meaning that using these representations explicitly will get you in trouble when porting.
- We reserve the right to change the memory layout of any type which has `_`-prefixed fields, so trying to use them might cause your code to break on updates.

## Logging

By default, zenoh-c enables Zenoh's logging library upon using the `z_open` or `z_scout` functions. This behavior can be disabled by adding `-DDISABLE_LOGGER_AUTOINIT:bool=true` to the `cmake` configuration command. The logger may then be manually re-enabled with the `zc_init_logger` function.

## Cross-Compilation

The following alternative options have been introduced to facilitate cross-compilation.
> :warning: **WARNING** :warning: : Perhaps additional efforts are necessary, that will depend of your environment.

- `-DZENOHC_CARGO_CHANNEL="+nightly"|"+beta"|"+stable"`: refers to a specific rust toolchain release [[rust-channels](https://rust-lang.github.io/rustup/concepts/channels.html)]
- `-DZENOHC_CARGO_FLAGS`: several optional flags can be used for compilation. [[cargo flags](https://doc.rust-lang.org/cargo/commands/cargo-build.html)]
- `-DZENOHC_CUSTOM_TARGET`: specifies a crosscompilation target. Currently rust support several Tire-1, Tire-2 and Tire-3 targets [[targets](https://doc.rust-lang.org/nightly/rustc/platform-support.html)]. But keep in mind that zenoh-c only have support for following targets: `aarch64-unknown-linux-gnu`, `x86_64-unknown-linux-gnu`, `arm-unknown-linux-gnueabi`

Let's put all together in an example:
Assuming you want to crosscompile for aarch64-unknown-linux-gnu.

1. Install required packages
   - `sudo apt install gcc-aarch64-linux-gnu`
2. *(Only if you're using `nightly`)
   - `rustup component add rust-src --toolchain nightly`
3. Compile Zenoh-C. Assume that it's in `zenoh-c` directory. Notice that build in this sample is performed outside of source directory

   ```bash
   export RUSTFLAGS="-Clinker=aarch64-linux-gnu-gcc -Car=aarch64-linux-gnu-ar"
   mkdir -p build && cd build
   cmake ../zenoh-c  -DZENOHC_CARGO_CHANNEL="+nightly" -DZENOHC_CARGO_FLAGS="-Zbuild-std=std,panic_abort" -DZENOHC_CUSTOM_TARGET="aarch64-unknown-linux-gnu" -DCMAKE_INSTALL_PREFIX=../aarch64/stage
   cmake --build . --target install
   ```

Additionally you can use `RUSTFLAGS` environment variable for lead the compilation.

If all goes right the building files will be located at:
`/path/to/zenoh-c/target/aarch64-unknown-linux-gnu/release`
and release files will be located at
`/path/to/zenoh-c/target/aarch64-unknown-linux-gnu/release`

## Rust Version

The Rust version we use is defined in [rust-toolchain.toml](rust-toolchain.toml), which is `1.72.0`.
There might be some memory mapping issue if you use the later version.

You can also specify the Rust version.

```bash
cmake ../zenoh-c -DZENOHC_CARGO_CHANNEL="+1.72.0"
```

## Zenoh features support (enabling/disabling protocols, etc)

It's necessary sometimes to build zenoh-c library with set of features different from default. For example: enable TCP and UDP only. This can be done by changing `ZENOHC_CARGO_FLAGS` parameter for cmake (notice ";" instead of space due to cmake peculiarities)

Available features can be found in Zenoh [Cargo.toml](https://github.com/eclipse-zenoh/zenoh/blob/main/zenoh/Cargo.toml)

```bash
cmake ../zenoh-c -DZENOHC_CARGO_FLAGS="--no-default-features;--features=zenoh/transport_tcp,zenoh/transport_udp"
```
