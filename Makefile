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
.PHONY: build examples all install clean

# Library name
ifeq ($(OS),Windows_NT)
  LIB_NAME=libzenohc.dll
else
  ifeq ($(shell uname -s),Darwin)
    LIB_NAME=libzenohc.dylib
  else
    LIB_NAME=libzenohc.so
  endif
endif

ifneq ($(TARGET),)
  TARGET_OPT=--target=$(TARGET)
endif

ifeq ($(BUILD_TYPE),Debug)
  BUILD_DIR=target${TARGET}/debug
  CARGOFLAGS=
  EXAMPLES=z_sub z_pub z_write z_query z_eval z_pull z_info z_scout
  LDFLAGS=
else 
  BUILD_DIR=target${TARGET}/release
  CARGOFLAGS=--release
  EXAMPLES=z_sub z_pub z_write z_query z_eval z_pull z_info z_scout z_sub_thr z_pub_thr
  LDFLAGS=-O3
endif

# Installation prefix
ifeq ($(PREFIX),)
  PREFIX=/usr/local
endif

build: include/zenoh.h $(BUILD_DIR)/$(LIB_NAME)

examples: $(addprefix $(BUILD_DIR)/examples/, $(EXAMPLES))

all: build examples

$(BUILD_DIR)/$(LIB_NAME): Cargo.toml src/lib.rs src/types.rs
	cargo build ${CARGOFLAGS} ${TARGET_OPT}

$(BUILD_DIR)/examples/%: examples/%.c include/zenoh.h $(BUILD_DIR)/$(LIB_NAME)
	$(CC) -o $@ $< -Iinclude -L$(BUILD_DIR) -lzenohc $(CFLAGS) $(LDFLAGS)

include/zenoh.h: src/lib.rs src/types.rs cbindgen.toml
	cbindgen --config cbindgen.toml --crate zenoh-c --output $@

install: $(BUILD_DIR)/$(LIB_NAME) include/zenoh.h
	install -d $(DESTDIR)$(PREFIX)/lib/
	install -m 755 $(BUILD_DIR)/$(LIB_NAME) $(DESTDIR)$(PREFIX)/lib/
	install -d $(DESTDIR)$(PREFIX)/include/
	install -m 755 include/zenoh.h $(DESTDIR)$(PREFIX)/include/

clean:
	rm -fr target
