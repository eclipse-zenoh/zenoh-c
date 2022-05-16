<img src="https://raw.githubusercontent.com/eclipse-zenoh/zenoh/master/zenoh-dragon.png" height="150">

[![CI](https://github.com/eclipse-zenoh/zenoh-c/workflows/CI/badge.svg)](https://github.com/eclipse-zenoh/zenoh-c/actions?query=workflow%3A%22CI%22)
[![Documentation Status](https://readthedocs.org/projects/zenoh-c/badge/?version=latest)](https://zenoh-c.readthedocs.io/en/latest/?badge=latest)
[![Discussion](https://img.shields.io/badge/discussion-on%20github-blue)](https://github.com/eclipse-zenoh/roadmap/discussions)
[![Discord](https://img.shields.io/badge/chat-on%20discord-blue)](https://discord.gg/vSDSpqnbkm)
[![License](https://img.shields.io/badge/License-EPL%202.0-blue)](https://choosealicense.com/licenses/epl-2.0/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)


# Eclipse zenoh C Client API

[Eclipse zenoh](http://zenoh.io) is an extremely efficient and fault-tolerant [Named Data Networking](http://named-data.net) (NDN) protocol 
that is able to scale down to extremely constrainded devices and networks. 

Check the website [zenoh.io](http://zenoh.io) and the [roadmap](https://github.com/eclipse-zenoh/roadmap) for more detailed information.

-------------------------------
## How to build it 

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
   cd rust
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

### Eval and Query Example
```bash
$ ./target/release/examples/z_eval
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
