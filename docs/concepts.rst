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

Owned objects can be passed to functions in two ways: by moving (passing the type `z_moved_xxx_t*`) or 
loaning (passing the type `z_loaned_xxx_t*`). In reality, these types are just pointers to the owned object, 
but the different types allow expressing the semantics of the operation:

- Passing `z_owned_xxx_t*` means passing a pointer to an uninitialized structure for constructing it.
- Passing `const z_loaned_xxx_t*` means passing a pointer to an owned structure that the function should not modify.
- Passing `z_loaned_xxx_t*` means passing a pointer to an owned structure that the function may modify, 
    but should leave in a usable state after return. There is one exception, see `Take from mutably loaned object operation` below.
- Passing `z_moved_xxx_t*` means passing a pointer to an owned structure to be consumed by the function, i.e., the caller should not use 
    it after the call and does not have to drop it.

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
Use the `z_xxx_move` function or the `z_move` macro to pass an owned object to such a function.

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
remain valid only as long as the data they reference is valid. Internally the view types are the variants of
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
"xxx" entities by functions `z_xxx_move`, `z_xxx_loan`, `z_xxx_loan_mut`, `z_xxx_take`, `z_xxx_take_from_loaned` 
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

Drop operation
--------------

Function `z_xxx_drop` accepts `z_moved_xxx_t*` pointer. It frees all resources hold by corresponding
`z_owned_xxx_t` object and sets this object to gravestone state, safe to double drop.

`z_drop` macro accepts `z_moved_xxx_t*` and calls corresponding `z_xxx_drop` function

Comparison with C++ move semantics
==================================

The behavior of `z_move` is similar to C++ `std::move`, as it converts a normal reference to an "rvalue reference" intended to be consumed by the function.
However, there is one significant difference: C++ calls the destructor automatically. Therefore, in C++, it is safe to leave the source object in a state that requires destruction.
This also means that in C++, the function that accepts an rvalue reference has no obligation to do anything with this reference. It is only important for the caller 
to not use it after the call.

There are no automatic destructors in C, so for the same logic, we would need to require the developer to call the destructor (`z_drop`) after the `z_move` operation. 
This is inconvenient, so for the move operation, our requirement is stricter than for C++: if a function expects `z_moved_xxx_t*`, it 
should leave the object on the passed pointer in a "gravestone" state, i.e., a state that does not hold any external resources and is safe to be forgotten.

Extending the move semantic for loaned references
=================================================

There is one important situation when we need to support move semantic similar to the C++ one: callbacks.

The arguments of callbacks are "mutable loaned" references (e.g. `z_loaned_sample_t*`). This allows to developer to not care about ownership of the object passed to callback:
the object passed is guaranteed to be destroyed by the caller.

But on the other hand sometimes it's necessary to take ownership of the object passed to callback for further processing. Therefore the take
operation from mutable reference is required.

To resolve this the `z_xxx_take_from_loaned` operation is introduced for `z_loaned_xxx_t*`. It behaves similarly to `z_xxx_take` for `z_moved_xxx_t*`: constructing new 
`z_owned_xxx_t` object, taking the data from the source object and leaving the source object in probably unusable state. But unlike `z_xxx_take`, the `z_xxx_take_from_loaned` doesn't
guarantee "gravestone" state after the operation, i.e. after the "take from loaned" operation the developer is still obliged to drop the source object.

Important: Zenoh API guarantees that it never uses this operation inside its code. I.e. it's always safe to pass object to function with `z_loan_mut` and continue using it after return.
The only purpose of this functionality is to allow user code to take ownership of the object passed to callbacks.

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

`z_take_from_loaned` usage *(Notice that this example if fictional: actually take from loaned is implemented only
for types used in callbacks at this moment: `z_loaned_sample_t*`, `z_loaned_reply_t*`, `z_loaned_hello_t*`, `z_loaned_query_t*`)*:

.. code-block:: c
    void sub_callback(z_loaned_sample_t* sample, void* arg) {
        z_owned_sample_t s;
        z_take_from_loaned(&s, sample);
        // Now we can save `s`` for further processing, e.g. send it to another thread

        // no need to drop `sample`` here, the subscriber will drop it        
    }
    ...
    z_owned_closure_sample_t callback;
    z_closure(&callback, sub_callback, NULL, NULL);
    z_owned_subscriber_t sub;
    if (z_declare_subscriber(z_loan(session), &sub, z_loan(keyexpr), z_move(callback), NULL) < 0) {
        printf("Unable to declare subscriber.\n");
        exit(-1);
    }


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