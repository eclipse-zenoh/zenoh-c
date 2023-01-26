<img src="https://raw.githubusercontent.com/eclipse-zenoh/zenoh/master/zenoh-dragon.png" height="150">

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

> :warning: **WARNING** :warning: : Zenoh and its ecosystem are under active development. When you build from git, make sure you also build from git any other Zenoh repository you plan to use (e.g. binding, plugin, backend, etc.). It may happen that some changes in git are not compatible with the most recent packaged Zenoh release (e.g. deb, docker, pip). We put particular effort in mantaining compatibility between the various git repositories in the Zenoh project. 

1. Make sure that [rust](https://www.rust-lang.org) is available on your platform:

  -- Ubuntu -- 

  ```bash
  $ sudo apt-get install rustc
  ```

  -- MacOS -- 

  ```bash
  $ brew install rust
  ```

2. Clone the [source] with `git`:

   ```sh
   git clone https://github.com/eclipse-zenoh/zenoh-c.git
   cd zenoh-c
   ```

[source]: https://github.com/eclipse-zenoh/zenoh-c

3. Build and install:

  ```bash
  $ cd /path/to/zenoh-c
  $ mkdir -p build && cd build 
  $ cmake -DCMAKE_BUILD_TYPE=Release ..
  $ cmake --build .
  $ cmake --build . --target install # on linux use **sudo**
  ```

You may alternatively use `-DCMAKE_BUILD_TYPE=RelWithDebInfo` if you wish to keep the debug symbols.

Note that the `install` target is only available for `Release` and `RelWithDebInfo` builds.  
CMake also offers the `Debug` build type, which we do not allow as an `install` target since you may suffer a significant performance hit if accidentally using this one.  
Finally, CMake typicall offers a `MinSizeRel` build type. While we do not prevent you from using it, note that it is strictly equivalent to running a `Release` build.

## Building the Examples

  ```bash
  $ cd /path/to/zenoh-c
  $ mkdir -p build && cd build #
  $ cmake -DCMAKE_BUILD_TYPE=Release .. # If Ninja is installed on your system, adding `-GNinja` to this command can greatly speed up the build time
  $ cmake --build . --target examples
  ```

You may also use `--target <example_name>` if you wish to only build a specific example.

All build artifacts will be in the `/path/to/zenoh-c/target/release` directory.

## Running the Examples

### Basic Pub/Sub Example
```bash
$ ./target/release/examples/z_sub
```

```bash
$ ./target/release/examples/z_pub
```

### Queryable and Query Example
```bash
$ ./target/release/examples/z_queryable
```

```bash
$ ./target/release/examples/z_get
```

## Running the Throughput Examples
```bash
$ ./target/release/examples/z_sub_thgr
```

```bash
$ ./target/release/examples/z_pub_thgr
```

## API conventions
Many of the types exposed by the `zenoh-c` API are types for which destruction is necessary. To help you spot these types, we named them with the convention that  any destructible type must start by `z_owned`.

For maximum performance, we try to make as few copies as possible. Sometimes, this implies moving data that you `z_owned`. Any function that takes a non-const pointer to a `z_owned` type will perform its destruction. To make this pattern more obvious, we encourage you to use the `z_move` macro instead of a simple `&` to create these pointers. Rest assured that all `z_owned` types are double-free safe, and that you may check whether any `z_owned_X_t` typed value is still valid by using `z_X_check(&val)`, or the `z_check(val)` macro if you're using C11.

We hope this convention will help you streamline your memory-safe usage of zenoh, as following it should make looking for leaks trivial: simply search for paths where a value of a `z_owned` type hasn't been passed to a function using `z_move`.

Functions that simply need to borrow your data will instead take values of the associated `z_X_t` type. You may construct them using `z_X_loan(&val)` (or the `z_loan(val)` generic macro with C11).

Note that some `z_X_t` typed values can be constructed without needing to `z_borrow` their owned variants. This allows you to reduce the amount of copies realized in your program.

The examples have been written with C11 in mind, using the conventions we encourage you to follow.

Finally, we strongly advise that you refrain from using structure field that starts with `_`:
* We try to maintain a common API between `zenoh-c` and [`zenoh-pico`](https://github.com/eclipse-zenoh/zenoh-pico), such that porting code from one to the other is, ideally, trivial. However, some types must have distinct representations in either library, meaning that using these representations explicitly will get you in trouble when porting.
* We reserve the right to change the memory layout of any type which has `_`-prefixed fields, so trying to use them might cause your code to break on updates.

## Logging
By default, zenoh-c enables Zenoh's logging library upon using the `z_open` or `z_scout` functions. This behaviour can be disabled by adding `-DDISABLE_LOGGER_AUTOINIT:bool=true` to the `cmake` configuration command. The logger may then be manually re-enabled with the `zc_init_logger` function.

## Cross-Compilation
* The following alternative options have been introduced to facilitate cross-compilation.
> :warning: **WARNING** :warning: : Perhaps aditional efforts are neccesary, that will depend of your enviroment.

- `-DCARGO_CHANNEL=<+nightly>,<beta>,<stable>`: refers to a specific rust toolchain release [`rust-channels`] https://rust-lang.github.io/rustup/concepts/channels.html
- `-DCARGO_FLAGS`: several optional flags can be used for compilation. [`cargo flags`] https://doc.rust-lang.org/cargo/commands/cargo-build.html
- `-DCUSTOM_TARGET`: specifies a crosscompilation target. Currently rust support several Tire-1, Tire-2 and Tire-3 targets [`targets`] https://doc.rust-lang.org/nightly/rustc/platform-support.html. But keep in mind that zenoh-c only have support for following targets: `aarch64-unknown-linux-gnu`, `x86_64-unknown-linux-gnu`, `arm-unknown-linux-gnueabi`

Lets put all together in an example:
Assuming you want to crosscompile for aarch64-unknown-linux-gnu.

1. install required packages
  - `sudo apt install gcc-aarch64-linux-gnu`

  ```bash
  $ cd /path/to/zenoh-c
  $ mkdir -p aarch64/stage
  $ mkdir -p build && cd build build
  $ cmake -DCMAKE_BUILD_TYPE=Release -DCARGO_CHANNEL="+nightly" -DCARGO_FLAGS="-Zbuild-std=std,panic_abort" -DCUSTOM_TARGET="aarch64-unknown-linux-gnu" -DCMAKE_INSTALL_PREFIX=../aarch64/stage
  $ cmake --build .
  $ cmake --build . --target install # on linux use **sudo**
  ```
Additionaly you can use `RUSTFLAGS` enviroment variable for lead the compilation. 


If all goes right the building files will be located at:
`/path/to/zenoh-c/target/aarch64-unknown-linux-gnu/release`
and release files will be located at
`/path/to/zenoh-c/target/aarch64-unknown-linux-gnu/release`

