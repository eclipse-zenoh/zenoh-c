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

Types
=====

String
------

.. autocstruct:: zenoh/net.h::z_string_t

.. autocfunction:: zenoh/net.h::z_string_make


Array of Str
------------

.. autocstruct:: zenoh/net.h::z_str_array_t

Bytes
-----

.. autocstruct:: zenoh/net.h::z_bytes_t

Properties
----------

.. c:type:: z_properties_t

  A map of key/value properties where the key is an ``unsigned int``
  and the value a :c:type:`z_string_t`. Multiple values are coma separated.

.. autocfunction:: zenoh/net.h::z_properties_make

.. autocfunction:: zenoh/net.h::z_properties_len

.. autocfunction:: zenoh/net.h::z_properties_insert

.. autocfunction:: zenoh/net.h::z_properties_get

.. autocfunction:: zenoh/net.h::z_properties_free

Scouting
========

Types
-----

Possible flags in a whatami bitmask : 

  .. c:var:: const unsigned int ZN_ROUTER

  .. c:var:: const unsigned int ZN_PEER

  .. c:var:: const unsigned int ZN_CLIENT

.. autocstruct:: zenoh/net.h::z_hello_t

.. autocstruct:: zenoh/net.h::z_hello_array_t

Functions
---------

.. autocfunction:: zenoh/net.h::z_scout

.. autocfunction:: zenoh/net.h::z_hello_array_free

Session
=======

Session configuration
---------------------

A zenoh session is configured through a :c:type:`z_properties_t` properties map.

Multiple values are coma separated.

The following constants define the several property keys accepted for a zenoh 
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

The following functions allow to create default :c:type:`z_properties_t` maps for 
zenoh session configuration. The returned configurations can be amended with extra 
options with :c:func:`z_properties_insert`.

.. autocfunction:: zenoh/net.h::z_config_empty

.. autocfunction:: zenoh/net.h::z_config_default

.. autocfunction:: zenoh/net.h::z_config_peer

.. autocfunction:: zenoh/net.h::z_config_client

Session management
------------------

.. autocfunction:: zenoh/net.h::z_open

.. autocfunction:: zenoh/net.h::z_info

.. autocfunction:: zenoh/net.h::z_close

Resource
========

Resource key
------------

.. autocstruct:: zenoh/net.h::z_keyexpr_t

.. autocfunction:: zenoh/net.h::z_expr

.. autocfunction:: zenoh/net.h::z_id

.. autocfunction:: zenoh/net.h::z_id_with_suffix

Sample
------

.. autocstruct:: zenoh/net.h::z_sample_t

.. autocfunction:: zenoh/net.h::z_sample_free

Resource declaration
--------------------

.. autocfunction:: zenoh/net.h::z_declare_resource

Publication
===========

Types
-----

.. c:type:: z_publisher_tr

  A zenoh Publisher.

.. autocenum:: zenoh/net.h::z_congestion_control_t

Functions
---------

.. autocfunction:: zenoh/net.h::z_declare_publisher

.. autocfunction:: zenoh/net.h::z_undeclare_publisher

.. autocfunction:: zenoh/net.h::z_write

.. autocfunction:: zenoh/net.h::z_write_ext

Subscription
============

Types
-----

.. c:type:: z_subscriber_t

  A zenoh subscriber.

.. autocenum:: zenoh/net.h::z_reliability_t

.. autocenum:: zenoh/net.h::z_submode_t

.. autocstruct:: zenoh/net.h::z_period_t

.. autocstruct:: zenoh/net.h::z_subinfo_t

.. autocfunction:: zenoh/net.h::z_subinfo_default

Functions
---------

.. autocfunction:: zenoh/net.h::z_declare_subscriber

.. autocfunction:: zenoh/net.h::z_pull

.. autocfunction:: zenoh/net.h::z_undeclare_subscriber

Query
=====

Types
-----

.. c:struct:: z_target_t

  Which amongst the matching queryables should be target of a :c:func:`z_query`.

  .. c:member:: z_target_t_Tag tag;

  .. c:member:: z_target_t_COMPLETE_Body complete;

    Members of z_target_t when :c:member:`z_target_t.tag` is set to ``z_target_t_COMPLETE``.

    .. c:member:: unsigned int n

      The number of complete queryables that should be target of a :c:func:`z_query`.

.. autocenum:: zenoh/net.h::z_target_t_Tag

.. autocfunction:: zenoh/net.h::z_target_default

.. autocstruct:: zenoh/net.h::z_query_target_t

  Predefined values for :c:member:`z_query_target_t.kind`: 

    .. c:var:: const unsigned int ZN_QUERYABLE_ALL_KINDS

    .. c:var:: const unsigned int ZN_QUERYABLE_EVAL

    .. c:var:: const unsigned int ZN_QUERYABLE_STORAGE

.. autocfunction:: zenoh/net.h::z_query_target_default

.. autocenum:: zenoh/net.h::z_consolidation_mode_t

.. autocstruct:: zenoh/net.h::z_query_consolidation_t

.. autocfunction:: zenoh/net.h::z_query_consolidation_default

.. autocstruct:: zenoh/net.h::z_reply_data_t

.. autocfunction:: zenoh/net.h::z_reply_data_free

.. autocstruct:: zenoh/net.h::z_reply_data_array_t

.. autocfunction:: zenoh/net.h::z_reply_data_array_free

.. autocstruct:: zenoh/net.h::z_reply_t

.. autocenum:: zenoh/net.h::z_reply_t_Tag

Functions
---------

.. autocfunction:: zenoh/net.h::z_query

.. autocfunction:: zenoh/net.h::z_query_collect

Queryable
=========

Types
-----

.. c:type:: z_queryable_t

  The zenoh Queryable.

.. c:type:: z_query_t

  A query received by a Queryable. 

.. autocfunction:: zenoh/net.h::z_query_res_name

.. autocfunction:: zenoh/net.h::z_query_predicate


Functions
---------

.. autocfunction:: zenoh/net.h::z_declare_queryable

  Predefined values for ``kind``: 

    .. c:var:: const unsigned int ZN_QUERYABLE_EVAL

    .. c:var:: const unsigned int ZN_QUERYABLE_STORAGE

.. autocfunction:: zenoh/net.h::z_send_reply

.. autocfunction:: zenoh/net.h::z_undeclare_queryable


