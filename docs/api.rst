..
.. Copyright (c) 2023 ZettaScale Technology
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

*************
API Reference
*************

Containers
=============

Slice
-----
Types
^^^^^
.. doxygenstruct:: z_owned_slice_t
.. doxygenstruct:: z_view_slice_t
.. doxygenstruct:: z_loaned_slice_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_slice_null
.. doxygenfunction:: z_view_slice_null
.. doxygenfunction:: z_slice_check
.. doxygenfunction:: z_view_slice_check
.. doxygenfunction:: z_slice_loan
.. doxygenfunction:: z_view_slice_loan
.. doxygenfunction:: z_slice_drop

.. doxygenfunction:: z_slice_empty
.. doxygenfunction:: z_view_slice_empty
.. doxygenfunction:: z_slice_wrap
.. doxygenfunction:: z_view_slice_wrap
.. doxygenfunction:: z_slice_from_str
.. doxygenfunction:: z_slice_data
.. doxygenfunction:: z_slice_len
.. doxygenfunction:: z_slice_is_empty


String
------
Types
^^^^^
.. doxygenstruct:: z_owned_str_t
.. doxygenstruct:: z_view_str_t
.. doxygenstruct:: z_loaned_str_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_str_check
.. doxygenfunction:: z_view_str_check
.. doxygenfunction:: z_str_null
.. doxygenfunction:: z_view_str_null
.. doxygenfunction:: z_str_loan
.. doxygenfunction:: z_view_str_loan
.. doxygenfunction:: z_str_drop

.. doxygenfunction:: z_str_empty
.. doxygenfunction:: z_view_str_empty

.. doxygenfunction:: z_str_wrap
.. doxygenfunction:: z_view_str_wrap
.. doxygenfunction:: z_str_from_substring
.. doxygenfunction:: z_str_data
.. doxygenfunction:: z_str_len
.. doxygenfunction:: z_str_is_empty

Slice map
---------
Types
^^^^^
.. doxygenstruct:: z_owned_slice_map_t
.. doxygenstruct:: z_loaned_slice_map_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_slice_map_check
.. doxygenfunction:: z_slice_map_null
.. doxygenfunction:: z_slice_map_drop
.. doxygenfunction:: z_slice_map_loan
.. doxygenfunction:: z_slice_map_loan_mut

.. doxygenfunction:: z_slice_map_new
.. doxygenfunction:: z_slice_map_get
.. doxygenfunction:: z_slice_map_len
.. doxygenfunction:: z_slice_map_is_empty
.. doxygenfunction:: z_slice_map_insert_by_alias
.. doxygenfunction:: z_slice_map_insert_by_copy
.. doxygenfunction:: z_slice_map_iterate

Slice array
-----------
Types
^^^^^
.. doxygenstruct:: z_owned_slice_array_t
.. doxygenstruct:: z_loaned_slice_array_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_slice_array_check
.. doxygenfunction:: z_slice_array_null
.. doxygenfunction:: z_slice_array_drop
.. doxygenfunction:: z_slice_array_loan
.. doxygenfunction:: z_slice_array_loan_mut

.. doxygenfunction:: z_slice_array_new
.. doxygenfunction:: z_slice_array_get
.. doxygenfunction:: z_slice_array_len
.. doxygenfunction:: z_slice_array_is_empty

Common
======

Key expression
--------------
Types
^^^^^
.. doxygenstruct:: z_owned_keyexpr_t
.. doxygenstruct:: z_view_keyexpr_t
.. doxygenstruct:: z_loaned_keyexpr_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_keyexpr_new
.. doxygenfunction:: z_view_keyexpr_new
.. doxygenfunction:: z_keyexpr_new_autocanonize
.. doxygenfunction:: z_view_keyexpr_new_autocanonize
.. doxygenfunction:: z_view_keyexpr_unchecked
.. doxygenfunction:: z_keyexpr_loan
.. doxygenfunction:: z_view_keyexpr_loan
.. doxygenfunction:: z_keyexpr_check
.. doxygenfunction:: z_view_keyexpr_check
.. doxygenfunction:: z_keyexpr_drop
.. doxygenfunction:: z_keyexpr_to_string
.. doxygenfunction:: z_keyexpr_as_slice
.. doxygenfunction:: z_keyexpr_canonize
.. doxygenfunction:: z_keyexpr_canonize_null_terminated
.. doxygenfunction:: z_keyexpr_is_canon
.. doxygenfunction:: z_keyexpr_concat
.. doxygenfunction:: z_keyexpr_join
.. doxygenfunction:: z_keyexpr_equals
.. doxygenfunction:: z_keyexpr_includes
.. doxygenfunction:: z_keyexpr_intersects
.. doxygenfunction:: z_declare_keyexpr
.. doxygenfunction:: z_undeclare_keyexpr

Encoding
--------
Types
^^^^^
.. doxygenstruct:: z_owned_encoding_t
.. doxygenstruct:: z_loaned_encoding_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_encoding_null
.. doxygenfunction:: z_encoding_loan
.. doxygenfunction:: z_encoding_check
.. doxygenfunction:: z_encoding_drop

.. doxygenfunction:: z_encoding_loan_default
.. doxygenfunction:: z_encoding_from_str

Value
-----
Types
^^^^^
.. doxygenstruct:: z_loaned_value_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_value_payload
.. doxygenfunction:: z_value_encoding

Sample
------
Types
^^^^^
.. doxygenstruct:: z_owned_sample_t
.. doxygenstruct:: z_loaned_sample_t
.. doxygenenum:: z_sample_kind_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_sample_loan
.. doxygenfunction:: z_sample_check
.. doxygenfunction:: z_sample_null
.. doxygenfunction:: z_sample_drop

.. doxygenfunction:: z_sample_timestamp
.. doxygenfunction:: z_sample_attachment
.. doxygenfunction:: z_sample_encoding
.. doxygenfunction:: z_sample_payload
.. doxygenfunction:: z_sample_priority
.. doxygenfunction:: z_sample_congestion_control
.. doxygenfunction:: z_sample_express



Timestamp
---------
Types
^^^^^
.. doxygenstruct:: z_timestamp_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_timestamp_id
.. doxygenfunction:: z_timestamp_npt64_time

System
======

Mutex
-----
Types
^^^^^
.. doxygenstruct:: z_owned_mutex_t
.. doxygenstruct:: z_loaned_mutex_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_mutex_check
.. doxygenfunction:: z_mutex_null
.. doxygenfunction:: z_mutex_loan_mut
.. doxygenfunction:: z_mutex_drop

.. doxygenfunction:: z_mutex_init
.. doxygenfunction:: z_mutex_lock
.. doxygenfunction:: z_mutex_unlock
.. doxygenfunction:: z_mutex_try_lock


Conditional Variable
--------------------
Types
^^^^^
.. doxygenstruct:: z_owned_condvar_t
.. doxygenstruct:: z_loaned_condvar_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_condvar_check
.. doxygenfunction:: z_condvar_null
.. doxygenfunction:: z_condvar_loan
.. doxygenfunction:: z_condvar_drop

.. doxygenfunction:: z_condvar_init
.. doxygenfunction:: z_condvar_wait
.. doxygenfunction:: z_condvar_signal


Task
----
Types
^^^^^
.. doxygenstruct:: z_owned_task_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_task_check
.. doxygenfunction:: z_task_null

.. doxygenfunction:: z_task_join
.. doxygenfunction:: z_task_detach

Session
=======

Session configuration
---------------------
Types
^^^^^
.. doxygenstruct:: z_owned_config_t
.. doxygenstruct:: z_loaned_config_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_config_null
.. doxygenfunction:: z_config_loan
.. doxygenfunction:: z_config_loan_mut
.. doxygenfunction:: z_config_check
.. doxygenfunction:: z_config_drop

.. doxygenfunction:: z_config_default
.. doxygenfunction:: z_config_client
.. doxygenfunction:: z_config_peer
.. doxygenfunction:: zc_config_from_file
.. doxygenfunction:: zc_config_from_str
.. doxygenfunction:: zc_config_insert_json
.. doxygenfunction:: zc_config_get
.. doxygenfunction:: zc_config_to_string

Session management
------------------

Types
^^^^^
.. doxygenstruct:: z_owned_session_t
.. doxygenstruct:: z_loaned_session_t
.. doxygenstruct:: z_id_t

.. doxygenstruct:: z_owned_closure_zid_t
    :members:

Functions
^^^^^^^^^
.. doxygenfunction:: z_open
.. doxygenfunction:: z_close

.. doxygenfunction:: z_session_loan
.. doxygenfunction:: z_session_check
.. doxygenfunction:: z_session_null
.. doxygenfunction:: z_session_drop

.. doxygenfunction:: zc_session_clone

.. doxygenfunction:: z_info_zid
.. doxygenfunction:: z_info_routers_zid
.. doxygenfunction:: z_info_peers_zid

.. doxygenfunction:: z_closure_zid_check
.. doxygenfunction:: z_closure_zid_null
.. doxygenfunction:: z_closure_zid_drop
.. doxygenfunction:: z_closure_zid_call

Publication
===========

Types
-----

.. doxygenstruct:: z_owned_publisher_t
.. doxygenstruct:: z_loaned_publisher_t

.. doxygenenum:: z_congestion_control_t
.. doxygenenum:: z_priority_t

.. doxygenstruct:: z_put_options_t
    :members:
.. doxygenstruct:: z_delete_options_t
.. doxygenstruct:: z_publisher_options_t
    :members:
.. doxygenstruct:: z_publisher_put_options_t
    :members:
.. doxygenstruct:: z_publisher_delete_options_t

.. doxygenstruct:: zcu_owned_matching_listener_t
.. doxygenstruct:: zcu_owned_closure_matching_status_t
    :members:

Functions
---------
.. doxygenfunction:: z_put
.. doxygenfunction:: z_delete

.. doxygenfunction:: z_declare_publisher
.. doxygenfunction:: z_publisher_put
.. doxygenfunction:: z_publisher_delete
.. doxygenfunction:: z_undeclare_publisher
.. doxygenfunction:: z_publisher_keyexpr

.. doxygenfunction:: z_publisher_null
.. doxygenfunction:: z_publisher_loan
.. doxygenfunction:: z_publisher_check
.. doxygenfunction:: z_publisher_drop

.. doxygenfunction:: z_put_options_default
.. doxygenfunction:: z_delete_options_default
.. doxygenfunction:: z_publisher_options_default
.. doxygenfunction:: z_publisher_put_options_default
.. doxygenfunction:: z_publisher_delete_options_default

.. doxygenfunction:: zcu_closure_matching_status_check
.. doxygenfunction:: zcu_closure_matching_status_null
.. doxygenfunction:: zcu_closure_matching_status_drop
.. doxygenfunction:: zcu_closure_matching_status_call

Subscription
============

Types
-----
.. doxygenstruct:: z_owned_subscriber_t
.. doxygenstruct:: z_loaned_subscriber_t

.. doxygenstruct:: z_owned_closure_sample_t
    :members:

.. doxygenenum:: z_reliability_t

.. doxygenstruct:: z_subscriber_options_t
    :members:

Functions
---------

.. doxygenfunction:: z_declare_subscriber
.. doxygenfunction:: z_undeclare_subscriber

.. doxygenfunction:: z_subscriber_check
.. doxygenfunction:: z_subscriber_null
.. doxygenfunction:: z_subscriber_drop

.. doxygenfunction:: z_closure_sample_call
.. doxygenfunction:: z_closure_sample_drop
.. doxygenfunction:: z_closure_sample_null
.. doxygenfunction:: z_closure_sample_check

.. doxygenfunction:: z_subscriber_options_default

Queryable
=========

Types
-----

.. doxygenstruct:: z_owned_queryable_t

.. doxygenstruct:: z_owned_query_t
.. doxygenstruct:: z_loaned_query_t

.. doxygenstruct:: z_owned_closure_query_t
    :members:

.. doxygenstruct:: z_queryable_options_t
    :members:
.. doxygenstruct:: z_query_reply_options_t
    :members:

Functions
---------
.. doxygenfunction::z_declare_queryable
.. doxygenfunction::z_undeclare_queryable

.. doxygenfunction::z_queryable_options_default
.. doxygenfunction::z_query_reply_options_default

.. doxygenfunction::z_queryable_null
.. doxygenfunction::z_queryable_check
.. doxygenfunction::z_queryable_drop

.. doxygenfunction::z_query_null
.. doxygenfunction::z_query_check
.. doxygenfunction::z_query_loan
.. doxygenfunction::z_query_drop
.. doxygenfunction:: z_query_clone

.. doxygenfunction:: z_query_keyexpr
.. doxygenfunction:: z_query_parameters
.. doxygenfunction:: z_query_value
.. doxygenfunction:: z_query_attachment
.. doxygenfunction:: z_query_reply

.. doxygenfunction:: z_closure_query_null
.. doxygenfunction:: z_closure_query_check
.. doxygenfunction:: z_closure_query_call
.. doxygenfunction:: z_closure_query_drop


Query
=====
Types
-----
.. doxygenstruct:: z_owned_reply_t
.. doxygenstruct:: z_loaned_reply_t

.. doxygenstruct:: z_owned_closure_reply_t
    :members:

.. doxygenstruct:: z_get_options_t
    :members:
.. doxygenenum:: z_query_target_t
.. doxygenenum:: z_consolidation_mode_t
.. doxygenstruct:: z_query_consolidation_t

Functions
---------

.. doxygenfunction:: z_get
.. doxygenfunction:: z_get_options_default

.. doxygenfunction:: z_query_consolidation_default
.. doxygenfunction:: z_query_consolidation_auto
.. doxygenfunction:: z_query_consolidation_none
.. doxygenfunction:: z_query_consolidation_monotonic
.. doxygenfunction:: z_query_consolidation_latest
.. doxygenfunction:: z_query_target_default

.. doxygenfunction:: z_reply_check
.. doxygenfunction:: z_reply_drop
.. doxygenfunction:: z_reply_clone
.. doxygenfunction:: z_reply_is_ok
.. doxygenfunction:: z_reply_ok
.. doxygenfunction:: z_reply_err
.. doxygenfunction:: z_reply_null

.. doxygenfunction:: z_closure_reply_call
.. doxygenfunction:: z_closure_reply_drop
.. doxygenfunction:: z_closure_reply_null
.. doxygenfunction:: z_closure_reply_check

Scouting
========

Types
-----
.. doxygenstruct:: z_owned_hello_t
.. doxygenstruct:: z_loaned_hello_t
.. doxygenstruct:: z_scout_options_t
    :members:
.. doxygenstruct:: z_owned_closure_hello_t
    :members:

Functions
---------
.. doxygenfunction:: z_scout

.. doxygenfunction:: z_hello_whatami
.. doxygenfunction:: z_hello_locators
.. doxygenfunction:: z_hello_zid
.. doxygenfunction:: z_hello_loan
.. doxygenfunction:: z_hello_drop 
.. doxygenfunction:: z_hello_null 
.. doxygenfunction:: z_hello_check

.. doxygenfunction:: z_whatami_to_str

.. doxygenfunction:: z_scout_options_default

.. doxygenfunction:: z_closure_hello_call
.. doxygenfunction:: z_closure_hello_drop
.. doxygenfunction:: z_closure_hello_null
.. doxygenfunction:: z_closure_hello_check

Liveliness
==========

Types
-----

.. doxygenstruct:: zc_owned_liveliness_token_t
.. doxygenstruct:: zc_liveliness_declaration_options_t
.. doxygenstruct:: zc_liveliness_get_options_t
.. doxygenstruct:: zc_liveliness_declare_subscriber_options_t

Functions
---------

.. doxygenfunction:: zc_liveliness_declare_token
.. doxygenfunction:: zc_liveliness_undeclare_token
.. doxygenfunction:: zc_liveliness_token_null
.. doxygenfunction:: zc_liveliness_token_check
.. doxygenfunction:: zc_liveliness_declare_subscriber
.. doxygenfunction:: zc_liveliness_get
.. doxygenfunction:: zc_liveliness_get_options_default

Publication Cache
=================

Types
-----

.. doxygenstruct:: ze_publication_cache_options_t
.. doxygenstruct:: ze_owned_publication_cache_t
.. doxygenenum:: zcu_locality_t

Functions
---------

.. doxygenfunction:: ze_declare_publication_cache
.. doxygenfunction:: ze_undeclare_publication_cache
.. doxygenfunction:: ze_publication_cache_check
.. doxygenfunction:: ze_publication_cache_null
.. doxygenfunction:: ze_publication_cache_options_default

Querying Subscriber
===================

Types
-----

.. doxygenstruct:: ze_owned_querying_subscriber_t
.. doxygenstruct:: ze_loaned_querying_subscriber_t
.. doxygenstruct:: ze_querying_subscriber_options_t
.. doxygenenum:: zcu_reply_keyexpr_t

Functions
---------

.. doxygenfunction:: ze_declare_querying_subscriber
.. doxygenfunction:: ze_undeclare_querying_subscriber
.. doxygenfunction:: ze_querying_subscriber_get
.. doxygenfunction:: ze_querying_subscriber_check
.. doxygenfunction:: ze_querying_subscriber_null
.. doxygenfunction:: ze_querying_subscriber_options_default
.. doxygenfunction:: zcu_reply_keyexpr_default
