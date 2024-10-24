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
remain valid only as long as the data they reference is valid. Typically the view types are the variants of
owned types that do not own the data. This allows to use view and owned types interchangeably.

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
    z_drop(z_move(owned)); // but no need to drop view

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
    ze_serialize_int64(&attachment, metadata);
    options.attachment = z_move(attachment); // the data itself is still in the `attachment`

    z_owned_bytes_t payload;
    z_bytes_copy_from_str(&payload, "Don't panic!");
    z_publisher_put(z_loan(pub), z_move(payload), &options);
    // the `payload` and `attachment` are consumed by the `z_publisher_put` function


Other Structures and Enums `z_xxx_t`
-----------------------------------------

Types named `z_xxx_t` are copyable, and can be passed by value. Some of them are just plain data structures or enums, like 
`z_timestamp_t`, `z_priority_t`. Some are temporary data access structures, like `z_bytes_slice_iterator_t`, `z_bytes_reader_t`, etc.

.. code-block:: c

    z_timestamp_t ts;
    z_timestamp_new(&ts, z_loan(session));
    z_timestamp_t ts1 = ts;

Common operations
=================

The transition between "owned", "loaned" and "moved" structures above is performed by corresponding functions.
The following operations are available: move, loan, mutable loan, take, check, and drop. They are performed for 
"xxx" entities by functions `z_xxx_move`, `z_xxx_loan`, `z_xxx_loan_mut`, `z_xxx_take_moved`, `z_xxx_take_loaned`,
`z_xxx_check`, and `z_xxx_drop`.
The generic macros `z_move`, `z_loan`, `z_loan_mut`, `z_take`, `z_check`, and `z_drop` are also provided.

Loan operation
--------------

Function `z_xxx_loan` accepts `const z_owned_xxx_t*` and returns a pointer `const z_loaned_xxx_t*` which gives read-only 
access to the `z_owned_xxx_t` entity.

The `z_loan` macro accepts a variable of `z_owned_xxx_t` type and calls the corresponding `z_xxx_loan` function.

Mutable loan operation
----------------------

The function `z_xxx_loan_mut` accepts `z_owned_xxx_t*` and
returns a pointer `z_xxx_loaned_t*` which allows reading and modifying the `z_owned_xxx_t` entity. 

There is also API for taking ownership of the mutably loaned object: `z_xxx_take_loaned` functions. This
is useful when user's code accepts a mutable loaned object. In this case the user's code is free to take
the passed object for further processing or to process it on place without taking ownership. This was done 
primarily for the callback functions: the callback handler is not obliged to take ownership of the passed object but
can do it if needed.

Though it's important to note that the zenoh API itself **never** takes ownership of the mutably loaned object. Otherwise,
the user would be obliged to call `z_check` on the object each time after mutably passing it to the zenoh API.

The `z_loan_mut` macro accepts a variable of `z_owned_xxx_t` type and calls the corresponding `z_xxx_loan_mut` function.

Move operation
--------------

The function `z_xxx_move` accepts `z_owned_xxx_t*` and
returns a pointer `z_moved_xxx_t*` which only allows taking
ownership of the `z_owned_xxx_t`. The agreement is that the function which accepts a `z_moved_xxx_t*` parameter
is obliged to take ownership of it (see "take" operation).

The `z_move` macro accepts a variable of `z_owned_xxx_t` type and calls the corresponding `z_xxx_move` function.

Take operation
--------------

Functions `z_xxx_take_moved` and `z_xxx_take_loaned` accept pointers
to uninitialized `z_owned_xxx_t` destination structures and
`z_moved_xxx_t*` and `z_loaned_xxx_t*` source pointers, respectively.

These functions move data from the source `z_owned_xxx_t` structure into the destination one. The source
structure is set to an empty "gravestone" state (see "check" operation).

The `z_take` macro accepts `z_moved_xxx_t*` or `z_loaned_xxx_t*` pointer and calls the corresponding
`z_xxx_take_moved` and `z_xxx_take_loaned` functions.

Check operation
---------------

When an owned object is dropped or taken, it's set to a so-called **gravestone** state, which is safe to 
double drop. No operations except "check" and "drop" are usually allowed on a dropped/taken object.

The function `z_xxx_check` returns true if the object is in a **valid** state, e.g., if the loan operation
on the object is allowed.

There is a catch: **gravestone** and **valid** states are not always opposite.
For some objects, the gravestone state is still a valid state.
Examples are `z_owned_bytes_t` in the "empty" state or `z_owned_encoding_t`
with `ZENOH_BYTES` encoding set. For such objects, the `z_check` always returns true, 
even after a "drop" or "take" operation.

The `z_check` macro accepts `const z_owned_xxx_t*` and calls corresponding `z_xxx_check` function.

Drop operation
--------------

Function `z_xxx_drop` accepts `z_moved_xxx_t*` pointer. It frees all resources hold by corresponding
`z_owned_xxx_t` object and sets this object to gravestone state, safe to double drop.

`z_drop` macro accepts `z_moved_xxx_t*` and calls corresponding `z_xxx_drop` function

Name Prefixes `z_`, `zc_`, `ze_`
================================

We try to maintain a common API between `zenoh-c` and `zenoh-pico`, such that porting code from one to the other is, ideally, trivial.
However, due to design limitations some functionality might be represented differently (or simply be not available) in either library.

The namespace prefixes are used to distinguish between different parts of the API.

Most functions and types in the C API use the `z_` prefix, which applies to the core Zenoh API.
These functions and types are guaranteed to be available in all Zenoh implementations on C 
(currently, Rust-based zenoh-c and pure C zenoh-pico).

The `zc_` prefix identifies API specific to zenoh-c, while zenoh-pico uses the `zp_` prefix for the same purpose.
E.g. zenoh-c and zenoh-pico have different approaches to configuration and therefore each have their own set 
of `zc_config_...` and `zp_config_...` functions.

The `ze_` prefix is used for the API that is not part of the core zenoh API. There is no guarantee that
these functions and types are available for both implementations. However, when they are provided for both, they should
have the same prototype and behavior. Typically, these are functions and types provided by the `zenoh-ext` Rust library 
for zenoh-c and are not available in zenoh-pico. However, the data serialization API is implemented in zenoh-pico with 
the same `ze_` prefix.