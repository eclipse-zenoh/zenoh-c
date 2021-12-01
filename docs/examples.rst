..
.. Copyright (c) 2017, 2020 ADLINK Technology Inc.
..
.. This program and the accompanying materials are made available under the
.. terms of the Eclipse Public License 2.0 which is available at
.. http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
.. which is available at https://www.apache.org/licenses/LICENSE-2.0.
..
.. SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
..
.. Contributors:
..   ADLINK zenoh team, <zenoh@adlink-labs.tech>
..

********
Examples
********

Publish
=======

.. code-block:: c

  #include <string.h>
  #include "zenoh/net.h"

  int main(int argc, char **argv) {
      char* value = "value";

      z_session_t *s = z_open(z_config_default());
      z_put(s, z_expr("/res/name"), (const uint8_t *)value, strlen(value));
      z_close(s);

      return 0;
  }

Subscribe
=========

.. code-block:: c

  #include <stdio.h>
  #include "zenoh/net.h"

  void data_handler(const z_sample_t *sample, const void *arg) {
      printf(">> Received (%.*s, %.*s)\n",
          (int)sample->key.len, sample->key.val,
          (int)sample->value.len, sample->value.val);
  }

  int main(int argc, char **argv) {
      z_session_t *s = z_open(z_config_default());
      z_subscriber_t *sub = z_declare_subscriber(s, z_expr("/res/name"), z_subinfo_default(), data_handler, NULL);

      char c = 0;
      while (c != 'q') {
          c = fgetc(stdin);
      }

      z_undeclare_subscriber(sub);
      z_close(s);
      return 0;
  }

Query
=====

.. code-block:: c

  #include <stdio.h>
  #include <unistd.h>
  #include <string.h>
  #include "zenoh/net.h"

  int main(int argc, char** argv) {
      z_session_t *s = z_open(z_config_default());
      z_reply_data_array_t replies = z_query_collect(s, z_expr("/res/name"), "", z_query_target_default(), z_query_consolidation_default());
      
      for(unsigned int i = 0; i < replies.len; ++i) {
          printf(">> Received (%.*s, %.*s)\n",
            (int)replies.val[i].data.key.len, replies.val[i].data.key.val,
            (int)replies.val[i].data.value.len, replies.val[i].data.value.val);
      }
      z_reply_data_array_free(replies);

      z_close(s);
      return 0;
  }