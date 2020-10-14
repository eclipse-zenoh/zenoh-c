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
.PHONY: lib example all install clean

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

ifeq ($(BUILD_TYPE),Debug)
  BUILD_DIR=target/debug
  CARGOFLAGS+=
  EXAMPLES=zn_sub zn_pub zn_write zn_query zn_eval zn_pull zn_info zn_scout
  LDFLAGS=
else 
  BUILD_DIR=target/release
  CARGOFLAGS+= --release
  EXAMPLES=zn_sub zn_pub zn_write zn_query zn_eval zn_pull zn_info zn_scout zn_sub_thr zn_pub_thr
  LDFLAGS=-O3
endif

# Installation prefix
ifeq ($(PREFIX),)
  PREFIX=/usr/local
endif

build: include/zenoh/net.h $(BUILD_DIR)/$(LIB_NAME)

examples: $(addprefix $(BUILD_DIR)/examples/, $(EXAMPLES))

all: build examples

$(BUILD_DIR)/$(LIB_NAME): src/lib.rs src/net/mod.rs
	cargo build ${CARGOFLAGS}

$(BUILD_DIR)/examples/%: examples/net/%.c include/zenoh/net.h $(BUILD_DIR)/$(LIB_NAME)
	$(CC) -o $@ $< -I include -L $(BUILD_DIR) -lzenohc $(CFLAGS) $(LDFLAGS)

include/zenoh/net.h: src/lib.rs src/net/mod.rs
	cbindgen --config cbindgen.toml --crate zenoh-c --output $@

install: $(BUILD_DIR)/$(LIB_NAME) include/zenoh.h include/zenoh/net.h
	install -d $(DESTDIR)$(PREFIX)/lib/
	install -m 755 $(BUILD_DIR)/$(LIB_NAME) $(DESTDIR)$(PREFIX)/lib/
	install -d $(DESTDIR)$(PREFIX)/include/
	install -m 755 include/zenoh.h $(DESTDIR)$(PREFIX)/include/
	install -d $(DESTDIR)$(PREFIX)/include/zenoh/
	install -m 755 include/zenoh/net.h $(DESTDIR)$(PREFIX)/include/zenoh/net.h

clean:
	rm -fr target include/zenoh/net.h
