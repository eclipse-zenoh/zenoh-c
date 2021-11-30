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
.PHONY: lib examples all install clean

# Library name
ifeq ($(OS),Windows_NT)
  LIB_NAME=libzenohc.dll
  STATIC_LDFLAGS=-lrt
else
  ifeq ($(shell uname -s),Darwin)
    LIB_NAME=libzenohc.dylib
    STATIC_LDFLAGS=-framework Security -framework Foundation
  else
    LIB_NAME=libzenohc.so
    STATIC_LDFLAGS=-lrt
  endif
endif

ifneq ($(TARGET),)
  TARGET_OPT=--target=$(TARGET)
endif

ifeq ($(BUILD_TYPE),Debug)
  BUILD_DIR=target${TARGET}/debug
  CARGOFLAGS=
  EXAMPLES=z_sub z_pub z_put z_get z_eval z_pull z_info z_scout
  LDFLAGS=
  CFLAGS=-g
else 
  BUILD_DIR=target${TARGET}/release
  CARGOFLAGS=--release
  EXAMPLES=z_sub z_pub z_put z_get z_eval z_pull z_info z_scout z_sub_thr z_pub_thr
  LDFLAGS=-O3
  CFLAGS=
endif

# Installation prefix
ifeq ($(PREFIX),)
  PREFIX=/usr/local
endif

mkdirs:
	mkdir -p $(BUILD_DIR)/examples/static_link
	mkdir -p $(BUILD_DIR)/examples/dynamic_link

build: $(BUILD_DIR)/$(LIB_NAME)

static_examples: $(addprefix $(BUILD_DIR)/examples/static_link/, $(EXAMPLES))
dynamic_examples: $(addprefix $(BUILD_DIR)/examples/dynamic_link/, $(EXAMPLES))

examples: mkdirs static_examples dynamic_examples
all: build examples

$(BUILD_DIR)/$(LIB_NAME): Cargo.toml Cargo.lock build.rs splitguide.yaml cbindgen.toml src/lib.rs src/types.rs
	cargo build ${CARGOFLAGS} ${TARGET_OPT}

$(BUILD_DIR)/examples/static_link/%: examples/%.c include/zenoh-macros.h include/zenoh-concrete.h include/zenoh-commons.h include/zenoh.h $(BUILD_DIR)/$(LIB_NAME)
	$(CC) -Wall -s -o $@ $< -Iinclude $(BUILD_DIR)/libzenohc.a -lpthread -ldl -lm $(CFLAGS) $(LDFLAGS) $(STATIC_LDFLAGS)

$(BUILD_DIR)/examples/dynamic_link/%: examples/%.c include/zenoh-macros.h include/zenoh-concrete.h include/zenoh-commons.h include/zenoh.h $(BUILD_DIR)/$(LIB_NAME)
	$(CC) -Wall -s -o $@ $< -Iinclude -L$(BUILD_DIR) -lzenohc $(CFLAGS) $(LDFLAGS)

install: build
	install -d $(DESTDIR)$(PREFIX)/lib/
	install -m 755 $(BUILD_DIR)/$(LIB_NAME) $(DESTDIR)$(PREFIX)/lib/
	install -d $(DESTDIR)$(PREFIX)/include/
	install -m 755 include/zenoh-macros.h include/zenoh-concrete.h include/zenoh-commons.h include/zenoh.h $(DESTDIR)$(PREFIX)/include/

clean:
	rm -fr target
