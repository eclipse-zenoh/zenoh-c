![zenoh banner](./zenoh-dragon.png)

![Build (simple)](https://github.com/eclipse-zenoh/zenoh-c/workflows/Build%20(simple)/badge.svg)
![Build cross-platforms](https://github.com/eclipse-zenoh/zenoh-c/workflows/Build%20cross-platforms/badge.svg)
[![Documentation Status](https://readthedocs.org/projects/zenoh-c/badge/?version=latest)](https://zenoh-c.readthedocs.io/en/latest/?badge=latest)
[![Gitter](https://badges.gitter.im/atolab/zenoh.svg)](https://gitter.im/atolab/zenoh?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)
[![License](https://img.shields.io/badge/License-EPL%202.0-blue)](https://choosealicense.com/licenses/epl-2.0/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)


# Eclipse zenoh C Client API

[Eclipse zenoh](http://zenoh.io) is an extremely efficient and fault-tolerant [Named Data Networking](http://named-data.net) (NDN) protocol 
that is able to scale down to extremely constrainded devices and networks. 

-------------------------------
## How to install it

Work in progress...

-------------------------------
## How to build it 
To build the **zenoh-c** client API you need to ensure that [rust](https://www.rust-lang.org) is available on your platform. 

  -- Ubuntu -- 

  ```bash
  $ sudo apt-get install rustc
  ```

  -- MacOS -- 

  ```bash
  $ brew install rust
  ```

The **zenoh-c** client API needs the nightly version of the rustc rust compiler.

  ```bash
  $ rustup toolchain install nightly
  $ rustup default nightly
  ```

The **zenoh-c** client API also needs [cbindgen](https://github.com/eqrion/cbindgen) to be installed on your platform.

  ```bash
  $ cargo install cbindgen
  ```

Once the above dependencies are satisfied, just do the following:

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
  $ make example
  ```

## Running the Examples

### Basic Pub/Sub Example
```bash
$ ./target/release/zn_sub
```

```bash
$ ./target/release/zn_pub
```

### Eval and Query Example
```bash
$ ./target/release/zn_eval
```

```bash
$ ./target/release/zn_query
```

## Running the Throughput Examples
```bash
$ ./target/release/zn_sub_thgr
```

```bash
$ ./target/release/zn_pub_thgr
```
