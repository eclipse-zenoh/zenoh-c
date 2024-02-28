..
.. Copyright (c) 2022 ZettaScale Technology
..
.. This program and the accompanying materials are made available under the
.. terms of the Eclipse Public License 2.0 which is available at
.. http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
.. which is available at https://www.apache.org/licenses/LICENSE-2.0.
..
.. SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
..
.. Contributors:
..   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
..

********
Examples
********

Publish
=======

.. code-block:: c

  #include <string.h>
  #include "zenoh.h"

  int main(int argc, char **argv) {
      z_owned_config_t config = z_config_default();
      z_owned_session_t s = z_open(z_move(config));

      char* value = "value";
      z_put(z_loan(s), z_keyexpr("key/expression"), (const uint8_t *)value, strlen(value), NULL);

      z_close(z_move(s));
      return 0;
  }

Subscribe
=========

.. code-block:: c

  #include <stdio.h>
  #include "zenoh.h"

  void data_handler(const z_sample_t *sample, const void *arg) {
      z_owned_str_t keystr = z_keyexpr_to_string(sample->keyexpr);
      printf(">> Received (%s, %.*s)\n",
          keystr, (int)sample->payload.len, sample->payload.start);
      z_drop(z_move(keystr));
  }

  int main(int argc, char **argv) {
      z_owned_config_t config = z_config_default();
      z_owned_session_t s = z_open(z_move(config));

      z_owned_closure_sample_t callback = z_closure(data_handler);
      z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_keyexpr("key/expression"), z_move(callback), NULL);

      char c = 0;
      while (c != 'q') {
          c = fgetc(stdin);
      }

      z_undeclare_subscriber(z_move(sub));
      z_close(z_move(s));
      return 0;
  }

Query
=====

.. code-block:: c

  #include <stdio.h>
  #include "zenoh.h"

  int main(int argc, char** argv) {
      z_owned_config_t config = z_config_default();
      z_owned_session_t s = z_open(z_move(config));

      z_owned_reply_channel_t channel = z_reply_fifo_new(16);
      z_get(z_loan(s), z_keyexpr("key/expression"), "", z_move(channel.send), NULL);
      z_owned_reply_t reply = z_reply_null();
      for (z_call(channel.recv, &reply); z_check(reply); z_call(channel.recv, &reply))
      {
          if (z_reply_is_ok(&reply))
          {
              z_sample_t sample = z_reply_ok(&reply);
              z_owned_str_t keystr = z_keyexpr_to_string(sample.keyexpr);
              printf(">> Received ('%s': '%.*s')\n", keystr, (int)sample.payload.len, sample.payload.start);
              z_drop(z_move(keystr));
          }
      }

      z_drop(reply);
      z_drop(channel);
      z_close(z_move(s));
      return 0;
  }