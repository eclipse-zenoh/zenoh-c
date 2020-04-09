#
# Copyright (c) 2017, 2020 ADLINK Technology Inc.
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ADLINK zenoh team, <zenoh@adlink-labs.tech>
#
.PHONY: test clean

# Build type. This set the CMAKE_BUILD_TYPE variable.
# Accepted values: Release, Debug, GCov
BUILD_TYPE?=Release

# zenoh-c/ directory
ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
# Build directory
BUILD_DIR=build

CROSS_BUILD_TARGETS=manylinux2010-x64 manylinux2010-x86 linux-x64 linux-armv5 linux-armv6 linux-arm64 osx-64
CROSS_BUILD_DIR=$(BUILD_DIR)/crossbuilds
CROSS_SCRIPTS_DIR=crossbuilds

# NOTES:
# - ARM:   can't use dockcross/dockcross since it uses an old GCC (4.8.3) which lacks <stdatomic.h> (even using -std=gnu11)
# - MacOS: can't use multiarch/crossbuild since it uses Clang 3.5.0 which lacks <stdatomic.h> (even using -std=gnu11)
DOCKER_CROSSBUILD_IMAGE=multiarch/crossbuild
DOCKER_OSXCROSS_IMAGE=liushuyu/osxcross

ifneq ($(ZENOH_DEBUG),)
	ZENOH_DEBUG_OPT := -DZENOH_DEBUG=$(ZENOH_DEBUG)
endif

CMAKE_OPT=$(ZENOH_DEBUG_OPT) -DCMAKE_BUILD_TYPE=$(BUILD_TYPE) -H.

# ZENOH_JAVA: when building zenoh-c for zenoh-java:
ifneq ($(ZENOH_JAVA),)
	CMAKE_OPT += -DSWIG_JAVA=ON -DTESTS=OFF -DEXAMPLES=OFF
endif
ifneq ($(JNI_INCLUDE_HOME),)
	CMAKE_OPT += -DJNI_INCLUDE_HOME=$(JNI_INCLUDE_HOME)
endif

all: cmake make

all-cross: $(CROSS_BUILD_TARGETS)

cmake: CMakeLists.txt
	mkdir -p $(BUILD_DIR)
	cmake $(CMAKE_OPT) -B$(BUILD_DIR)

make: $(BUILD_DIR)/Makefile
	make -C$(BUILD_DIR)

install:
	make -C$(BUILD_DIR) install

test:
	make -C$(BUILD_DIR) test


DOCKER_OK := $(shell docker version 2> /dev/null)
DOCKER_CROSSBUILD_INFO := $(shell docker image inspect $(DOCKER_CROSSBUILD_IMAGE) 2> /dev/null)
DOCKER_OSXCROSS_INFO := $(shell docker image inspect $(DOCKER_OSXCROSS_IMAGE) 2> /dev/null)

check-docker:
ifndef DOCKER_OK
	$(error "Docker is not available. Please install Docker")
endif
ifeq ($(DOCKER_CROSSBUILD_INFO),[])
	docker pull $(DOCKER_CROSSBUILD_IMAGE)
endif
ifeq ($(DOCKER_OSXCROSS_INFO),[])
	docker pull $(DOCKER_OSXCROSS_IMAGE)
endif


linux-armv5: check-docker
	docker run --rm -v $(ROOT_DIR):/workdir -e CROSS_TRIPLE=arm-linux-gnueabi $(DOCKER_CROSSBUILD_IMAGE) \
		cmake $(CMAKE_OPT) -B$(CROSS_BUILD_DIR)/$@
	docker run --rm -v $(ROOT_DIR):/workdir -e CROSS_TRIPLE=arm-linux-gnueabi $(DOCKER_CROSSBUILD_IMAGE) \
		make VERBOSE=1 -C$(CROSS_BUILD_DIR)/$@

linux-armv6: check-docker
	docker run --rm -v $(ROOT_DIR):/workdir -e CROSS_TRIPLE=arm-linux-gnueabihf $(DOCKER_CROSSBUILD_IMAGE) \
		cmake $(CMAKE_OPT) -B$(CROSS_BUILD_DIR)/$@
	docker run --rm -v $(ROOT_DIR):/workdir -e CROSS_TRIPLE=arm-linux-gnueabihf $(DOCKER_CROSSBUILD_IMAGE) \
		make VERBOSE=1 -C$(CROSS_BUILD_DIR)/$@

linux-arm64: check-docker
	docker run --rm -v $(ROOT_DIR):/workdir -e CROSS_TRIPLE=aarch64-linux-gnu $(DOCKER_CROSSBUILD_IMAGE) \
		cmake $(CMAKE_OPT) -B$(CROSS_BUILD_DIR)/$@
	docker run --rm -v $(ROOT_DIR):/workdir -e CROSS_TRIPLE=aarch64-linux-gnu $(DOCKER_CROSSBUILD_IMAGE) \
		make VERBOSE=1 -C$(CROSS_BUILD_DIR)/$@

linux-x64: check-docker
	docker run --rm -v $(ROOT_DIR):/workdir $(DOCKER_CROSSBUILD_IMAGE) \
		cmake $(CMAKE_OPT) -B$(CROSS_BUILD_DIR)/$@
	docker run --rm -v $(ROOT_DIR):/workdir $(DOCKER_CROSSBUILD_IMAGE) \
		make VERBOSE=1 -C$(CROSS_BUILD_DIR)/$@

osx-64: check-docker
	docker run --rm -v $(ROOT_DIR):/workdir -w /workdir -e CC=x86_64-apple-darwin18-clang -e CXX=x86_64-apple-darwin18-clang $(DOCKER_OSXCROSS_IMAGE) \
		cmake $(CMAKE_OPT) -DCMAKE_SYSTEM_NAME=Darwin -B$(CROSS_BUILD_DIR)/$@
	docker run --rm -v $(ROOT_DIR):/workdir -w /workdir -e CC=x86_64-apple-darwin18-clang -e CXX=x86_64-apple-darwin18-clang $(DOCKER_OSXCROSS_IMAGE) \
		make VERBOSE=1 -C$(CROSS_BUILD_DIR)/$@

manylinux2010-x86: check-docker
	dockcross/dockcross-manylinux2010-x86 cmake $(CMAKE_OPT) -B$(CROSS_BUILD_DIR)/$@
	dockcross/dockcross-manylinux2010-x86 make VERBOSE=1 -C$(CROSS_BUILD_DIR)/$@

manylinux2010-x64: check-docker
	dockcross/dockcross-manylinux2010-x64 cmake $(CMAKE_OPT) -B$(CROSS_BUILD_DIR)/$@
	dockcross/dockcross-manylinux2010-x64 make VERBOSE=1 -C$(CROSS_BUILD_DIR)/$@


clean:
	rm -fr $(BUILD_DIR)
