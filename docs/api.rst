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

*************
API Reference
*************

Generic types
=============

Bytes
-----

.. autocstruct:: zenoh_commons.h::z_bytes_t

.. autocfunction:: zenoh_commons.h::z_bytes_check

.. Scouting
.. ========

Session
=======

Session configuration
---------------------

.. autocstruct:: zenoh_concrete.h::z_config_t
.. autocstruct:: zenoh_concrete.h::z_owned_config_t

.. autocfunction:: zenoh_commons.h::z_config_new
.. autocfunction:: zenoh_commons.h::z_config_default
.. autocfunction:: zenoh_commons.h::z_config_empty
.. autocfunction:: zenoh_commons.h::z_config_client
.. autocfunction:: zenoh_commons.h::z_config_peer
.. autocfunction:: zenoh_commons.h::z_config_from_file
.. autocfunction:: zenoh_commons.h::z_config_from_str
.. autocfunction:: zenoh_commons.h::z_config_insert_json
.. autocfunction:: zenoh_commons.h::z_config_get
.. autocfunction:: zenoh_commons.h::z_config_to_string
.. autocfunction:: zenoh_commons.h::z_config_loan
.. autocfunction:: zenoh_commons.h::z_config_check
.. autocfunction:: zenoh_commons.h::z_config_drop

Session management
------------------

Types
^^^^^

.. autocstruct:: zenoh_concrete.h::z_session_t
.. autocstruct:: zenoh_concrete.h::z_owned_session_t

Functions
^^^^^^^^^

.. autocfunction:: zenoh_commons.h::z_open
.. autocfunction:: zenoh_commons.h::z_info_zid
.. autocfunction:: zenoh_commons.h::z_info_routers_zid
.. autocfunction:: zenoh_commons.h::z_info_peers_zid
.. autocfunction:: zenoh_commons.h::z_close

.. autocfunction:: zenoh_commons.h::z_session_loan
.. autocfunction:: zenoh_commons.h::z_session_check

Key expression
==============

.. autocstruct:: zenoh_commons.h::z_keyexpr_t
.. autocstruct:: zenoh_commons.h::z_owned_keyexpr_t

.. autocfunction:: zenoh_commons.h::z_keyexpr
.. autocfunction:: zenoh_commons.h::z_keyexpr_unchecked
.. autocfunction:: zenoh_commons.h::z_keyexpr_to_string
.. autocfunction:: zenoh_commons.h::z_keyexpr_as_bytes
.. autocfunction:: zenoh_commons.h::z_keyexpr_canonize
.. autocfunction:: zenoh_commons.h::z_keyexpr_canonize_null_terminated
.. autocfunction:: zenoh_commons.h::z_keyexpr_is_canon
.. autocfunction:: zenoh_commons.h::z_keyexpr_is_valid
.. autocfunction:: zenoh_commons.h::z_keyexpr_concat
.. autocfunction:: zenoh_commons.h::z_keyexpr_join
.. autocfunction:: zenoh_commons.h::z_keyexpr_equals
.. autocfunction:: zenoh_commons.h::z_keyexpr_includes
.. autocfunction:: zenoh_commons.h::z_keyexpr_intersects

.. autocfunction:: zenoh_commons.h::z_keyexpr_new
.. autocfunction:: zenoh_commons.h::z_keyexpr_loan
.. autocfunction:: zenoh_commons.h::z_keyexpr_check
.. autocfunction:: zenoh_commons.h::z_keyexpr_drop

Encoding
========

.. autocstruct:: zenoh_commons.h::z_encoding_t
.. autocstruct:: zenoh_commons.h::z_owned_encoding_t

.. autocfunction:: zenoh_commons.h::z_encoding_default

.. autocfunction:: zenoh_commons.h::z_encoding_loan
.. autocfunction:: zenoh_commons.h::z_encoding_check
.. autocfunction:: zenoh_commons.h::z_encoding_drop

.. autocstruct:: zenoh_commons.h::z_encoding_prefix_t

Value
=====

.. autocstruct:: zenoh_commons.h::z_value_t

Sample
======

.. autocstruct:: zenoh_commons.h::z_sample_t

.. Publication
.. ===========

.. Types
.. -----

.. .. autocstruct:: zenoh_commons.h::z_congestion_control_t
.. .. autocstruct:: zenoh_commons.h::z_priority_t

.. .. autocstruct:: zenoh_commons.h::z_put_options_t
.. .. autocenum:: zenoh_commons.h::z_put_options_field_t

.. Functions
.. ---------

.. .. autocfunction:: zenoh_commons.h::z_put
.. .. autocfunction:: zenoh_commons.h::z_put_ext

.. .. autocfunction:: zenoh_commons.h::z_declare_publication
.. .. autocfunction:: zenoh_commons.h::z_undeclare_publication

.. Subscription
.. ============

.. Types
.. -----

.. .. autocstruct:: zenoh_concrete.h::z_owned_subscriber_t
.. .. autocenum:: zenoh_commons.h::z_reliability
.. .. autocenum:: zenoh_commons.h::z_submode_t
.. .. autocstruct:: zenoh_commons.h::z_period_t
.. .. autocstruct:: zenoh_commons.h::z_subinfo_t
.. .. autocfunction:: zenoh_commons.h::z_subinfo_default
.. .. autocfunction:: zenoh_commons.h::z_subinfo_period

.. Functions
.. ---------

.. .. autocfunction:: zenoh_commons.h::z_subscribe
.. .. autocfunction:: zenoh_commons.h::z_pull
.. .. autocfunction:: zenoh_commons.h::z_subscriber_check
.. .. autocfunction:: zenoh_commons.h::z_subscriber_close

.. Query
.. =====

.. Types
.. -----

.. .. c:struct:: z_query_target_t

..   Which amongst the matching queryables should be target of a :c:func:`get`.

..   .. c:member:: z_query_target_t_Tag tag;

..   .. c:member:: z_query_target_t_COMPLETE_Body complete;

..     Members of z_query_target_t when :c:member:`z_query_target_t.tag` is set to ``z_target_t_COMPLETE``.

..     .. c:member:: unsigned int n

..       The number of complete queryables that should be target of a :c:func:`z_get`.

.. .. autocenum:: zenoh_commons.h::z_query_target_t_Tag

.. .. autocfunction:: zenoh_commons.h::z_query_target_default

.. .. autocenum:: zenoh_commons.h::z_consolidation_mode_t
.. .. autocstruct:: zenoh_commons.h::z_query_consolidation_t
.. .. autocfunction:: zenoh_commons.h::z_query_consolidation_default

.. .. autocstruct:: zenoh_commons.h::z_owned_reply_data_t
.. .. autocfunction:: zenoh_commons.h::z_reply_data_check
.. .. autocfunction:: zenoh_commons.h::z_reply_data_free


.. .. autocstruct:: zenoh_commons.h::z_owned_reply_data_array_t
.. .. autocfunction:: zenoh_commons.h::z_reply_data_array_check
.. .. autocfunction:: zenoh_commons.h::z_reply_data_array_free

.. .. autocstruct:: zenoh_commons.h::z_owned_reply_t
.. .. autocenum:: zenoh_commons.h::z_reply_t_Tag
.. .. autocfunction:: zenoh_commons.h::z_reply_check
.. .. autocfunction:: zenoh_commons.h::z_reply_free

.. Functions
.. ---------

.. .. autocfunction:: zenoh_commons.h::z_get

.. .. autocfunction:: zenoh_commons.h::z_get_collect

.. Queryable
.. =========

.. Types
.. -----

.. .. autocstruct:: zenoh_concrete.h::z_owned_queryable_t

.. .. c:type:: z_query_t

..   A query received by a Queryable. 

.. .. autocfunction:: zenoh_commons.h::z_query_key_expr
.. .. autocfunction:: zenoh_commons.h::z_query_predicate


.. Functions
.. ---------

.. .. autocfunction:: zenoh_commons.h::z_queryable_new
.. .. autocfunction:: zenoh_commons.h::z_send_reply
.. .. autocfunction:: zenoh_commons.h::z_queryable_check
.. .. autocfunction:: zenoh_commons.h::z_queryable_close


