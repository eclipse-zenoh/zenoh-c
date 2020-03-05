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

BUILD_DIR=build
CROSS_BUILD_DIR=$(BUILD_DIR)/dockcross
CROSS_SCRIPTS_DIR=dockcross

ifneq ($(ZENOH_DEBUG),)
	ZENOH_DEBUG_OPT := -DZENOH_DEBUG=$(ZENOH_DEBUG)
endif

all: cmake-debug make

gcov: cmake-gcov make

release: cmake-release make

cmake-debug: CMakeLists.txt
	mkdir -p $(BUILD_DIR)
	cmake $(ZENOH_DEBUG_OPT) -DCMAKE_BUILD_TYPE=Debug -B$(BUILD_DIR) -H.

cmake-gcov: CMakeLists.txt
	mkdir -p $(BUILD_DIR)
	cmake $(ZENOH_DEBUG_OPT) -DCMAKE_BUILD_TYPE=GCov -B$(BUILD_DIR) -H.

cmake-release: CMakeLists.txt
	mkdir -p $(BUILD_DIR)
	cmake $(ZENOH_DEBUG_OPT) -DCMAKE_BUILD_TYPE=Release -B$(BUILD_DIR) -H.

make: $(BUILD_DIR)/Makefile
	make -C$(BUILD_DIR)

install:
	make -C$(BUILD_DIR) install

test:
	make -C$(BUILD_DIR) test

all-cross: check-docker $(CROSS_BUILD_DIR)/linux-x64 $(CROSS_BUILD_DIR)/linux-armv6


DOCKER_OK := $(shell docker version 2> /dev/null)

check-docker:
ifndef DOCKER_OK
	$(error "Docker is not available please install Docker")
endif


$(CROSS_BUILD_DIR)/%: CMakeLists.txt
	$(CROSS_SCRIPTS_DIR)/dockcross-$* cmake -DJAVA_HOME=${JAVA_HOME} -DCMAKE_BUILD_TYPE=Release -B$@ -H.
	$(CROSS_SCRIPTS_DIR)/dockcross-$* make -C$@


clean:
	rm -fr $(BUILD_DIR)
