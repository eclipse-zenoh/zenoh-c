..
.. Copyright (c) 2024 ZettaScale Technology
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

Types Classification
====================

Zenoh-C types fall into these categories:

- Owned types: `z_owned_xxx_t`
- Loaned types: `z_loaned_xxx_t`
- Moved types: `z_moved_xxx_t`
- View types: `z_view_xxx_t`
- Option structures: `z_xxx_options_t`
- Enums and plain data structures: `z_xxx_t`

Owned Types `z_owned_xxx_t`
---------------------------

The Zenoh-C library incorporates concepts like ownership, moving, and borrowing.

Types prefixed with `z_owned_xxx_t` "own" external resources (e.g., memory, file descriptors). 
These types must be destroyed at the end of their lifecycle using the `z_xxx_drop` function or 
the `z_drop` macro. Example:

.. code-block:: c

    z_owned_string_t s;
    z_string_copy_from_str(&s, "Hello, world!");
    //...
    z_drop(z_move(s));

Owned objects can be passed to functions in two ways: by moving (`z_moved_xxx_t`) or 
loaning (`z_loaned_xxx_t`).

Loaned Types `z_loaned_xxx_t`
-----------------------------

To temporarily pass an owned object, it can be loaned using `z_xxx_loan` functions, which return 
a pointer to the corresponding `z_loaned_xxx_t`. For readability, the generic macro `z_loan` is also available.

Functions accepting a loaned object can either read (`const z_loaned_xxx_t*`) or read and 
modify (`z_loaned_xxx_t*`) the object. In both cases, ownership remains with the caller. Example:

.. code-block:: c

    z_owned_string_t s, s1;
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

When a function accepts a `z_moved_xxx_t*` parameter, it takes ownership of the passed object. 
To pass the object, use the `z_xxx_move` function or the `z_move` macro.

Once the object is moved, the caller should no longer use it. While calling `z_drop` is safe, 
it's not required. Note that `z_drop` itself takes ownership, so `z_move` is also needed in this case. Example:

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

View Types `z_view_xxx_t`
-------------------------

`z_view_xxx_t` types are reference types that point to external data. These values do not need to be dropped and 
remain valid only as long as the data they reference is valid. 

A key feature is that `z_view_xxx_t` types are loaned as `z_loaned_xxx_t`, just like their owned counterparts, 
allowing consistent use of both owned and view types. Example:

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

Options Structures `z_xxx_options_t`
------------------------------------

`z_xxx_options_t` are Plain Old Data (POD) structures used to pass multiple parameters to functions. This makes API 
compact and allows to extend the API keeping backward compatibility.

Note that when an "options" structure contains `z_moved_xxx_t*` fields, assigning `z_move` to this field does not 
affect the owned object. However, passing the structure to a function transfers ownership of the object. Example:

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


Enums and Plain Data Structures `z_xxx_t`
-----------------------------------------

Types named `z_xxx_t` are simple, copyable, and can be passed by value. They do not have special handling. 
Examples include `z_timestamp_t`, `z_priority_t`, etc.

.. code-block:: c

    z_timestamp_t ts;
    z_timestamp_new(&ts, z_loan(session));
    z_timestamp_t ts1 = ts;

Common operations
=================

The transition between "owned", "loaned" and "moved" structures above is performed by corresponding functions.
The following operations are available: move, loan, mutable loan, take, check, and drop. They are performed for 
"xxx" entity  by functions `z_xxx_move`, `z_xxx_loan`, `z_xxx_loan_mut`, `z_xxx_take_moved`, `z_xxx_take_loaned`,
`z_xxx_check`, and `z_xxx_drop`.
The generic macros `z_move`, `z_loan`, `z_loan_mut`, `z_take`, `z_check`, and `z_drop` are also provided.

Loan operation
--------------

Function `z_xxx_loan` accepts `const z_owned_xxx_t*` and returns pointer `const z_loaned_xxx_t*` which gives read only 
access to `z_owned_xxx_t` entity.

`z_loan` macro accepts variable of `z_owned_xxx_t` type and calls corresponding `z_xxx_loan` function.

Mutable loan operation
----------------------

Function `z_xxx_loan_mut` accepts `z_owned_xxx_t*` and
returns pointer `z_xxx_loaned_t*` which allows to
read and modify `z_owned_xxx_t` entity and if supported by the type take ownership on it (see "take" operation)

`z_loan_mut` macro accepts variable of `z_owned_xxx_t` type and calls corresponding `z_xxx_loan_mut` function.

Move operation
--------------

Function `z_xxx_move` accepts `z_owned_xxx_t*` and
returns pointer `z_moved_xxx_t*` which only allows to take
ownership of `z_owned_xxx_t`. The agreement is that the function which accepts `z_moved_xxx_t*` parameter
is obliged to take ownership on it (see "take" operation)

`z_move` macro accepts varible of `z_owned_xxx_t` type anc calls corresponding `z_move_xxx` function

Take operation
--------------

Functions `z_xxx_take_moved` and `z_xxx_take_loaned` accepts pointer
to unitialized `z_owned_xxx_t` destination structure and
 `z_moved_xxx_t*` and `z_loaned_xxx_t*` source pointers correspondingly.

These functions moves data from source `z_owned_xxx_t` structure into destination one. The source
structure is set to empty "gravestone" state (see "check" operation)

`z_take` macro accepts `z_moved_xxx_t*` and `z_loaned_xxx_t*` pointers and calls corresponding
`z_xxx_take_moved` and `z_xxx_take_loaned` functions.

Check operation
---------------

When owned object is dropped or taken it's set to so-called "gravestone" state which is safe to 
double drop. No operations except "check" and "drop" are normally allowed on dropped/taken object.

Function `z_xxx_check` returns true if object is in valid state, e.g. if all operations
on the object are allowed.

There is small catch: for some objects the gravestone state is still valid state.
Examples are `z_owned_bytes_t` which is set to "empty" state by the drop and `z_owned_encoding_t`
which becomes `ZENOH_BYTES`. For such objects the `z_check` always returns true, even after "drop" or "take"
operation.

`z_check` macro accepts `const z_owned_xxx_t*` and calls corresponding `z_xxx_check` function

Drop operation
--------------

Function `z_xxx_drop` accepts `z_moved_xxx_t*` pointer. It frees all resources hold by corresponding
`z_owned_xxx_t` object and sets this object to gravestone state, safe to double drop.

`z_drop` macro accepts `z_moved_xxx_t*` and calls corresponding `z_xxx_drop` function

Name Prefixes `z_`, `zc_`, `ze_`
================================

Most functions and types in the C API use the `z_` prefix, which applies to the common zenoh C API
(currently Rust-based zenoh-c and pure C zenoh-pico).

The `zc_` prefix is specific to zenoh-c. zenoh-pico uses the `zp_` prefix for the same purpose.

The `ze_` prefix identifies functions and types from the `zenoh-ext` Rust library that are not
part of the core Zenoh API and therefore are placed in a separate namespace.