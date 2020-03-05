![zenoh banner](./zenoh-dragon.png)

[![Build Status](https://travis-ci.com/eclipse-zenoh/zenoh-c.svg?branch=master)](https://travis-ci.com/eclipse-zenoh/zenoh-c)
[![codecov](https://codecov.io/gh/eclipse-zenoh/zenoh-c/branch/master/graph/badge.svg)](https://codecov.io/gh/eclipse-zenoh/zenoh-c)
[![Coverity Scan Build Status](https://scan.coverity.com/projects/19243/badge.svg)](https://scan.coverity.com/projects/eclipse-zenoh-c)
[![Documentation Status](https://readthedocs.org/projects/zenoh-c/badge/?version=latest)](https://zenoh-c.readthedocs.io/en/latest/?badge=latest)
[![Gitter](https://badges.gitter.im/atolab/zenoh.svg)](https://gitter.im/atolab/zenoh?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)
[![License](https://img.shields.io/badge/License-EPL%202.0-blue)](https://choosealicense.com/licenses/epl-2.0/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)


# Eclipse zenoh C Client API

## Building 
To build the **zenoh-c** client API you need to ensure that [cmake](https://cmake.org) is available on your platform -- if not please install it. 

Once the [cmake](https://cmake.org) dependency is satisfied, just do the following for **CMake** version 3 and higher:

  -- CMake version 3 and higher -- 

  ```bash
  $ cd /path/to/zenoh-c
  $ mkdir build
  $ cd build
  $ cmake -DCMAKE_BUILD_TYPE=Release ..
  $ make 
  $ make install # on linux use **sudo**
  ```

For those that still have **CMake** version 2.8, do the following commands:

  ```bash
  $ cd /path/to/zenoh-c
  $ mkdir build
  $ cd build
  $ cmake -DCMAKE_BUILD_TYPE=Release ../cmake-2.8
  $ make 
  $ make install # on linux use **sudo**
  ```

If you want to build with debug symbols configure with the option -DCMAKE_BUILD_TYPE=Debug.

## Running the Examples
The simplest way to run some of the example is to get a prebuilt instance of the **zenoh** network 
router at [http://github.com/atolab/atobin](http://github.com/atolab/atobin) and then to run the 
examples on your machine.

### Starting the zenoh Network Service
Assuming you've downloaded the **zenoh** network service on the current directory, then simply do:

```bash
$ ./zenohd 
```

To see the zenoh manual page, simply do:

```bash
$ ./zenohd --help
```


### Basic Pub/Sub Example
Assuming that (1) you are running the **zenoh** network service,  and (2) you are under the build directory, do:
```bash
$ ./z_sub
```

And on another shell, do:
```bash
$ ./z_pub
```
## Storage and Query Example
Assuming you are running the **zenoh** network service, do:
```bash
$ ./z_storage
```
And on another shell, do:
```bash
$ ./z_pub
```
After a few publications just terminate the publisher, and then try to query the storage:
```bash
$ ./z_query
```







