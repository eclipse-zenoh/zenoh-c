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

*************
API Reference
*************

Generic types
=============

String
------

.. c:type:: z_string_t

  A borrowed null-terminated string. (``const char*``).

.. autocstruct:: zenoh_commons.h::z_owned_string_t

.. autocfunction:: zenoh_commons.h::z_string_new
.. autocfunction:: zenoh_commons.h::z_string_borrow
.. autocfunction:: zenoh_commons.h::z_string_check
.. autocfunction:: zenoh_commons.h::z_string_free

Array of Str
------------

.. autocstruct:: zenoh_commons.h::z_owned_str_array_t

.. autocfunction:: zenoh_commons.h::z_str_array_check
.. autocfunction:: zenoh_commons.h::z_str_array_free

Bytes
-----

.. autocstruct:: zenoh_commons.h::z_bytes_t
.. autocstruct:: zenoh_commons.h::z_owned_bytes_t

.. autocfunction:: zenoh_commons.h::z_bytes_new
.. autocfunction:: zenoh_commons.h::z_bytes_borrow
.. autocfunction:: zenoh_commons.h::z_bytes_check
.. autocfunction:: zenoh_commons.h::z_bytes_free

Scouting
========

Types
-----

Possible flags in a whatami bitmask : 

  .. c:var:: const unsigned int ZN_ROUTER

  .. c:var:: const unsigned int ZN_PEER

  .. c:var:: const unsigned int ZN_CLIENT

.. autocstruct:: zenoh_commons.h::z_owned_hello_t

.. autocstruct:: zenoh_commons.h::z_owned_hello_array_t

Functions
---------

.. autocfunction:: zenoh_commons.h::z_scout
.. autocfunction:: zenoh_commons.h::z_hello_check
.. autocfunction:: zenoh_commons.h::z_hello_free
.. autocfunction:: zenoh_commons.h::z_hello_array_check
.. autocfunction:: zenoh_commons.h::z_hello_array_free

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
.. autocfunction:: zenoh_commons.h::z_config_set
.. autocfunction:: zenoh_commons.h::z_config_get
.. autocfunction:: zenoh_commons.h::z_config_len
.. autocfunction:: zenoh_commons.h::z_config_to_str
.. autocfunction:: zenoh_commons.h::z_config_borrow
.. autocfunction:: zenoh_commons.h::z_config_check
.. autocfunction:: zenoh_commons.h::z_config_free

The following constants define the several configuration keys accepted for a zenoh 
session configuration and the associated accepted values.

.. c:var:: const unsigned int ZN_CONFIG_MODE_KEY

  The library mode.

    - Accepted values : ``"peer"``, ``"client"``.
    - Default value : ``"peer"``.

.. c:var:: const unsigned int ZN_CONFIG_PEER_KEY

  The locator of a peer to connect to.
    - Accepted values : ``<locator>`` (ex: ``"tcp/10.10.10.10:7447"``).
    - Default value : None.
    - Multiple values accepted.

.. c:var:: const unsigned int ZN_CONFIG_LISTENER_KEY

  A locator to listen on.

    - Accepted values : ``<locator>`` (ex: ``"tcp/10.10.10.10:7447"``).
    - Default value : None.
    - Multiple values accepted.

.. c:var:: const unsigned int ZN_CONFIG_USER_KEY

  The user name to use for authentication.

    - Accepted values : ``<string>``.
    - Default value : None.

.. c:var:: const unsigned int ZN_CONFIG_PASSWORD_KEY

  The password to use for authentication.

    - Accepted values : ``<string>``.
    - Default value : None.


.. c:var:: const unsigned int ZN_CONFIG_MULTICAST_SCOUTING_KEY

  Activates/Desactivates multicast scouting.

    - Accepted values : ``"true"``, ``"false"``.
    - Default value : ``"true"``.

.. c:var:: const unsigned int ZN_CONFIG_MULTICAST_INTERFACE_KEY

  The network interface to use for multicast scouting.

    - Accepted values : ``"auto"``, ``<ip address>``, ``<interface name>``.
    - Default value : ``"auto"``.

.. c:var:: const unsigned int ZN_CONFIG_MULTICAST_ADDRESS_KEY

  The multicast address and ports to use for multicast scouting.

    - Accepted values : ``<ip address>:<port>``.
    - Default value : ``"224.0.0.224:7447"``.

.. c:var:: const unsigned int ZN_CONFIG_SCOUTING_TIMEOUT_KEY

  In client mode, the period dedicated to scouting a router before failing.

    - Accepted values : ``<float in seconds>``.
    - Default value : ``"3.0"``.

.. c:var:: const unsigned int ZN_CONFIG_SCOUTING_DELAY_KEY

  In peer mode, the period dedicated to scouting first remote peers before doing anything else.

    - Accepted values : ``<float in seconds>``.
    - Default value : ``"0.2"``.

.. c:var:: const unsigned int ZN_CONFIG_ADD_TIMESTAMP_KEY

  Indicates if data messages should be timestamped.

    - Accepted values : ``"true"``, ``"false"``.
    - Default value : ``"false"``.

.. c:var:: const unsigned int ZN_CONFIG_LOCAL_ROUTING_KEY

  Indicates if local writes/queries should reach local subscribers/queryables.

    - Accepted values : ``"true"``, ``"false"``.
    - Default value : ``"true"``.

Session management
------------------

Types
^^^^^

.. autocstruct:: zenoh_concrete.h::z_session_t
.. autocstruct:: zenoh_concrete.h::z_owned_session_t

Functions
^^^^^^^^^

.. autocfunction:: zenoh_commons.h::z_open
.. autocfunction:: zenoh_commons.h::z_info
.. autocfunction:: zenoh_commons.h::z_close

.. autocfunction:: zenoh_commons.h::z_session_borrow
.. autocfunction:: zenoh_commons.h::z_session_check

Key expression
==============

.. autocstruct:: zenoh_commons.h::z_keyexpr_t
.. autocstruct:: zenoh_commons.h::z_owned_keyexpr_t

.. autocfunction:: zenoh_commons.h::z_expr
.. autocfunction:: zenoh_commons.h::z_id
.. autocfunction:: zenoh_commons.h::z_id_with_suffix

.. autocfunction:: zenoh_commons.h::z_keyexpr_new
.. autocfunction:: zenoh_commons.h::z_keyexpr_new_borrowed
.. autocfunction:: zenoh_commons.h::z_keyexpr_borrow
.. autocfunction:: zenoh_commons.h::z_keyexpr_check
.. autocfunction:: zenoh_commons.h::z_keyexpr_free

.. autocfunction:: zenoh_commons.h::z_declare_expr

Sample
======

.. autocstruct:: zenoh_commons.h::z_sample_t
.. autocstruct:: zenoh_commons.h::z_owned_sample_t

.. autocfunction:: zenoh_commons.h::z_sample_borrow
.. autocfunction:: zenoh_commons.h::z_sample_check
.. autocfunction:: zenoh_commons.h::z_sample_free

Publication
===========

Types
-----

.. autocstruct:: zenoh_concrete.h::z_put_options_t
.. autocenum:: zenoh_commons.h::z_put_options_field_t

.. autocfunction:: zenoh_commons.h::z_put_options_default
.. autocfunction:: zenoh_commons.h::z_put_options_set

Functions
---------

.. autocfunction:: zenoh_commons.h::z_put
.. autocfunction:: zenoh_commons.h::z_put_ext

.. autocfunction:: zenoh_commons.h::z_declare_publication
.. autocfunction:: zenoh_commons.h::z_undeclare_publication

Subscription
============

Types
-----

.. autocstruct:: zenoh_concrete.h::z_owned_subscriber_t
.. autocenum:: zenoh_commons.h::z_reliability_t
.. autocenum:: zenoh_commons.h::z_submode_t
.. autocstruct:: zenoh_commons.h::z_period_t
.. autocstruct:: zenoh_commons.h::z_subinfo_t
.. autocfunction:: zenoh_commons.h::z_subinfo_default

Functions
---------

.. autocfunction:: zenoh_commons.h::z_subscribe
.. autocfunction:: zenoh_commons.h::z_pull
.. autocfunction:: zenoh_commons.h::z_subscriber_check
.. autocfunction:: zenoh_commons.h::z_subscriber_close

Query
=====

Types
-----

.. c:struct:: z_target_t

  Which amongst the matching queryables should be target of a :c:func:`get`.

  .. c:member:: z_target_t_Tag tag;

  .. c:member:: z_target_t_COMPLETE_Body complete;

    Members of z_target_t when :c:member:`z_target_t.tag` is set to ``z_target_t_COMPLETE``.

    .. c:member:: unsigned int n

      The number of complete queryables that should be target of a :c:func:`z_get`.

.. autocenum:: zenoh_commons.h::z_target_t_Tag

.. autocfunction:: zenoh_commons.h::z_target_default

.. autocstruct:: zenoh_commons.h::z_query_target_t

  Predefined values for :c:member:`z_query_target_t.kind`: 

    .. c:var:: const unsigned int ZN_QUERYABLE_ALL_KINDS

    .. c:var:: const unsigned int ZN_QUERYABLE_EVAL

    .. c:var:: const unsigned int ZN_QUERYABLE_STORAGE
  
.. autocfunction:: zenoh_commons.h::z_query_target_default

.. autocenum:: zenoh_commons.h::z_consolidation_mode_t
.. autocstruct:: zenoh_commons.h::z_query_consolidation_t
.. autocfunction:: zenoh_commons.h::z_query_consolidation_default

.. autocstruct:: zenoh_commons.h::z_owned_reply_data_t
.. autocfunction:: zenoh_commons.h::z_reply_data_check
.. autocfunction:: zenoh_commons.h::z_reply_data_free


.. autocstruct:: zenoh_commons.h::z_owned_reply_data_array_t
.. autocfunction:: zenoh_commons.h::z_reply_data_array_check
.. autocfunction:: zenoh_commons.h::z_reply_data_array_free

.. autocstruct:: zenoh_commons.h::z_owned_reply_t
.. autocenum:: zenoh_commons.h::z_reply_t_Tag
.. autocfunction:: zenoh_commons.h::z_reply_check
.. autocfunction:: zenoh_commons.h::z_reply_free

Functions
---------

.. autocfunction:: zenoh_commons.h::z_get

.. autocfunction:: zenoh_commons.h::z_get_collect

Queryable
=========

Types
-----

.. autocstruct:: zenoh_concrete.h::z_owned_queryable_t

.. c:type:: z_query_t

  A query received by a Queryable. 

.. autocfunction:: zenoh_commons.h::z_query_key_expr
.. autocfunction:: zenoh_commons.h::z_query_predicate


Functions
---------

.. autocfunction:: zenoh_commons.h::z_queryable_new

  Predefined values for ``kind``: 

    .. c:var:: const unsigned int ZN_QUERYABLE_EVAL

    .. c:var:: const unsigned int ZN_QUERYABLE_STORAGE

.. autocfunction:: zenoh_commons.h::z_send_reply
.. autocfunction:: zenoh_commons.h::z_queryable_check
.. autocfunction:: zenoh_commons.h::z_queryable_close


