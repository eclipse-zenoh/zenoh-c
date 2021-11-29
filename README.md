![zenoh banner](./zenoh-dragon.png)

[![CI](https://github.com/eclipse-zenoh/zenoh-c/workflows/CI/badge.svg)](https://github.com/eclipse-zenoh/zenoh-c/actions?query=workflow%3A%22CI%22)
[![Documentation Status](https://readthedocs.org/projects/zenoh-c/badge/?version=latest)](https://zenoh-c.readthedocs.io/en/latest/?badge=latest)
[![Gitter](https://badges.gitter.im/atolab/zenoh.svg)](https://gitter.im/atolab/zenoh?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)
[![License](https://img.shields.io/badge/License-EPL%202.0-blue)](https://choosealicense.com/licenses/epl-2.0/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)


# Eclipse zenoh C Client API

[Eclipse zenoh](http://zenoh.io) is an extremely efficient and fault-tolerant [Named Data Networking](http://named-data.net) (NDN) protocol 
that is able to scale down to extremely constrainded devices and networks. 

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
  $ make
  $ make install # on linux use **sudo**
  ```

If you want to build with debug symbols set the `BUILD_TYPE=Debug` environment variable before running `make` and `make install`:

  ```bash
  $ cd /path/to/zenoh-c
  $ export BUILD_TYPE=Debug 
  $ make
  $ make install # on linux use **sudo**
  ```

## Building the Examples

  ```bash
  $ cd /path/to/zenoh-c
  $ make examples
  ```

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

Functions that simply need to borrow your data will instead take values of the associated `z_X_t` type. You may construct them using `z_X_borrow(&val)` (or `z_borrow(val)` with C11).

Note that some `z_X_t` typed values can be constructed without needing to `z_borrow` their owned variants. This allows you to reduce the amount of copies realized in your program.

The examples have been written with C11 in mind, using the conventions we encourage you to follow.

Finally, we strongly advise that you refrain from using structure field that starts with `_`:
* We try to maintain a common API between `zenoh-c` and [`zenoh-pico`](https://github.com/eclipse-zenoh/zenoh-pico), such that porting code from one to the other is, ideally, trivial. However, some types must have distinct representations in either library, meaning that using these representations explicitly will get you in trouble when porting.
* We reserve the right to change the memory layout of any type which has `_`-prefixed fields, so trying to use them might cause your code to break on updates.