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
      if (z_open(&s, z_move(config), NULL) < 0) {
          printf("Failed to open Zenoh session\n");
          exit(-1);
      }
      
      z_owned_bytes_t payload;
      z_bytes_from_static_str(&payload, "value");
      z_view_keyexpr_t key_expr;
      z_view_keyexpr_from_str(&key_expr, "key/expression");

      z_put(z_loan(s), z_loan(key_expr), z_move(payload), NULL);

      z_drop(z_move(s));
      return 0;
  }

Subscribe
=========

.. code-block:: c

  #include <stdio.h>
  #include "zenoh.h"

  void data_handler(z_loaned_sample_t *sample, void *arg) {
      z_view_string_t key_string;
      z_keyexpr_as_view_string(z_sample_keyexpr(sample), &key_string);
      z_owned_string_t payload_string;
      z_bytes_to_string(z_sample_payload(sample), &payload_string);
      printf(">> Received (%.*s, %.*s)\n", 
          (int)z_string_len(z_loan(key_string)), z_string_data(z_loan(key_string)), 
          (int)z_string_len(z_loan(payload_string)), z_string_data(z_loan(payload_string))
      );

      z_drop(z_move(payload_string));
  }

  int main(int argc, char **argv) {
      z_owned_config_t config;
      z_config_default(&config);
      z_owned_session_t s;
      if (z_open(&s, z_move(config), NULL) < 0) {
          printf("Failed to open Zenoh session.\n");
          exit(-1);
      }

      z_owned_closure_sample_t callback;
      z_closure(&callback, data_handler, NULL, NULL);

      z_view_keyexpr_t key_expr;
      z_view_keyexpr_from_str(&key_expr, "key/expression");

      z_owned_subscriber_t sub;
      if (z_declare_subscriber(z_loan(s), &sub, z_loan(key_expr), z_move(callback), NULL) < 0) {
          printf("Unable to create Zenoh subscriber.\n");
          z_drop(z_move(s));
          exit(-1);
      }

      char c = 0;
      while (c != 'q') {
          c = fgetc(stdin);
      }

      z_drop(z_move(sub));
      z_drop(z_move(s));
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
      if (z_open(&s, z_move(config), NULL) < 0) {
          printf("Failed to open Zenoh session.\n");
          exit(-1);
      }

      z_view_keyexpr_t key_expr;
      z_view_keyexpr_from_str(&key_expr, "key/expression");

      z_owned_fifo_handler_reply_t handler;
      z_owned_closure_reply_t closure;
      z_fifo_channel_reply_new(&closure, &handler, 16);

      z_get(z_loan(s), z_loan(key_expr), "", z_move(closure), NULL);
      z_owned_reply_t reply;
      for (z_result_t res = z_recv(z_loan(handler), &reply); res == Z_OK; res = z_recv(z_loan(handler), &reply)) {
          if (z_reply_is_ok(z_loan(reply))) {
              const z_loaned_sample_t* sample = z_reply_ok(z_loan(reply));
              z_view_string_t key_string;
              z_keyexpr_as_view_string(z_sample_keyexpr(sample), &key_string);
              z_owned_string_t payload_string;
              z_bytes_to_string(z_sample_payload(sample), &payload_string);
              printf(">> Received (%.*s, %.*s)\n",
                  (int)z_string_len(z_loan(key_string)), z_string_data(z_loan(key_string)),
                  (int)z_string_len(z_loan(payload_string)), z_string_data(z_loan(payload_string))
              );
              z_drop(z_move(payload_string));
          }
      }

      z_drop(z_move(reply));
      z_drop(z_move(s));
      return 0;
  }


Queryable
=========

.. code-block:: c

  #include <stdio.h>
  #include "zenoh.h"

  void query_handler(z_loaned_query_t *query, void *context) {
      z_view_string_t key_string;
      z_keyexpr_as_view_string(z_query_keyexpr(query), &key_string);

      const z_loaned_bytes_t* payload = z_query_value(query);
      if (z_bytes_len(payload) > 0) {
          z_owned_string_t payload_string;
          z_bytes_to_string(payload, &payload_string);

          printf(">> [Queryable ] Received Query '%.*s' with value '%.*s'\n", 
              (int)z_string_len(z_loan(key_string)), z_string_data(z_loan(key_string)),
              (int)z_string_len(z_loan(payload_string)), z_string_data(z_loan(payload_string)));
          z_drop(z_move(payload_string));
      } else {
          printf(">> [Queryable ] Received Query '%s'\n", z_string_data(z_loan(key_string)));
      }

      z_owned_bytes_t reply_payload;
      z_bytes_from_static_str(&reply_payload, "reply");

      z_view_keyexpr_t reply_keyexpr;
      z_view_keyexpr_from_str(&reply_keyexpr, (const char *)context);

      z_query_reply(query, z_loan(reply_keyexpr), z_move(reply_payload), NULL);
  }

  int main(int argc, char **argv) {
      z_owned_config_t config;
      z_config_default(&config);
      z_owned_session_t s;
      if (z_open(&s, z_move(config), NULL) < 0) {
          printf("Failed to open Zenoh session\n");
          exit(-1);
      }

      z_view_keyexpr_t key_expr;
      z_view_keyexpr_from_str(&key_expr, "key/expression");

      z_owned_closure_query_t callback;
      z_closure(&callback, query_handler, NULL, &key_expr);
      z_owned_queryable_t qable;

      if (z_declare_queryable(z_loan(s), &qable, z_loan(key_expr), z_move(callback), NULL) < 0) {
          printf("Unable to create Zenoh queryable.\n");
          exit(-1);
      }

      char c = 0;
      while (c != 'q') {
          c = fgetc(stdin);
      }

      z_drop(z_move(qable));
      z_drop(z_move(s));
      return 0;
  }
