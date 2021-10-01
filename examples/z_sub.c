/*
 * Copyright (c) 2017, 2020 ADLINK Technology Inc.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Eclipse Public License 2.0 which is available at
 * http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
 * which is available at https://www.apache.org/licenses/LICENSE-2.0.
 *
 * SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
 *
 * Contributors:
 *   ADLINK zenoh team, <zenoh@adlink-labs.tech>
 */
#include <stdio.h>
#include "zenoh.h"

void data_handler(const z_sample_t *sample, const void *arg)
{
  printf(">> [Subscription listener] Received (%s, %.*s)\n",
         sample->key.start, (int)sample->value.len, sample->value.val);
}

int main(int argc, char **argv)
{
  z_init_logger();

  char *uri = "/demo/example/**";
  if (argc > 1)
  {
    uri = argv[1];
  }
  z_owned_config_t config = z_config__default();
  if (argc > 2)
  {
    z_config__insert(config.borrow, ZN_CONFIG_PEER_KEY, z_string__new(argv[2]));
  }

  printf("Openning session...\n");
  z_owned_session_t s = z_open(config);
  if (s.borrow == 0)
  {
    printf("Unable to open session!\n");
    exit(-1);
  }

  printf("Declaring Subscriber on '%s'...\n", uri);
  z_owned_subscriber_t sub = z_register_subscriber(
      s.borrow, z_rname(uri), z_subinfo__default(), data_handler, NULL);
  if (sub.borrow == 0)
  {
    printf("Unable to declare subscriber.\n");
    exit(-1);
  }

  char c = 0;
  while (c != 'q')
  {
    c = fgetc(stdin);
  }

  z_unregister_subscriber(sub);
  z_close(s);
  return 0;
}