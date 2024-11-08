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
The following operations are available: move, loan, mutable loan, take, and drop. They are performed for 
"xxx" entities by functions `z_xxx_move`, `z_xxx_loan`, `z_xxx_loan_mut`, `z_xxx_take`, `z_xxx_take_loaned` 
(for certain types), and `z_xxx_drop`.
The generic macros `z_move`, `z_loan`, `z_loan_mut`, `z_take`, and `z_drop` are also provided.

Loan operation
--------------

Function `z_xxx_loan` accepts `const z_owned_xxx_t*` and returns a pointer `const z_loaned_xxx_t*` which gives read-only 
access to the `z_owned_xxx_t` entity.

The `z_loan` macro accepts a variable of `z_owned_xxx_t` type and calls the corresponding `z_xxx_loan` function.

Mutable loan operation
----------------------

The function `z_xxx_loan_mut` accepts `z_owned_xxx_t*` and
returns a pointer `z_xxx_loaned_t*` which allows reading and modifying the `z_owned_xxx_t` entity. 

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

Functions `z_xxx_take` accept pointers to uninitialized `z_owned_xxx_t` destination structures and
`z_moved_xxx_t*` source pointers.

These functions move data from the source `z_owned_xxx_t` structure into the destination one. The source
structure is set to an empty "gravestone" state, like after a drop operation.

The `z_take` macro accepts `z_moved_xxx_t*` pointer and calls the corresponding
`z_xxx_take` functions.

Take from mutably loaned object operation
-----------------------------------------

Functions `z_xxx_take_from_loaned` accept pointers to uninitialized `z_owned_xxx_t` destination structures and
`z_loaned_xxx_t*` source pointers.

These functions move data from the source `z_loaned_xxx_t` structure into the destination one. The source
structure is set to "valid but unspecified" state: it **have** to be dropped, no other operation on it is safe,
unless if it's explicitly specified. See also section "Comparison with C++ move semantics".

The `z_take_from_loaned` macro accepts `z_loaned_xxx_t*` pointer and calls the corresponding
`z_xxx_take_from_loaned` functions.



There is also an API for taking owned object from a mutably loaned object: the `z_xxx_take_from_loaned` functions. 
After this operation object is left in some "valid but unspecified" state. The logic is similar to C++ guarantee after `std::move` operation
(see https://stackoverflow.com/questions/7930105/does-moving-leave-the-object-in-a-usable-state)

The difference from C++ is that, instead of single `std::move` zenoh-c provides different `z_move` and `z_loan_mut` operations with corresponding
`z_take` and `z_take_from_loaned`. This is made for user convenience: 

 It have to be dropped
These functions
are available only for types passed to callback functions as mutable references, such as `z_loaned_sample_t*`, `z_loaned_reply_t*`, 
`z_loaned_hello_t*`, and `z_loaned_query_t*`.

This feature is specifically designed for callbacks: it allows the callback to either process the passed object in place or take 
ownership of it for further processing. No zenoh-c API functions take ownership of mutably loaned objects, i.e it's always safe to pass object
to function with `z_loan_mut` and continue using it after return.

Drop operation
--------------

Function `z_xxx_drop` accepts `z_moved_xxx_t*` pointer. It frees all resources hold by corresponding
`z_owned_xxx_t` object and sets this object to gravestone state, safe to double drop.

`z_drop` macro accepts `z_moved_xxx_t*` and calls corresponding `z_xxx_drop` function

Comparison with C++ move semantics
==================================

The behavior of `z_move` is similar to C++ `std::move`, it converts normal reference to a "rvalue reference" which is intended to be consumed by the function.
The difference is that the C++ automatically destructs the object. So the move semantics in C++ means taking the heavy data from rvalue-referenced object and
leaving it in some valid state which is later destroyed by it's destructor. 

There is no automatic destructors in C, so for the same logic we would need to require developer to call destructor (`z_drop`) even after `z_move` operation. 
This is inconvenient, so for move operation our requirement is more strict than for C++: if function expects `z_moved_xxx_t*` it 
should left the object on passed pointer in "gravestone" state, i.e. state which doesn't hold any external resources and so safe to be forgotten. 
(There is also a second requirement for gravestone state: double drop safety. This decision is kind of arbitrary, but it helps to avoid segmentation faults).

Unfortunately this strict `z_move` semantic is not enough in the situations below:

First problem is that arguments of callbacks are "mutable loaned" references (e.g. `z_loaned_sample_t*`). It would be more logical to make them "moved" references to give
ownership to the callback function. But in this case the callback function would be obliged to take the ownership and drop the object after use even if
he needs only to read the object.

But on the other hand sometimes it's necessary to take ownership of the object passed to callback for further processing. Therefore the take
operation from mutable reference is required.

The second problem is that the C++ API doesn't use the moved/loaned syntax sugar, as C++ has its own move semantics. The C++ `std::move` can be called
on any non-const reference, so we need to support this behavior. Detailed explanation is in note below, it's ok to skip it.

..note::
Zenoh C++ API bypasses zenoh-c protection by simply making `reinterpret_cast` from `z_loaned_xxx_t*` to `z_owned_xxx_t*` and back when necessary. This means that 
if the move constructor in C++ accepts e.g. C++ object `Reply&&`, it can't be sure if this reference points to `z_owned_reply_t` with valid gravestone state (internally in Rust this
corresponds to `Option<Reply>` and gravestone state is `None`) or is it `z_loaned_reply_t*` received from inside zenoh-c, which points just to `Reply`, not option-wrapped. 
(It's important to notice that `Reply` and `Option<Reply>` have same size and layout in memory due to Null-Pointer Optimization, so it's safe to treat
 `Option<Reply>` just as `Reply`, but not in other direction).

To resolve this the `z_take_from_loaned` operation is introduced for `z_loaned_xxx_t*`. It behaves similarly to `z_take` for `z_moved_xxx_t*` but doesn't provide
guarantee that the object is kept in gravestone state. Instead it only guarantees that the object is left in state safe to be dropped, nothing more. 
Unlike `z_move`, this operation is full equivalent to C++ move semantics: object is left in "valid but unspecified" state and it still has to be destructed.

Zenoh guarantees that it never uses this operation inside its code. I.e. it's always safe to pass object to function with `z_loan_mut` and continue using it after return. 
It's recommended to follow this rule in user code too and use `z_take_from_loaned` only in exceptional cases.

Examples:

`z_move` and `z_take` usage:

.. code-block:: c

    void consume_string(z_moved_string_t* ps) {
        z_owned_string_t s;
        z_take(&s, ps);
        printf("%.*s\n", z_string_len(z_loan(s)), z_string_data(z_loan(s)));
        z_drop(s);
    }
    ...
    z_owned_string_t s;
    z_string_copy_from_str(&s, "Hello, world!");
    consume_string(z_move(s));
    // no need to drop s here, passing it by z_move promises that it's dropped inside consume_string
``

`z_loan_mut` and `z_take_from_loaned` usage:

.. code-block:: c

    void may_consume_string(z_loaned_string_t* ps) {
        if (z_string_len(ps) < 42) {
            // process it in place
        } else {
            z_owned_string_t s;
            z_take_from_loaned(&s, ps);
            // save s for further processing
        }
    }
    ...
    z_owned_string_t s;
    z_string_copy_from_str(&s, "Hello, world!");
    may_consume_string(z_loan_mut(s));
    // can't make any assumptions about s here, but still obliged to drop it
    z_drop(s);

``


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