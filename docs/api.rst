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

.. autocstruct:: zenoh_commons.h::z_view_slice_t
.. autocstruct:: zenoh_commons.h::z_owned_slice_t
.. autocstruct:: zenoh_commons.h::z_loaned_slice_t

.. autocfunction:: zenoh_commons.h::z_slice_empty
.. autocfunction:: zenoh_commons.h::z_view_slice_empty
.. autocfunction:: zenoh_commons.h::z_slice_wrap
.. autocfunction:: zenoh_commons.h::z_view_slice_wrap
.. autocfunction:: zenoh_commons.h::z_slice_from_str
.. autocfunction:: zenoh_commons.h::z_slice_check
.. autocfunction:: zenoh_commons.h::z_view_slice_check
.. autocfunction:: zenoh_commons.h::z_slice_drop
.. autocfunction:: zenoh_commons.h::z_view_slice_drop
.. autocfunction:: zenoh_commons.h::z_slice_loan
.. autocfunction:: zenoh_commons.h::z_view_slice_loan
.. autocfunction:: zenoh_commons.h::z_slice_data
.. autocfunction:: zenoh_commons.h::z_slice_len


String
-----

.. autocstruct:: zenoh_commons.h::z_view_str_t
.. autocstruct:: zenoh_commons.h::z_owned_str_t
.. autocstruct:: zenoh_commons.h::z_loaned_str_t

.. autocfunction:: zenoh_commons.h::z_str_empty
.. autocfunction:: zenoh_commons.h::z_view_str_empty
.. autocfunction:: zenoh_commons.h::z_str_check
.. autocfunction:: zenoh_commons.h::z_view_str_check
.. autocfunction:: zenoh_commons.h::z_view_str_null
.. autocfunction:: zenoh_commons.h::z_str_null
.. autocfunction:: zenoh_commons.h::z_str_wrap
.. autocfunction:: zenoh_commons.h::z_view_str_wrap
.. autocfunction:: zenoh_commons.h::z_str_from_substring
.. autocfunction:: zenoh_commons.h::z_str_drop
.. autocfunction:: zenoh_commons.h::z_view_str_drop
.. autocfunction:: zenoh_commons.h::z_str_loan
.. autocfunction:: zenoh_commons.h::z_view_str_loan
.. autocfunction:: zenoh_commons.h::z_str_data
.. autocfunction:: zenoh_commons.h::z_str_len

Slice map
---------

.. autocstruct:: zenoh_commons.h::z_owned_slice_map_t

.. autocfunction:: zenoh_commons.h::z_slice_map_new
.. autocfunction:: zenoh_commons.h::z_slice_map_check
.. autocfunction:: zenoh_commons.h::z_slice_map_null
.. autocfunction:: zenoh_commons.h::z_slice_map_drop
.. autocfunction:: zenoh_commons.h::z_slice_map_loan
.. autocfunction:: zenoh_commons.h::z_slice_map_loan_mut
.. autocfunction:: zenoh_commons.h::z_slice_map_get
.. autocfunction:: zenoh_commons.h::z_slice_map_len
.. autocfunction:: zenoh_commons.h::z_slice_map_is_empty
.. autocfunction:: zenoh_commons.h::z_slice_map_insert_by_alias
.. autocfunction:: zenoh_commons.h::z_slice_map_insert_by_copy
.. autocfunction:: zenoh_commons.h::z_slice_map_iterate

Slice array
---------

.. autocstruct:: zenoh_commons.h::z_owned_slice_array_t

.. autocfunction:: zenoh_commons.h::z_slice_array_new
.. autocfunction:: zenoh_commons.h::z_slice_array_check
.. autocfunction:: zenoh_commons.h::z_slice_array_null
.. autocfunction:: zenoh_commons.h::z_slice_array_drop
.. autocfunction:: zenoh_commons.h::z_slice_array_loan
.. autocfunction:: zenoh_commons.h::z_slice_array_loan_mut
.. autocfunction:: zenoh_commons.h::z_slice_array_get
.. autocfunction:: zenoh_commons.h::z_slice_array_len
.. autocfunction:: zenoh_commons.h::z_slice_array_is_empty

.. Scouting
.. ========

Session
=======

Session configuration
---------------------


.. autocstruct:: zenoh_commons.h::z_loaned_config_t
.. autocstruct:: zenoh_commons.h::z_owned_config_t
.. autocstruct:: zenoh_commons.h::z_owned_scouting_config_t

.. autocfunction:: zenoh_commons.h::z_config_default
.. autocfunction:: zenoh_commons.h::z_config_client
.. autocfunction:: zenoh_commons.h::z_config_peer
.. autocfunction:: zenoh_commons.h::zc_config_from_file
.. autocfunction:: zenoh_commons.h::zc_config_from_str
.. autocfunction:: zenoh_commons.h::zc_config_insert_json
.. autocfunction:: zenoh_commons.h::zc_config_get
.. autocfunction:: zenoh_commons.h::zc_config_to_string
.. autocfunction:: zenoh_commons.h::z_config_loan
.. autocfunction:: zenoh_commons.h::z_config_loan_mut
.. autocfunction:: zenoh_commons.h::z_config_check
.. autocfunction:: zenoh_commons.h::z_config_drop

Session management
------------------

Types
^^^^^

.. autocstruct:: zenoh_concrete.h::z_loaned_session_t
.. autocstruct:: zenoh_concrete.h::z_owned_session_t

.. autocstruct:: zenoh_commons.h::z_owned_closure_zid_t

Functions
^^^^^^^^^

.. autocfunction:: zenoh_commons.h::z_open
.. autocfunction:: zenoh_commons.h::z_close

.. autocfunction:: zenoh_commons.h::z_session_loan
.. autocfunction:: zenoh_commons.h::z_session_check

.. autocfunction:: zenoh_commons.h::z_info_zid
.. autocfunction:: zenoh_commons.h::z_info_routers_zid
.. autocfunction:: zenoh_commons.h::z_info_peers_zid

.. autocfunction:: zenoh_commons.h::z_closure_zid_call
.. autocfunction:: zenoh_commons.h::z_closure_zid_drop


Key expression
==============

.. autocstruct:: zenoh_commons.h::z_view_keyexpr_t
.. autocstruct:: zenoh_commons.h::z_loaned_keyexpr_t
.. autocstruct:: zenoh_commons.h::z_owned_keyexpr_t

.. autocfunction:: zenoh_commons.h::z_keyexpr_new
.. autocfunction:: zenoh_commons.h::z_view_keyexpr_new
.. autocfunction:: zenoh_commons.h::z_keyexpr_new_autocanonize
.. autocfunction:: zenoh_commons.h::z_view_keyexpr_new_autocanonize
.. autocfunction:: zenoh_commons.h::z_view_keyexpr_unchecked
.. autocfunction:: zenoh_commons.h::z_keyexpr_loan
.. autocfunction:: zenoh_commons.h::z_view_keyexpr_loan
.. autocfunction:: zenoh_commons.h::z_keyexpr_check
.. autocfunction:: zenoh_commons.h::z_view_keyexpr_check
.. autocfunction:: zenoh_commons.h::z_keyexpr_drop
.. autocfunction:: zenoh_commons.h::z_keyexpr_to_string
.. autocfunction:: zenoh_commons.h::z_keyexpr_as_slice
.. autocfunction:: zenoh_commons.h::z_keyexpr_canonize
.. autocfunction:: zenoh_commons.h::z_keyexpr_canonize_null_terminated
.. autocfunction:: zenoh_commons.h::z_keyexpr_is_canon
.. autocfunction:: zenoh_commons.h::z_keyexpr_concat
.. autocfunction:: zenoh_commons.h::z_keyexpr_join
.. autocfunction:: zenoh_commons.h::z_keyexpr_equals
.. autocfunction:: zenoh_commons.h::z_keyexpr_includes
.. autocfunction:: zenoh_commons.h::z_keyexpr_intersects




.. autocfunction:: zenoh_commons.h::z_declare_keyexpr
.. autocfunction:: zenoh_commons.h::z_undeclare_declare_keyexpr

Encoding
========

.. autocstruct:: zenoh_commons.h::z_loaned_encoding_t
.. autocstruct:: zenoh_commons.h::z_owned_encoding_t

.. autocfunction:: zenoh_commons.h::z_encoding_default

.. autocfunction:: zenoh_commons.h::z_encoding_loan
.. autocfunction:: zenoh_commons.h::z_encoding_check
.. autocfunction:: zenoh_commons.h::z_encoding_drop

.. autocstruct:: zenoh_commons.h::z_encoding_prefix_t

Value
=====

.. autocstruct:: zenoh_commons.h::z_loaned_value_t
.. autocstruct:: zenoh_commons.h::z_owned_value_t

Sample
======

.. autocstruct:: zenoh_commons.h::z_loaned_sample_t
.. autocstruct:: zenoh_commons.h::z_owned_sample_t

Publication
===========

Types
-----
.. autocstruct:: zenoh_commons.h::z_loaned_publisher_t
.. autocstruct:: zenoh_commons.h::z_owned_publisher_t

.. autocstruct:: zenoh_commons.h::z_congestion_control_t
.. autocstruct:: zenoh_commons.h::z_priority_t

.. autocstruct:: zenoh_commons.h::z_put_options_t
.. autocfunction:: zenoh_commons.h::z_put_options_default

.. autocstruct:: zenoh_commons.h::z_publisher_options_t
.. autocfunction:: zenoh_commons.h::z_publisher_options_default

.. autocstruct:: zenoh_commons.h::z_publisher_put_options_t

Functions
---------

.. autocfunction:: zenoh_commons.h::
.. autocfunction:: zenoh_commons.h::z_delete

.. autocfunction:: zenoh_commons.h::z_declare_publisher
.. autocfunction:: zenoh_commons.h::z_publisher_put
.. autocfunction:: zenoh_commons.h::z_publisher_delete
.. autocfunction:: zenoh_commons.h::z_undeclare_publisher

Subscription
============

Types
-----

.. autocstruct:: zenoh_concrete.h::z_loaned_subscriber_t
.. autocstruct:: zenoh_concrete.h::z_owned_subscriber_t

.. autocstruct:: zenoh_commons.h::z_owned_closure_sample_t

.. autocenum:: zenoh_commons.h::z_reliability_t

.. autocstruct:: zenoh_commons.h::z_subscriber_options_t
.. autocfunction:: zenoh_commons.h::z_subscriber_options_default

Functions
---------

.. autocfunction:: zenoh_commons.h::z_declare_subscriber
.. autocfunction:: zenoh_commons.h::z_subscriber_check
.. autocfunction:: zenoh_commons.h::z_undeclare_subscriber

.. autocfunction:: zenoh_commons.h::z_closure_sample_call
.. autocfunction:: zenoh_commons.h::z_closure_sample_drop

Query
=====

Types
-----

.. autocstruct:: zenoh_commons.h::z_owned_closure_reply_t

.. autocstruct:: zenoh_commons.h::z_get_options_t

.. autocenum:: zenoh_commons.h::z_query_target_t

.. autocenum:: zenoh_commons.h::z_consolidation_mode_t

.. c:type:: z_query_consolidation_t

   The replies consolidation strategy to apply on replies to a :c:func:`z_get`.

    - **AUTO**: Automatic query consolidation strategy selection.
    - **MANUAL**: Manual query consolidation strategy selection.

.. autocfunction:: zenoh_commons.h::z_query_consolidation_default
.. autocfunction:: zenoh_commons.h::z_query_consolidation_auto
.. autocfunction:: zenoh_commons.h::z_query_consolidation_none
.. autocfunction:: zenoh_commons.h::z_query_consolidation_monotonic
.. autocfunction:: zenoh_commons.h::z_query_consolidation_latest

.. autocstruct:: zenoh_commons.h::z_owned_reply_t
.. autocfunction:: zenoh_commons.h::z_reply_check
.. autocfunction:: zenoh_commons.h::z_reply_drop

Functions
---------

.. autocfunction:: zenoh_commons.h::z_get

.. autocfunction:: zenoh_commons.h::z_reply_is_ok
.. autocfunction:: zenoh_commons.h::z_reply_ok
.. autocfunction:: zenoh_commons.h::z_reply_err
.. autocfunction:: zenoh_commons.h::z_reply_null

.. autocfunction:: zenoh_commons.h::z_closure_reply_call
.. autocfunction:: zenoh_commons.h::z_closure_reply_drop

Queryable
=========

Types
-----

.. autocstruct:: zenoh_concrete.h::z_owned_queryable_t

.. autocstruct:: zenoh_commons.h::z_owned_closure_query_t

.. autocfunction:: zenoh_commons.h::z_query_keyexpr
.. autocfunction:: zenoh_commons.h::z_query_parameters
.. autocfunction:: zenoh_commons.h::z_query_value
.. autocfunction:: zenoh_commons.h::z_query_attachment

Functions
---------

.. autocfunction:: zenoh_commons.h::z_declare_queryable
.. autocfunction:: zenoh_commons.h::z_query_reply
.. autocfunction:: zenoh_commons.h::z_queryable_check
.. autocfunction:: zenoh_commons.h::z_undeclare_queryable

.. autocfunction:: zenoh_commons.h::z_closure_query_call
.. autocfunction:: zenoh_commons.h::z_closure_query_drop

Liveliness
==========

Types
-----

.. autocstruct:: zenoh_commons.h::zc_owned_liveliness_token_t
.. autocstruct:: zenoh_commons.h::zc_owned_liveliness_declaration_options_t
.. autocstruct:: zenoh_commons.h::zc_liveliness_get_options_t
.. autocstruct:: zenoh_commons.h::zc_owned_liveliness_declare_subscriber_options_t

Functions
---------

.. autocfunction:: zenoh_commons.h::zc_liveliness_declare_token
.. autocfunction:: zenoh_commons.h::zc_liveliness_undeclare_token
.. autocfunction:: zenoh_commons.h::zc_liveliness_token_null
.. autocfunction:: zenoh_commons.h::zc_liveliness_token_check
.. autocfunction:: zenoh_commons.h::zc_liveliness_declare_subscriber
.. autocfunction:: zenoh_commons.h::zc_liveliness_get
.. autocfunction:: zenoh_commons.h::zc_liveliness_get_options_default

Publication Cache
=================

Types
-----

.. autocstruct:: zenoh_commons.h::ze_publication_cache_options_t
.. autocstruct:: zenoh_commons.h::ze_owned_publication_cache_t

Functions
---------

.. autocfunction:: zenoh_commons.h::ze_declare_publication_cache
.. autocfunction:: zenoh_commons.h::ze_undeclare_publication_cache
.. autocfunction:: zenoh_commons.h::ze_publication_cache_check
.. autocfunction:: zenoh_commons.h::ze_publication_cache_null
.. autocfunction:: zenoh_commons.h::ze_publication_cache_options_default

Querying Subscriber
===================

Types
-----

.. autocstruct:: zenoh_commons.h::ze_owned_querying_subscriber_t
.. autocstruct:: zenoh_commons.h::ze_querying_subscriber_options_t

Functions
---------

.. autocfunction:: zenoh_commons.h::ze_declare_querying_subscriber
.. autocfunction:: zenoh_commons.h::ze_undeclare_querying_subscriber
.. autocfunction:: zenoh_commons.h::ze_querying_subscriber_get
.. autocfunction:: zenoh_commons.h::ze_querying_subscriber_check
.. autocfunction:: zenoh_commons.h::ze_querying_subscriber_null
.. autocfunction:: zenoh_commons.h::ze_querying_subscriber_options_default
