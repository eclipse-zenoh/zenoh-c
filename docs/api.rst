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
.. doxygenfunction:: z_slice_loan
.. doxygenfunction:: z_view_slice_loan
.. doxygenfunction:: z_slice_drop
.. doxygenfunction:: z_slice_clone

.. doxygenfunction:: z_slice_empty
.. doxygenfunction:: z_slice_copy_from_buf
.. doxygenfunction:: z_slice_from_buf
.. doxygenfunction:: z_view_slice_empty
.. doxygenfunction:: z_view_slice_from_buf
.. doxygenfunction:: z_slice_data
.. doxygenfunction:: z_slice_len
.. doxygenfunction:: z_slice_is_empty


String
------
Types
^^^^^
.. doxygenstruct:: z_owned_string_t
.. doxygenstruct:: z_view_string_t
.. doxygenstruct:: z_loaned_string_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_string_loan
.. doxygenfunction:: z_view_string_loan
.. doxygenfunction:: z_string_drop
.. doxygenfunction:: z_string_clone

.. doxygenfunction:: z_string_empty
.. doxygenfunction:: z_view_string_empty

.. doxygenfunction:: z_string_copy_from_str
.. doxygenfunction:: z_view_string_from_str
.. doxygenfunction:: z_string_copy_from_substr
.. doxygenfunction:: z_view_string_from_substr 
.. doxygenfunction:: z_string_from_str
.. doxygenfunction:: z_string_data
.. doxygenfunction:: z_string_len
.. doxygenfunction:: z_string_is_empty
.. doxygenfunction:: z_string_as_slice

String Array
------------
Types
^^^^^
.. doxygenstruct:: z_owned_string_array_t
.. doxygenstruct:: z_loaned_string_array_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_string_array_drop
.. doxygenfunction:: z_string_array_loan
.. doxygenfunction:: z_string_array_loan_mut
.. doxygenfunction:: z_string_array_clone

.. doxygenfunction:: z_string_array_new
.. doxygenfunction:: z_string_array_push_by_alias
.. doxygenfunction:: z_string_array_push_by_copy
.. doxygenfunction:: z_string_array_get
.. doxygenfunction:: z_string_array_len
.. doxygenfunction:: z_string_array_is_empty

Common
======

Key expression
--------------
Types
^^^^^
.. doxygenstruct:: z_owned_keyexpr_t
.. doxygenstruct:: z_view_keyexpr_t
.. doxygenstruct:: z_loaned_keyexpr_t
.. doxygenenum:: z_keyexpr_intersection_level_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_keyexpr_from_str
.. doxygenfunction:: z_view_keyexpr_from_str
.. doxygenfunction:: z_keyexpr_from_str_autocanonize
.. doxygenfunction:: z_view_keyexpr_from_str_autocanonize
.. doxygenfunction:: z_view_keyexpr_from_str_unchecked

.. doxygenfunction:: z_keyexpr_from_substr
.. doxygenfunction:: z_view_keyexpr_from_substr
.. doxygenfunction:: z_keyexpr_from_substr_autocanonize
.. doxygenfunction:: z_view_keyexpr_from_substr_autocanonize
.. doxygenfunction:: z_view_keyexpr_from_substr_unchecked

.. doxygenfunction:: z_keyexpr_loan
.. doxygenfunction:: z_view_keyexpr_loan
.. doxygenfunction:: z_keyexpr_clone
.. doxygenfunction:: z_keyexpr_drop

.. doxygenfunction:: z_keyexpr_as_view_string

.. doxygenfunction:: z_keyexpr_canonize
.. doxygenfunction:: z_keyexpr_canonize_null_terminated
.. doxygenfunction:: z_keyexpr_is_canon

.. doxygenfunction:: z_keyexpr_concat
.. doxygenfunction:: z_keyexpr_join
.. doxygenfunction:: z_keyexpr_equals
.. doxygenfunction:: z_keyexpr_includes
.. doxygenfunction:: z_keyexpr_intersects
.. doxygenfunction:: z_keyexpr_relation_to

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
.. doxygenfunction:: z_encoding_loan
.. doxygenfunction:: z_encoding_loan_mut
.. doxygenfunction:: z_encoding_drop
.. doxygenfunction:: z_encoding_loan_default
.. doxygenfunction:: z_encoding_from_str
.. doxygenfunction:: z_encoding_from_substr
.. doxygenfunction:: z_encoding_set_schema_from_str
.. doxygenfunction:: z_encoding_set_schema_from_substr
.. doxygenfunction:: z_encoding_to_string
.. doxygenfunction:: z_encoding_equals
.. doxygenfunction:: z_encoding_clone

Predefined Encodings
^^^^^^^^^^^^^^^^^^^^
.. doxygenfunction:: z_encoding_zenoh_bytes
.. doxygenfunction:: z_encoding_zenoh_string
.. doxygenfunction:: z_encoding_zenoh_serialized
.. doxygenfunction:: z_encoding_application_octet_stream
.. doxygenfunction:: z_encoding_text_plain
.. doxygenfunction:: z_encoding_application_json
.. doxygenfunction:: z_encoding_text_json
.. doxygenfunction:: z_encoding_application_cdr
.. doxygenfunction:: z_encoding_application_cbor
.. doxygenfunction:: z_encoding_application_yaml
.. doxygenfunction:: z_encoding_text_yaml
.. doxygenfunction:: z_encoding_text_json5
.. doxygenfunction:: z_encoding_application_python_serialized_object
.. doxygenfunction:: z_encoding_application_protobuf
.. doxygenfunction:: z_encoding_application_java_serialized_object
.. doxygenfunction:: z_encoding_application_openmetrics_text
.. doxygenfunction:: z_encoding_image_png
.. doxygenfunction:: z_encoding_image_jpeg
.. doxygenfunction:: z_encoding_image_gif
.. doxygenfunction:: z_encoding_image_bmp
.. doxygenfunction:: z_encoding_image_webp
.. doxygenfunction:: z_encoding_application_xml
.. doxygenfunction:: z_encoding_application_x_www_form_urlencoded
.. doxygenfunction:: z_encoding_text_html
.. doxygenfunction:: z_encoding_text_xml
.. doxygenfunction:: z_encoding_text_css
.. doxygenfunction:: z_encoding_text_javascript
.. doxygenfunction:: z_encoding_text_markdown
.. doxygenfunction:: z_encoding_text_csv
.. doxygenfunction:: z_encoding_application_sql
.. doxygenfunction:: z_encoding_application_coap_payload
.. doxygenfunction:: z_encoding_application_json_patch_json
.. doxygenfunction:: z_encoding_application_json_seq
.. doxygenfunction:: z_encoding_application_jsonpath
.. doxygenfunction:: z_encoding_application_jwt
.. doxygenfunction:: z_encoding_application_mp4
.. doxygenfunction:: z_encoding_application_soap_xml
.. doxygenfunction:: z_encoding_application_yang
.. doxygenfunction:: z_encoding_audio_aac
.. doxygenfunction:: z_encoding_audio_flac
.. doxygenfunction:: z_encoding_audio_mp4
.. doxygenfunction:: z_encoding_audio_ogg
.. doxygenfunction:: z_encoding_audio_vorbis
.. doxygenfunction:: z_encoding_video_h261
.. doxygenfunction:: z_encoding_video_h263
.. doxygenfunction:: z_encoding_video_h264
.. doxygenfunction:: z_encoding_video_h265
.. doxygenfunction:: z_encoding_video_h266
.. doxygenfunction:: z_encoding_video_mp4
.. doxygenfunction:: z_encoding_video_ogg
.. doxygenfunction:: z_encoding_video_raw
.. doxygenfunction:: z_encoding_video_vp8
.. doxygenfunction:: z_encoding_video_vp9

Reply Error
-----------
Types
^^^^^
.. doxygenstruct:: z_owned_reply_err_t
.. doxygenstruct:: z_loaned_reply_err_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_reply_err_payload
.. doxygenfunction:: z_reply_err_encoding

.. doxygenfunction:: z_reply_err_loan
.. doxygenfunction:: z_reply_err_clone
.. doxygenfunction:: z_reply_err_drop

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
.. doxygenfunction:: z_sample_drop
.. doxygenfunction:: z_sample_clone

.. doxygenfunction:: z_sample_timestamp
.. doxygenfunction:: z_sample_attachment
.. doxygenfunction:: z_sample_encoding
.. doxygenfunction:: z_sample_payload
.. doxygenfunction:: z_sample_priority
.. doxygenfunction:: z_sample_congestion_control
.. doxygenfunction:: z_sample_express
.. doxygenfunction:: z_sample_reliability
.. doxygenfunction:: z_sample_keyexpr
.. doxygenfunction:: z_sample_kind


Timestamp
---------
Types
^^^^^
.. doxygenstruct:: z_timestamp_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_timestamp_id
.. doxygenfunction:: z_timestamp_ntp64_time


Payload
-------
Types
^^^^^
.. doxygenstruct:: z_owned_bytes_t
.. doxygenstruct:: z_loaned_bytes_t
.. doxygenstruct:: z_bytes_reader_t
.. doxygenstruct:: z_owned_bytes_writer_t
.. doxygenstruct:: z_loaned_bytes_writer_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_bytes_len
.. doxygenfunction:: z_bytes_copy_from_slice
.. doxygenfunction:: z_bytes_from_slice
.. doxygenfunction:: z_bytes_copy_from_buf
.. doxygenfunction:: z_bytes_from_buf
.. doxygenfunction:: z_bytes_from_static_buf
.. doxygenfunction:: z_bytes_copy_from_string
.. doxygenfunction:: z_bytes_from_string
.. doxygenfunction:: z_bytes_copy_from_str
.. doxygenfunction:: z_bytes_from_str
.. doxygenfunction:: z_bytes_from_static_str
.. doxygenfunction:: z_bytes_to_slice
.. doxygenfunction:: z_bytes_to_string

.. doxygenfunction:: z_bytes_empty
.. doxygenfunction:: z_bytes_clone
.. doxygenfunction:: z_bytes_loan
.. doxygenfunction:: z_bytes_loan_mut
.. doxygenfunction:: z_bytes_drop

.. doxygenfunction:: z_bytes_get_reader
.. doxygenfunction:: z_bytes_reader_read
.. doxygenfunction:: z_bytes_reader_seek
.. doxygenfunction:: z_bytes_reader_tell
.. doxygenfunction:: z_bytes_reader_remaining

.. doxygenfunction:: z_bytes_writer_empty
.. doxygenfunction:: z_bytes_writer_finish
.. doxygenfunction:: z_bytes_writer_write_all
.. doxygenfunction:: z_bytes_writer_append

System
======

Random
------
Functions
^^^^^^^^^
.. doxygenfunction:: z_random_u8
.. doxygenfunction:: z_random_u16
.. doxygenfunction:: z_random_u32
.. doxygenfunction:: z_random_u64
.. doxygenfunction:: z_random_fill

Sleep
------
Functions
^^^^^^^^^
.. doxygenfunction:: z_sleep_s
.. doxygenfunction:: z_sleep_ms
.. doxygenfunction:: z_sleep_us

Time
----

Types
^^^^^
.. doxygenstruct:: z_clock_t
.. doxygenstruct:: z_time_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_clock_now
.. doxygenfunction:: z_clock_elapsed_s
.. doxygenfunction:: z_clock_elapsed_ms
.. doxygenfunction:: z_clock_elapsed_us

.. doxygenfunction:: z_time_now
.. doxygenfunction:: z_time_elapsed_s
.. doxygenfunction:: z_time_elapsed_ms
.. doxygenfunction:: z_time_elapsed_us
.. doxygenfunction:: z_time_now_as_str


Mutex
-----
Types
^^^^^
.. doxygenstruct:: z_owned_mutex_t
.. doxygenstruct:: z_loaned_mutex_t

Functions
^^^^^^^^^
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
.. doxygenfunction:: z_task_init
.. doxygenfunction:: z_task_drop
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
.. doxygenfunction:: z_config_loan
.. doxygenfunction:: z_config_loan_mut
.. doxygenfunction:: z_config_drop
.. doxygenfunction:: z_config_clone

.. doxygenfunction:: z_config_default
.. doxygenfunction:: zc_config_from_env
.. doxygenfunction:: zc_config_from_file
.. doxygenfunction:: zc_config_from_str
.. doxygenfunction:: zc_config_insert_json5
.. doxygenfunction:: zc_config_to_string

Session management
------------------

Types
^^^^^
.. doxygenstruct:: z_owned_session_t
.. doxygenstruct:: z_loaned_session_t
.. doxygenstruct:: z_id_t

.. doxygenstruct:: z_loaned_closure_zid_t
.. doxygenstruct:: z_owned_closure_zid_t

Functions
^^^^^^^^^
.. doxygenfunction:: z_open
.. doxygenfunction:: z_close
.. doxygenfunction:: z_session_is_closed

.. doxygenfunction:: z_session_loan
.. doxygenfunction:: z_session_loan_mut
.. doxygenfunction:: z_session_drop

.. doxygenfunction:: z_info_zid
.. doxygenfunction:: z_info_routers_zid
.. doxygenfunction:: z_info_peers_zid
.. doxygenfunction:: z_id_to_string

.. doxygenfunction:: z_closure_zid_drop
.. doxygenfunction:: z_closure_zid_loan
.. doxygenfunction:: z_closure_zid_call
.. doxygenfunction:: z_closure_zid

Matching
========

Types
-----
.. doxygenstruct:: zc_owned_matching_listener_t
.. doxygenstruct:: zc_owned_closure_matching_status_t
.. doxygenstruct:: zc_matching_status_t
    :members:

Functions
---------

.. doxygenfunction:: zc_matching_listener_drop
.. doxygenfunction:: zc_matching_listener_undeclare
.. doxygenfunction:: zc_closure_matching_status_drop
.. doxygenfunction:: zc_closure_matching_status_loan
.. doxygenfunction:: zc_closure_matching_status_call
.. doxygenfunction:: zc_closure_matching_status


Publication
===========

Types
-----

.. doxygenstruct:: z_owned_publisher_t
.. doxygenstruct:: z_loaned_publisher_t

.. doxygenenum:: z_congestion_control_t
.. doxygenenum:: z_priority_t
.. doxygenenum:: z_reliability_t

.. doxygenstruct:: z_put_options_t
    :members:
.. doxygenstruct:: z_delete_options_t
    :members:
.. doxygenstruct:: z_publisher_options_t
    :members:
.. doxygenstruct:: z_publisher_put_options_t
    :members:
.. doxygenstruct:: z_publisher_delete_options_t
    :members:

Functions
---------
.. doxygenfunction:: z_put
.. doxygenfunction:: z_delete

.. doxygenfunction:: z_declare_publisher
.. doxygenfunction:: z_undeclare_publisher
.. doxygenfunction:: z_publisher_put
.. doxygenfunction:: z_publisher_delete
.. doxygenfunction:: z_publisher_keyexpr
.. doxygenfunction:: z_publisher_id

.. doxygenfunction:: z_publisher_loan
.. doxygenfunction:: z_publisher_drop

.. doxygenfunction:: z_put_options_default
.. doxygenfunction:: z_delete_options_default
.. doxygenfunction:: z_publisher_options_default
.. doxygenfunction:: z_publisher_put_options_default
.. doxygenfunction:: z_publisher_delete_options_default

.. doxygenfunction:: z_reliability_default

.. doxygenfunction:: zc_publisher_get_matching_status
.. doxygenfunction:: zc_publisher_declare_matching_listener
.. doxygenfunction:: zc_publisher_declare_background_matching_listener

Subscription
============

Types
-----
.. doxygenstruct:: z_owned_subscriber_t
.. doxygenstruct:: z_loaned_subscriber_t

.. doxygenstruct:: z_loaned_closure_sample_t
.. doxygenstruct:: z_owned_closure_sample_t

.. doxygenstruct:: z_subscriber_options_t
    :members:

.. doxygenstruct:: z_owned_fifo_handler_sample_t
.. doxygenstruct:: z_loaned_fifo_handler_sample_t
.. doxygenstruct:: z_owned_ring_handler_sample_t
.. doxygenstruct:: z_loaned_ring_handler_sample_t

Functions
---------

.. doxygenfunction:: z_declare_subscriber
.. doxygenfunction:: z_undeclare_subscriber
.. doxygenfunction:: z_declare_background_subscriber
.. doxygenfunction:: z_subscriber_keyexpr

.. doxygenfunction:: z_subscriber_drop

.. doxygenfunction:: z_closure_sample_call
.. doxygenfunction:: z_closure_sample_loan
.. doxygenfunction:: z_closure_sample_drop
.. doxygenfunction:: z_closure_sample

.. doxygenfunction:: z_subscriber_options_default

.. doxygenfunction:: z_fifo_channel_sample_new
.. doxygenfunction:: z_ring_channel_sample_new

.. doxygenfunction:: z_fifo_handler_sample_drop
.. doxygenfunction:: z_fifo_handler_sample_loan
.. doxygenfunction:: z_fifo_handler_sample_recv
.. doxygenfunction:: z_fifo_handler_sample_try_recv

.. doxygenfunction:: z_ring_handler_sample_drop
.. doxygenfunction:: z_ring_handler_sample_loan
.. doxygenfunction:: z_ring_handler_sample_recv
.. doxygenfunction:: z_ring_handler_sample_try_recv

Queryable
=========

Types
-----

.. doxygenstruct:: z_owned_queryable_t
.. doxygenstruct:: z_loaned_queryable_t

.. doxygenstruct:: z_owned_query_t
.. doxygenstruct:: z_loaned_query_t

.. doxygenstruct:: z_loaned_closure_query_t
.. doxygenstruct:: z_owned_closure_query_t

.. doxygenstruct:: z_queryable_options_t
    :members:
.. doxygenstruct:: z_query_reply_options_t
    :members:
.. doxygenstruct:: z_query_reply_err_options_t
    :members:
.. doxygenstruct:: z_query_reply_del_options_t
    :members:

.. doxygenstruct:: z_owned_fifo_handler_query_t
.. doxygenstruct:: z_loaned_fifo_handler_query_t
.. doxygenstruct:: z_owned_ring_handler_query_t
.. doxygenstruct:: z_loaned_ring_handler_query_t

Functions
---------
.. doxygenfunction:: z_declare_queryable
.. doxygenfunction:: z_undeclare_queryable
.. doxygenfunction:: z_declare_background_queryable
.. doxygenfunction:: z_queryable_id

.. doxygenfunction:: z_queryable_options_default
.. doxygenfunction:: z_query_reply_options_default
.. doxygenfunction:: z_query_reply_err_options_default
.. doxygenfunction:: z_query_reply_del_options_default

.. doxygenfunction:: z_queryable_loan
.. doxygenfunction:: z_queryable_drop

.. doxygenfunction:: z_query_loan
.. doxygenfunction:: z_query_drop
.. doxygenfunction:: z_query_clone

.. doxygenfunction:: z_query_keyexpr
.. doxygenfunction:: z_query_parameters
.. doxygenfunction:: z_query_payload
.. doxygenfunction:: z_query_encoding
.. doxygenfunction:: z_query_attachment
.. doxygenfunction:: z_query_reply
.. doxygenfunction:: z_query_reply_err
.. doxygenfunction:: z_query_reply_del

.. doxygenfunction:: z_closure_query_call
.. doxygenfunction:: z_closure_query_loan
.. doxygenfunction:: z_closure_query_drop
.. doxygenfunction:: z_closure_query

.. doxygenfunction:: z_fifo_channel_query_new
.. doxygenfunction:: z_ring_channel_query_new

.. doxygenfunction:: z_fifo_handler_query_drop
.. doxygenfunction:: z_fifo_handler_query_loan
.. doxygenfunction:: z_fifo_handler_query_recv
.. doxygenfunction:: z_fifo_handler_query_try_recv

.. doxygenfunction:: z_ring_handler_query_drop
.. doxygenfunction:: z_ring_handler_query_loan
.. doxygenfunction:: z_ring_handler_query_recv
.. doxygenfunction:: z_ring_handler_query_try_recv

Query
=====

Types
-----
.. doxygenstruct:: z_owned_querier_t
.. doxygenstruct:: z_loaned_querier_t

.. doxygenstruct:: z_owned_reply_t
.. doxygenstruct:: z_loaned_reply_t

.. doxygenstruct:: z_loaned_closure_reply_t
.. doxygenstruct:: z_owned_closure_reply_t

.. doxygenstruct:: z_get_options_t
    :members:
.. doxygenenum:: z_query_target_t
.. doxygenenum:: z_consolidation_mode_t
.. doxygenenum:: zc_reply_keyexpr_t
.. doxygenstruct:: z_query_consolidation_t

.. doxygenstruct:: z_querier_options_t
    :members:
.. doxygenstruct:: z_querier_get_options_t
    :members:

.. doxygenstruct:: z_owned_fifo_handler_reply_t
.. doxygenstruct:: z_loaned_fifo_handler_reply_t
.. doxygenstruct:: z_owned_ring_handler_reply_t
.. doxygenstruct:: z_loaned_ring_handler_reply_t
    


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
.. doxygenfunction:: zc_reply_keyexpr_default

.. doxygenfunction:: z_declare_querier
.. doxygenfunction:: z_undeclare_querier
.. doxygenfunction:: z_querier_loan
.. doxygenfunction:: z_querier_drop
.. doxygenfunction:: z_declare_background_querier
.. doxygenfunction:: z_querier_id
.. doxygenfunction:: z_querier_keyexpr
.. doxygenfunction:: z_querier_get
.. doxygenfunction:: zc_querier_get_matching_status
.. doxygenfunction:: zc_querier_declare_matching_listener
.. doxygenfunction:: zc_querier_declare_background_matching_listener

.. doxygenfunction:: z_querier_options_default
.. doxygenfunction:: z_querier_get_options_default

.. doxygenfunction:: z_reply_drop
.. doxygenfunction:: z_reply_clone
.. doxygenfunction:: z_reply_loan
.. doxygenfunction:: z_reply_is_ok
.. doxygenfunction:: z_reply_ok
.. doxygenfunction:: z_reply_err

.. doxygenfunction:: z_closure_reply_call
.. doxygenfunction:: z_closure_reply_loan
.. doxygenfunction:: z_closure_reply_drop
.. doxygenfunction:: z_closure_reply

.. doxygenfunction:: z_fifo_channel_reply_new
.. doxygenfunction:: z_ring_channel_reply_new

.. doxygenfunction:: z_fifo_handler_reply_drop
.. doxygenfunction:: z_fifo_handler_reply_loan
.. doxygenfunction:: z_fifo_handler_reply_recv
.. doxygenfunction:: z_fifo_handler_reply_try_recv

.. doxygenfunction:: z_ring_handler_reply_drop
.. doxygenfunction:: z_ring_handler_reply_loan
.. doxygenfunction:: z_ring_handler_reply_recv
.. doxygenfunction:: z_ring_handler_reply_try_recv

Scouting
========

Types
-----
.. doxygenstruct:: z_owned_hello_t
.. doxygenstruct:: z_loaned_hello_t
.. doxygenstruct:: z_scout_options_t
    :members:
.. doxygenstruct:: z_loaned_closure_hello_t
.. doxygenstruct:: z_owned_closure_hello_t

Functions
---------
.. doxygenfunction:: z_scout

.. doxygenfunction:: z_hello_whatami
.. doxygenfunction:: z_hello_locators
.. doxygenfunction:: z_hello_zid
.. doxygenfunction:: z_hello_loan
.. doxygenfunction:: z_hello_clone
.. doxygenfunction:: z_hello_drop

.. doxygenfunction:: z_whatami_to_view_string

.. doxygenfunction:: z_scout_options_default

.. doxygenfunction:: z_closure_hello_call
.. doxygenfunction:: z_closure_hello_loan
.. doxygenfunction:: z_closure_hello_drop
.. doxygenfunction:: z_closure_hello

Liveliness
==========

Types
-----

.. doxygenstruct:: z_owned_liveliness_token_t
.. doxygenstruct:: z_liveliness_token_options_t
    :members:
.. doxygenstruct:: z_liveliness_get_options_t
    :members:
.. doxygenstruct:: z_liveliness_subscriber_options_t
    :members:

Functions
---------
.. doxygenfunction:: z_liveliness_declare_subscriber
.. doxygenfunction:: zc_liveliness_declare_background_subscriber
.. doxygenfunction:: z_liveliness_get

.. doxygenfunction:: z_liveliness_declare_token
.. doxygenfunction:: z_liveliness_undeclare_token
.. doxygenfunction:: z_liveliness_token_loan
.. doxygenfunction:: z_liveliness_token_drop

.. doxygenfunction:: z_liveliness_subscriber_options_default
.. doxygenfunction:: z_liveliness_token_options_default
.. doxygenfunction:: z_liveliness_get_options_default

Logging
=======

Types
-----

.. doxygenstruct:: zc_owned_closure_log_t
.. doxygenstruct:: zc_loaned_closure_log_t
.. doxygenenum:: zc_log_severity_t

Functions
---------

.. doxygenfunction:: zc_try_init_log_from_env
.. doxygenfunction:: zc_init_log_from_env_or
.. doxygenfunction:: zc_init_log_with_callback

.. doxygenfunction:: zc_closure_log_call
.. doxygenfunction:: zc_closure_log_loan
.. doxygenfunction:: zc_closure_log_drop
.. doxygenfunction:: zc_closure_log


Other
=====

Functions
---------
.. doxygenfunction:: zc_stop_z_runtime
.. doxygenfunction:: zc_cleanup_orphaned_shm_segments 

Ext
===

Serialization / Deserialization
-------------------------------
Types
^^^^^
.. doxygenstruct:: ze_owned_serializer_t
.. doxygenstruct:: ze_loaned_serializer_t
.. doxygenstruct:: ze_deserializer_t

Functions
^^^^^^^^^
.. doxygenfunction:: ze_serialize_slice
.. doxygenfunction:: ze_serialize_buf
.. doxygenfunction:: ze_serialize_string
.. doxygenfunction:: ze_serialize_str
.. doxygenfunction:: ze_serialize_substr
.. doxygenfunction:: ze_serialize_uint8
.. doxygenfunction:: ze_serialize_uint16
.. doxygenfunction:: ze_serialize_uint32
.. doxygenfunction:: ze_serialize_uint64
.. doxygenfunction:: ze_serialize_int8
.. doxygenfunction:: ze_serialize_int16
.. doxygenfunction:: ze_serialize_int32
.. doxygenfunction:: ze_serialize_int64
.. doxygenfunction:: ze_serialize_float
.. doxygenfunction:: ze_serialize_double
.. doxygenfunction:: ze_serialize_bool

.. doxygenfunction:: ze_deserialize_slice
.. doxygenfunction:: ze_deserialize_string
.. doxygenfunction:: ze_deserialize_uint8
.. doxygenfunction:: ze_deserialize_uint16
.. doxygenfunction:: ze_deserialize_uint32
.. doxygenfunction:: ze_deserialize_uint64
.. doxygenfunction:: ze_deserialize_int8
.. doxygenfunction:: ze_deserialize_int16
.. doxygenfunction:: ze_deserialize_int32
.. doxygenfunction:: ze_deserialize_int64
.. doxygenfunction:: ze_deserialize_float
.. doxygenfunction:: ze_deserialize_double
.. doxygenfunction:: ze_deserialize_bool

.. doxygenfunction:: ze_serializer_empty
.. doxygenfunction:: ze_serializer_finish
.. doxygenfunction:: ze_serializer_serialize_slice
.. doxygenfunction:: ze_serializer_serialize_buf
.. doxygenfunction:: ze_serializer_serialize_string
.. doxygenfunction:: ze_serializer_serialize_str
.. doxygenfunction:: ze_serializer_serialize_substr
.. doxygenfunction:: ze_serializer_serialize_uint8
.. doxygenfunction:: ze_serializer_serialize_uint16
.. doxygenfunction:: ze_serializer_serialize_uint32
.. doxygenfunction:: ze_serializer_serialize_uint64
.. doxygenfunction:: ze_serializer_serialize_int8
.. doxygenfunction:: ze_serializer_serialize_int16
.. doxygenfunction:: ze_serializer_serialize_int32
.. doxygenfunction:: ze_serializer_serialize_int64
.. doxygenfunction:: ze_serializer_serialize_float
.. doxygenfunction:: ze_serializer_serialize_double
.. doxygenfunction:: ze_serializer_serialize_bool
.. doxygenfunction:: ze_serializer_serialize_sequence_length

.. doxygenfunction:: ze_deserializer_from_bytes
.. doxygenfunction:: ze_deserializer_is_done
.. doxygenfunction:: ze_deserializer_deserialize_slice
.. doxygenfunction:: ze_deserializer_deserialize_string
.. doxygenfunction:: ze_deserializer_deserialize_uint8
.. doxygenfunction:: ze_deserializer_deserialize_uint16
.. doxygenfunction:: ze_deserializer_deserialize_uint32
.. doxygenfunction:: ze_deserializer_deserialize_uint64
.. doxygenfunction:: ze_deserializer_deserialize_int8
.. doxygenfunction:: ze_deserializer_deserialize_int16
.. doxygenfunction:: ze_deserializer_deserialize_int32
.. doxygenfunction:: ze_deserializer_deserialize_int64
.. doxygenfunction:: ze_deserializer_deserialize_float
.. doxygenfunction:: ze_deserializer_deserialize_double
.. doxygenfunction:: ze_deserializer_deserialize_bool
.. doxygenfunction:: ze_deserializer_deserialize_sequence_length

Advanced Publisher
------------------

Types
^^^^^

.. doxygenstruct:: ze_owned_advanced_publisher_t
.. doxygenstruct:: ze_loaned_advanced_publisher_t

.. doxygenstruct:: ze_advanced_publisher_cache_options_t
    :members:
.. doxygenstruct:: ze_advanced_publisher_options_t
    :members:
.. doxygenstruct:: ze_advanced_publisher_put_options_t
    :members:
.. doxygenstruct:: ze_advanced_publisher_delete_options_t
    :members:

Functions
^^^^^^^^^
.. doxygenfunction:: ze_declare_advanced_publisher
.. doxygenfunction:: ze_undeclare_advanced_publisher
.. doxygenfunction:: ze_advanced_publisher_put
.. doxygenfunction:: ze_advanced_publisher_delete
.. doxygenfunction:: ze_advanced_publisher_keyexpr
.. doxygenfunction:: ze_advanced_publisher_id

.. doxygenfunction:: ze_advanced_publisher_loan
.. doxygenfunction:: ze_advanced_publisher_drop

.. doxygenfunction:: ze_advanced_publisher_options_default
.. doxygenfunction:: ze_advanced_publisher_cache_options_default
.. doxygenfunction:: ze_advanced_publisher_put_options_default
.. doxygenfunction:: ze_advanced_publisher_delete_options_default

.. doxygenfunction:: ze_advanced_publisher_get_matching_status
.. doxygenfunction:: ze_advanced_publisher_declare_matching_listener
.. doxygenfunction:: ze_advanced_publisher_declare_background_matching_listener

Advanced Subscriber
-------------------

Types
^^^^^
.. doxygenstruct:: ze_owned_advanced_subscriber_t
.. doxygenstruct:: ze_loaned_advanced_subscriber_t

.. doxygenstruct:: ze_advanced_subscriber_history_options_t
    :members:
.. doxygenstruct:: ze_advanced_subscriber_recovery_options_t
    :members:
.. doxygenstruct:: ze_advanced_subscriber_options_t
    :members:

Functions
^^^^^^^^^

.. doxygenfunction:: ze_declare_advanced_subscriber
.. doxygenfunction:: ze_declare_background_advanced_subscriber
.. doxygenfunction:: ze_undeclare_advanced_subscriber

.. doxygenfunction:: ze_advanced_subscriber_detect_publishers
.. doxygenfunction:: ze_advanced_subscriber_detect_publishers_background

.. doxygenfunction:: ze_advanced_subscriber_drop

.. doxygenfunction:: ze_advanced_subscriber_history_options_default
.. doxygenfunction:: ze_advanced_subscriber_recovery_options_default
.. doxygenfunction:: ze_advanced_subscriber_options_default

Publication Cache (deprecated)
------------------------------

Types
^^^^^

.. doxygenstruct:: ze_owned_publication_cache_t
.. doxygenstruct:: ze_loaned_publication_cache_t
.. doxygenstruct:: ze_publication_cache_options_t
    :members:
.. doxygenenum:: zc_locality_t

Functions
^^^^^^^^^

.. doxygenfunction:: ze_declare_publication_cache
.. doxygenfunction:: ze_undeclare_publication_cache
.. doxygenfunction:: ze_declare_background_publication_cache

.. doxygenfunction:: ze_publication_cache_drop
.. doxygenfunction:: ze_publication_cache_loan
.. doxygenfunction:: ze_publication_cache_keyexpr

.. doxygenfunction:: ze_publication_cache_options_default

Querying Subscriber (deprecated)
--------------------------------

Types
^^^^^

.. doxygenstruct:: ze_owned_querying_subscriber_t
.. doxygenstruct:: ze_loaned_querying_subscriber_t
.. doxygenstruct:: ze_querying_subscriber_options_t
    :members:

Functions
^^^^^^^^^

.. doxygenfunction:: ze_declare_querying_subscriber
.. doxygenfunction:: ze_undeclare_querying_subscriber
.. doxygenfunction:: ze_declare_background_querying_subscriber
.. doxygenfunction:: ze_querying_subscriber_get

.. doxygenfunction:: ze_querying_subscriber_drop

.. doxygenfunction:: ze_querying_subscriber_options_default
