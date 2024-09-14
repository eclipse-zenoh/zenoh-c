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
Concepts
********

Types classification
====================

There are following categories of types in the zenoh-c library:

- owned types `z_owned_xxx_t`
- loaned types `z_loaned_xxx_t`
- moved types `z_moved_xxx_t`
- view types `z_view_xxx_t`
- options structures `z_xxx_options_t`
- enums, plain data structures `z_xxx_t`

Owned types `z_owned_xxx_t`
---------------------------

Zenoh-c library uses in it's API some principles of Rust language, which the library is based on.
This includes the concept of ownership, moving and borrowing (loaning).

There are types in the zenoh-c which are named in format `z_owned_xxx_t`. Theese "owned" types once being
constructed may contain ("own") some external resources (like memory, file descriptors, etc.). On the end
of lifecycle of such type it should be destroyed by corresponding `z_xxx_drop` function or by generic
macro `z_drop`. Example:

.. code-block:: c

    z_owned_string_t s;
    z_string_copy_from_str(&s, "Hello, world!");
    //...
    z_drop(z_move(s));

There are two variants how the owned object may be passed to a function: by moving and by loaning. This is achieved
by types `z_loaned_xxx_t` and `z_moved_xxx_t`. 

Loaned types `z_loaned_xxx_t`
-----------------------------

When we need to temporarily pass the owned object to a function, we can do it by loaning it. This is done by
`z_xxx_loan` functions which accepts the owned object and returns the pointer to corresponding `z_loaned_xxx_t` type.
The generic macro `z_loan` is also provided for the purpose of code readability. 
The functions accepting a loaned object can either 
only read it (`const z_loaned_xxx_t*`) or read and modify (`z_loaned_xxx_t*`), but in any case caller keeps ownership
on the passed object. Example:

.. code-block:: c

    z_owned_string_t s, s1
    z_string_copy_from_str(&s, "Hello, world!");
    // notice that the prototype of z_string_clone is
    // void z_string_clone(z_owned_string_t* dst, const z_loaned_string_t* src);
    // I.e. the only way to pass the source string is by loaning it
    z_string_clone(&s1, z_loan(s));
    //...
    z_drop(z_move(s));
    z_drop(z_move(s1));

Moved types `z_moved_xxx_t`
---------------------------

When the function accepts the `z_moved_xxx_t*` parameter, this means that it takes the ownership of the passed object. 
To pass the owned object to such function the caller should use `z_xxx_move` function or generic macro `z_move`.

If the owned object is passed to some function by moving the caller should not use it anymore. It's still safe to call `z_drop` on it, 
but it's not necessary.  Notice also that the `z_drop` itself finally accepts ownership of owned structure, so it's 
also requizes `z_move` conversion. Example:

.. code-block:: c
    
    z_owned_config_t cfg;
    z_config_default(&cfg);
    z_owned_session_t session;
    // session takes ownership of the config
    if (z_open(&session, z_move(cfg)) == Z_OK) {
        //...
        z_drop(z_move(session));
    }
    // z_drop(z_move(cfg)); // this is safe but useless

View types `z_view_xxx_t`
-------------------------

`z_view_xxx_t` types are the types which only references some extra data. Values of these types need not to
be dropped and they are valid only while the data they reference is valid. Another important property of the view
types is that they are loaned to the same `z_loaned_xxx_t` type as their owned counterparts. This allows to use
both owned and view types in the same way. Example:

.. code-block:: c

    z_owned_string_t owned;
    z_string_copy_from_str(&owned, "Hello, world!");
    z_view_string_t view;
    z_view_string_from_str(&view, "Hello, another world!");
    z_owned_string_t dst;
    z_string_clone(&dst, z_loan(owned));
    z_drop(z_move(dst));
    z_string_clone(&dst, z_loan(view));
    z_drop(z_move(dst));

Options structures `z_xxx_options_t`
------------------------------------

The structures with `z_xxx_options_t` names are POD (Plain Old Data) structures which are used to pass multiple
parameters to the functions and achieve exencability of the API. These structures are nothing more than just a way to pass
multiple parameters to the function in a single argument. 
It's important to keep this logic in mind when "options" structure contains `z_moved_xxx_t*` fields. 
Assigning the `z_moved_xxx_t*` field with `z_move` optation itself doesn't do anything with the owned object. 
But passing this "options" structure to the function means ownership transfer of the owned object. Example:

.. code-block:: c

    // assume that we want to mark our message with some metadate of type int64_t
    z_publisher_put_options_t options;
    z_publisher_put_options_default(&options);
    int64_t metadata = 42;
    z_owned_bytes_t attachment;
    z_bytes_serialize_from_int64(&attachment, metadata);
    options.attachment = z_move(attachment); // the data itself is still in the `attachment`

    z_owned_bytes_t payload;
    z_bytes_serialize_from_str(&payload, "Don't panic!");
    z_publisher_put(z_loan(pub), z_move(payload), &options);
    // the `payload` and `attachment` are consumed by the `z_publisher_put` function


Enums, plain data structures `z_xxx_t`
--------------------------------------

There are also types named `z_xxx_t` which are copyable and can be passed by value. There is nothing special about them.
Examples of these types are `z_timestamp_t`, `z_priority_t`, etc.

.. code-block:: c

    z_timestamp_t ts;
    z_timestamp_new(&ts, z_loan(session))
    z_timestamp_t ts1 = ts;


Name prefixes `z_`, `zc_`, `ze_`
=======================================

Most of funcitons and types in the C API have names prefixed with `z_`. This prefix is used for 
the part of API which is common for all zenoh C API implementations (currently Rust based zenoh-c and 
pure C zenoh-pico).

There is also `zc_` prefix which is used for the functions and types which are specific for the zenoh-c. In zenoh-pico
there is similar `zp_` prefix is used for the same purpose.

`ze_` prefix marks the functions and types which wraps `zenoh-ext` Rust library. They are not a part of the core 
zenoh API so they are spearated into different namespace.