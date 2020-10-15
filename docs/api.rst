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

.. autocstruct:: zenoh/net.h::zn_string_t

Bytes
-----

.. autocstruct:: zenoh/net.h::zn_bytes_t

Properties
----------

.. c:type:: zn_properties_t

  Data structure representing a set of id/value properties where the id is an unsigned int
  and the value a :c:type:`zn_bytes_t` bytes array. Several entries with the same id are allowed.

.. autocfunction:: zenoh/net.h::zn_properties_make

.. autocfunction:: zenoh/net.h::zn_properties_len

.. autocfunction:: zenoh/net.h::zn_properties_add

.. autocfunction:: zenoh/net.h::zn_property_id

.. autocfunction:: zenoh/net.h::zn_property_value

.. autocfunction:: zenoh/net.h::zn_properties_free

Scouting
========

.. c:var:: const unsigned int ZN_ROUTER

.. c:var:: const unsigned int ZN_PEER

.. c:var:: const unsigned int ZN_CLIENT

.. c:type:: zn_scout_t

  A set of scouted zenoh entities.

.. c:type:: zn_locators_t

  A set of zenoh locators.

.. autocfunction:: zenoh/net.h::zn_scout

.. autocfunction:: zenoh/net.h::zn_scout_len

.. autocfunction:: zenoh/net.h::zn_scout_whatami

.. autocfunction:: zenoh/net.h::zn_scout_peerid

.. autocfunction:: zenoh/net.h::zn_scout_peerid_len

.. autocfunction:: zenoh/net.h::zn_scout_locators

.. autocfunction:: zenoh/net.h::zn_scout_locators_len

.. autocfunction:: zenoh/net.h::zn_scout_locator_get

.. autocfunction:: zenoh/net.h::zn_scout_locators_free

.. autocfunction:: zenoh/net.h::zn_scout_free

Session
=======

Session configuration
---------------------

A zenoh-net session is configured through a set of :c:type:`zn_properties_t`.

Several properties with the same id are accepted. If only one property is 
expected for a given id, then the last property with the id is used.

The following constants define the several property ids accepted for a zenoh-net 
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

The following functions allow to create default sets of :c:type:`zn_properties_t` for 
zenoh-net session configuration. The returned configurations can be amended with extra 
options with :c:func:`zn_properties_add`.

.. autocfunction:: zenoh/net.h::zn_config_empty

.. autocfunction:: zenoh/net.h::zn_config_default

.. autocfunction:: zenoh/net.h::zn_config_peer

.. autocfunction:: zenoh/net.h::zn_config_client

Session management
------------------

.. autocfunction:: zenoh/net.h::zn_open

.. autocfunction:: zenoh/net.h::zn_info

.. autocfunction:: zenoh/net.h::zn_close

Resource
========

Resource key
------------

.. c:type:: zn_reskey_t

  A resource key.
  
  Resources are identified by URI like string names.  
  Examples : ``"/some/resource/key"``, ``"/a/selection/*/of/resources/**"``.
  Resource names can be mapped to numerical ids through :c:func:`zn_declare_resource` 
  for wire and computation efficiency.

  A resource key can be either:

    - a plain string resource name.
    - a pure numerical id.
    - the combination of a numerical prefix and a string suffix.

.. autocfunction:: zenoh/net.h::zn_rname

.. autocfunction:: zenoh/net.h::zn_rid

.. autocfunction:: zenoh/net.h::zn_rid_with_suffix

Sample
------

.. autocstruct:: zenoh/net.h::zn_sample_t

Resource declaration
--------------------

.. autocfunction:: zenoh/net.h::zn_declare_resource

Publication
===========

.. c:type:: zn_publisher_tr

  A zenoh-net Publisher.

.. autocfunction:: zenoh/net.h::zn_declare_publisher

.. autocfunction:: zenoh/net.h::zn_undeclare_publisher

.. autocfunction:: zenoh/net.h::zn_write

Subscription
============

.. c:type:: zn_subscriber_t

  A zenoh-net subscriber.

.. c:type:: zn_subinfo_t

  Informations to be passed to :c:func:`zn_declare_subscriber` to configure the created :c:type:`zn_subscriber_t`.

.. autocfunction:: zenoh/net.h::zn_subinfo_default

.. autocfunction:: zenoh/net.h::zn_subinfo_pull

.. autocfunction:: zenoh/net.h::zn_declare_subscriber

.. autocfunction:: zenoh/net.h::zn_pull

.. autocfunction:: zenoh/net.h::zn_undeclare_subscriber

Query
=====

.. c:struct:: zn_query_target_t

  The zenoh-net queryables that should be target of a :c:func:`zn_query`.

.. autocfunction:: zenoh/net.h::zn_query_target_default

.. c:struct:: zn_query_consolidation_t

  The kind of consolidation that should be applied on replies to a :c:func:`zn_query`.

.. autocfunction:: zenoh/net.h::zn_query_consolidation_default

.. autocstruct:: zenoh/net.h::zn_source_info_t

.. autocfunction:: zenoh/net.h::zn_query

Queryable
=========

.. c:type:: zn_queryable_t

  The zenoh-net Queryable.

.. autocfunction:: zenoh/net.h::zn_declare_queryable

.. autocfunction:: zenoh/net.h::zn_undeclare_queryable


