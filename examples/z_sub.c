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
         sample->key, (int)sample->value.len, sample->value.val);
}

int main(int argc, char **argv)
{
  z_init_logger();

  char *uri = "/demo/example/**";
  if (argc > 1)
  {
    uri = argv[1];
  }
  z_owned_config_t config = z_config_default();
  if (argc > 2)
  {
    z_config_set(z_borrow(config), ZN_CONFIG_PEER_KEY, z_string_new(argv[2]));
  }

  printf("Openning session...\n");
  z_owned_session_t s = z_open(&config);
  if (!z_check(s))
  {
    printf("Unable to open session!\n");
    exit(-1);
  }

  printf("Declaring Subscriber on '%s'...\n", uri);
  z_owned_reskey_t key = z_rname(uri);
  z_owned_subscriber_t sub = z_register_subscriber(z_borrow(s), z_borrow(key), z_subinfo_default(), data_handler, NULL);
  if (!z_check(sub))
  {
    printf("Unable to declare subscriber.\n");
    exit(-1);
  }

  char c = 0;
  while (c != 'q')
  {
    c = fgetc(stdin);
  }
  z_reskey_free(&key);
  z_unregister_subscriber(&sub);
  z_close(&s);
  return 0;
}