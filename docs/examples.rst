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
      z_owned_config_t config;
      z_config_default(&config);
      z_owned_session_t s;
      if (z_open(&s, z_move(config)) != 0) {
          printf("Failed to open Zenoh session\n");
          exit(-1);
      }
      
      z_view_str_t value;
      z_view_str_t_wrap(&value, "value");
      z_owned_bytes_t payload;
      z_bytes_encode_from_string(&payload, z_loan(value));
      z_view_keyexpr_t key_expr;
      z_view_keyexpr_from_string(&key_expr, "key/expression");

      z_put(z_loan(s), z_loan(key_expr), z_move(payload), NULL);

      z_close(z_move(s));
      return 0;
  }

Subscribe
=========

.. code-block:: c

  #include <stdio.h>
  #include "zenoh.h"

  void data_handler(const z_loaned_sample_t *sample, const void *arg) {
      z_owned_str_t key_string;
      z_keyexpr_to_string(z_sample_keyexpr(sample), &key_string);
      z_owned_str_t payload_string;
      z_bytes_decode_into_string(z_sample_payload(sample), &payload_string);
      printf(">> Received (%s, %.*s)\n", z_str_data(z_loan(key_string)), z_str_data(z_loan(payload_string)));

      z_drop(z_move(key_string));
      z_drop(z_move(payload_string));
  }

  int main(int argc, char **argv) {
      z_owned_config_t config;
      z_config_default(&config);
      z_owned_session_t s;
      if (z_open(&s, z_move(config)) != 0) {
          printf("Failed to open Zenoh session.\n");
          exit(-1);
      }

      z_owned_closure_sample_t callback;
      z_closure(&callback, data_handler, NULL, NULL);

      z_view_keyexpr_t key_expr;
      z_view_keyexpr_from_string(&key_expr, "key/expression");

      z_owned_subscriber_t sub;
      if (z_declare_subscriber(&sub, z_loan(s), z_loan(key_expr) z_move(callback), NULL) != 0) {
          printf("Unable to create Zenoh subscriber.\n");
          z_close(z_move(s));
          exit(-1);
      }

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
      z_owned_config_t config;
      z_config_default(&config);
      z_owned_session_t s;
      if (z_open(&s, z_move(config)) != 0) {
          printf("Failed to open Zenoh session.\n");
          exit(-1);
      }

      z_view_keyexpr_t key_expr;
      z_view_keyexpr_from_string(&key_expr, "key/expression");

      z_owned_reply_channel_t channel;
      zc_reply_fifo_new(&channel, 16);

      z_get(z_loan(s), z_loan(key_expr), "", z_move(channel.send), NULL);
      z_owned_reply_t reply;
      for (z_call(channel.recv, &reply); z_check(reply); z_call(channel.recv, &reply)) {
          if (z_reply_is_ok(&reply)) {
              const z_loaned_sample_t* sample = z_reply_ok(&reply);
              z_owned_str_t key_string;
              z_keyexpr_to_string(z_sample_keyexpr(sample), &key_string);
              z_owned_str_t payload_string;
              z_bytes_decode_into_string(z_sample_payload(sample), &payload_string);
              printf(">> Received (%s, %.*s)\n", z_str_data(z_loan(key_string)), z_str_data(z_loan(payload_string)));
              z_drop(z_move(key_string));
              z_drop(z_move(payload_string));
          }
      }

      z_drop(reply);
      z_drop(channel);
      z_close(z_move(s));
      return 0;
  }


Queryable
=========

.. code-block:: c

  #include <stdio.h>
  #include "zenoh.h"

  void query_handler(const z_loaned_query_t *query, void *context) {
      z_owned_str_t key_string;
      z_keyexpr_to_string(z_query_keyexpr(query), &key_string);

      const z_loaned_bytes_t* payload =  z_value_payload(z_query_value(query));
      if (z_bytes_len(payload) > 0) {
          z_owned_str_t payload_string;
          z_bytes_decode_into_string(payload, &payload_string);

          printf(">> [Queryable ] Received Query '%s' with value '%s'\n", 
              z_str_data(z_loan(key_string)), z_str_data(z_loan(payload_string)));
        z_drop(z_move(payload_string));
        } else {
            printf(">> [Queryable ] Received Query '%s'\n", z_str_data(z_loan(key_string)));
      }

      z_view_str_t reply_string;
      z_view_str_wrap(&reply_string, "reply");
      z_owned_bytes_t reply_payload;
      z_bytes_encode_from_string(&reply_payload, z_loan(reply_string));

      z_view_keyexpr_t reply_keyexpr;
      z_view_keyexpr_from_string(&reply_keyexpr, (const char *)context);

      z_query_reply(query, z_loan(reply_keyexpr), z_move(reply_payload), &options);
      z_drop(z_move(key_string));
  }

  int main(int argc, char **argv) {
      z_owned_config_t config;
      z_config_default(&config);
      z_owned_session_t s;
      if (z_open(&s, z_move(config)) != 0) {
          printf("Failed to open Zenoh session\n");
          exit(-1);
      }

      z_view_keyexpr_t key_expr;
      z_view_keyexpr_from_string(&key_expr, "key/expression");

      z_owned_closure_query_t callback;
      z_closure(&callback, query_handler, NULL, (void*)keyexpr);
      z_owned_queryable_t qable;

      if (z_declare_queryable(&qable, z_loan(s), z_loan(ke), z_move(callback), NULL) < 0) {
          printf("Unable to create Zenoh queryable.\n");
          exit(-1);
      }

      char c = 0;
      while (c != 'q') {
          c = fgetc(stdin);
      }

      z_undeclare_queryable(z_move(qable));
      z_close(z_move(s));
      return 0;
  }