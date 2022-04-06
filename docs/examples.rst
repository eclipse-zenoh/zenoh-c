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
      char* value = "value";

      z_owned_config_t config = z_config_default();
      z_owned_session_t s = z_open(z_move(config));
      z_put(z_borrow(s), z_expr("/key/expression"), (const uint8_t *)value, strlen(value));
      z_close(z_move(s));

      return 0;
  }

Subscribe
=========

.. code-block:: c

  #include <stdio.h>
  #include "zenoh.h"

  void data_handler(const z_sample_t *sample, const void *arg) {
      printf(">> Received (%.*s, %.*s)\n",
          (int)sample->key.suffix.len, sample->key.suffix.start,
          (int)sample->value.len, sample->value.start);
  }

  int main(int argc, char **argv) {
      z_owned_config_t config = z_config_default();
      z_owned_session_t s = z_open(z_move(config));
      z_owned_subscriber_t sub = z_subscribe(z_borrow(s), z_expr("/key/expression"), z_subinfo_default(), data_handler, NULL);

      char c = 0;
      while (c != 'q') {
          c = fgetc(stdin);
      }

      z_subscriber_close(z_move(sub));
      z_close(z_move(s));
      return 0;
  }

Query
=====

.. code-block:: c

  #include <stdio.h>
  #include <unistd.h>
  #include <string.h>
  #include "zenoh.h"

  int main(int argc, char** argv) {
      z_owned_config_t config = z_config_default();
      z_owned_session_t s = z_open(z_move(config));
      z_owned_reply_data_array_t replies = z_get_collect(z_borrow(s), z_expr("/key/expression"), "", z_query_target_default(), z_query_consolidation_default());

      for(unsigned int i = 0; i < replies.len; ++i) {
          printf(">> Received (%.*s, %.*s)\n",
            (int)replies.val[i].sample.key.suffix.len, replies.val[i].sample.key.suffix.start,
            (int)replies.val[i].sample.value.len, replies.val[i].sample.value.start);
      }
      z_reply_data_array_free(z_move(replies));

      z_close(z_move(s));
      return 0;
  }