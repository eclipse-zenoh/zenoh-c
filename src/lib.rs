//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
use async_std::channel::{bounded, Sender};
use async_std::prelude::FutureExt;
use async_std::sync::Arc;
use async_std::task;
use futures::prelude::*;
use futures::select;
use libc::{c_char, c_int, c_uchar, c_uint, c_ulong, size_t};
use std::ffi::{c_void, CStr};
use std::mem::ManuallyDrop;
use std::slice;
use zenoh::config::whatami::WhatAmIMatcher;
use zenoh::config::{Config, ConfigProperties, IntKeyMapLike, WhatAmI};
use zenoh::info::InfoProperties;
use zenoh::prelude::{Encoding, Priority, Sample, SampleKind, Selector, ZFuture, ZInt};
use zenoh::publication::CongestionControl;
use zenoh::queryable::Query;
use zenoh::scouting::Hello;
use zenoh::Session;

mod types;
pub use types::*;

pub const Z_SESSION_PADDING_U64: usize = 3;

/// An owned zenoh session.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by borrowing it using `z_X_borrow(&val)`.  
/// The `z_borrow(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_borrow(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_owned_session_t([u64; Z_SESSION_PADDING_U64]);
impl From<Session> for z_owned_session_t {
    fn from(session: Session) -> Self {
        unsafe { z_owned_session_t(std::mem::transmute(Some(session))) }
    }
}
impl AsRef<Option<Session>> for z_owned_session_t {
    fn as_ref(&self) -> &Option<Session> {
        unsafe { std::mem::transmute(self) }
    }
}
impl AsMut<Option<Session>> for z_owned_session_t {
    fn as_mut(&mut self) -> &mut Option<Session> {
        unsafe { std::mem::transmute(self) }
    }
}
impl AsRef<Option<Session>> for z_session_t {
    fn as_ref(&self) -> &Option<Session> {
        unsafe { (&*self.0).as_ref() }
    }
}

/// A borrowed zenoh session.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_session_t(*const z_owned_session_t);

/// Returns a :c:type:`z_session_t` borrowed from `s`.
#[no_mangle]
pub extern "C" fn z_session_borrow(s: &z_owned_session_t) -> z_session_t {
    z_session_t(s)
}

/// A borrowed zenoh config.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_config_t(*const z_owned_config_t);
/// An owned zenoh configuration.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by borrowing it using `z_X_borrow(&val)`.  
/// The `z_borrow(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_borrow(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_owned_config_t(*mut ());

/// Returns a :c:type:`z_config_t` borrowed from `s`.
#[no_mangle]
pub extern "C" fn z_config_borrow(s: &z_owned_config_t) -> z_config_t {
    z_config_t(s)
}
impl AsRef<Option<Config>> for z_config_t {
    fn as_ref(&self) -> &Option<Config> {
        unsafe { (&*self.0).as_ref() }
    }
}
impl AsMut<Option<Config>> for z_config_t {
    fn as_mut(&mut self) -> &mut Option<Config> {
        unsafe { (&mut *(self.0 as *mut z_owned_config_t)).as_mut() }
    }
}
impl AsRef<Option<Config>> for z_owned_config_t {
    fn as_ref(&self) -> &Option<Config> {
        unsafe { std::mem::transmute(self) }
    }
}
impl AsMut<Option<Config>> for z_owned_config_t {
    fn as_mut(&mut self) -> &mut Option<Config> {
        unsafe { std::mem::transmute(self) }
    }
}

enum SubOps {
    Pull,
    Close,
}

type Subscriber = Option<Arc<Sender<SubOps>>>;
pub const Z_SUBSCRIBER_PADDING_U64: usize = 1;
/// An owned zenoh subscriber. Destroying the subscriber cancels the subscription.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by borrowing it using `z_X_borrow(&val)`.  
/// The `z_borrow(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_borrow(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_owned_subscriber_t([u64; Z_SUBSCRIBER_PADDING_U64]);
impl AsRef<Subscriber> for z_owned_subscriber_t {
    fn as_ref(&self) -> &Subscriber {
        unsafe { std::mem::transmute(self) }
    }
}
impl AsMut<Subscriber> for z_owned_subscriber_t {
    fn as_mut(&mut self) -> &mut Subscriber {
        unsafe { std::mem::transmute(self) }
    }
}

type Queryable = Option<Arc<Sender<bool>>>;
pub const Z_QUERYABLE_PADDING_U64: usize = 1;
/// An owned zenoh queryable.  
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by borrowing it using `z_X_borrow(&val)`.  
/// The `z_borrow(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_borrow(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_owned_queryable_t([u64; Z_QUERYABLE_PADDING_U64]);
impl AsRef<Queryable> for z_owned_queryable_t {
    fn as_ref(&self) -> &Queryable {
        unsafe { std::mem::transmute(self) }
    }
}
impl AsMut<Queryable> for z_owned_queryable_t {
    fn as_mut(&mut self) -> &mut Queryable {
        unsafe { std::mem::transmute(self) }
    }
}
#[allow(non_camel_case_types)]
pub struct z_query_t(Query);

/// Constructs a key expression from an expression id.
/// Since id-only kes expressions don't need destruction, a `z_keyexpr_t` is returned instead of its owned variant.
#[no_mangle]
pub extern "C" fn z_id(id: c_ulong) -> z_keyexpr_t {
    unsafe { z_keyexpr_new_borrowed(id, std::ptr::null()) }
}

/// Constructs a borrowed key expression from an expression id and a suffix.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_id_with_suffix(id: c_ulong, suffix: *const c_char) -> z_keyexpr_t {
    z_keyexpr_new_borrowed(id, suffix)
}

/// Constructs a key expression from an expression id and a suffix. `suffix`'s content is copied.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by borrowing it using `z_X_borrow(&val)`.  
/// The `z_borrow(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_borrow(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_id_with_suffix_new(
    id: c_ulong,
    suffix: *const c_char,
) -> z_owned_keyexpr_t {
    z_keyexpr_new(id, suffix)
}

/// Constructs a borrowed key expression from a string expression.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_expr(name: *const c_char) -> z_keyexpr_t {
    z_keyexpr_new_borrowed(0, name)
}

/// Constructs a key expression from a string expression. `name`'s content is copied.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by borrowing it using `z_X_borrow(&val)`.  
/// The `z_borrow(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_borrow(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_expr_new(name: *const c_char) -> z_owned_keyexpr_t {
    z_keyexpr_new(0, name)
}

/// Return a new, zenoh-allocated, empty configuration.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by borrowing it using `z_X_borrow(&val)`.  
/// The `z_borrow(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_borrow(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[no_mangle]
pub extern "C" fn z_config_new() -> z_owned_config_t {
    unsafe { z_owned_config_t(std::mem::transmute(Some(Box::new(Config::default())))) }
}

/// Gets the number of available keys for configuration.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_config_len(config: z_config_t) -> c_uint {
    config
        .as_ref()
        .as_ref()
        .map(|c| c.ikeys().len() as c_uint)
        .unwrap_or(0)
}

/// Gets the property with the given integer key from the configuration.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_config_get(config: z_config_t, key: c_uint) -> z_owned_string_t {
    let val = config
        .as_ref()
        .as_ref()
        .map(|c| c.iget(key as u64))
        .flatten();
    match val {
        Some(val) => val.into_owned().into(),
        None => z_owned_string_t::default(),
    }
}

/// Inserts a property with a given key to a properties map.
/// If a property with the same key already exists in the properties map, it is replaced.
///
/// Parameters:
///   config: A pointer to the properties map.
///   key: The key of the property to add.
///   value: The value of the property to add.
#[no_mangle]
#[allow(clippy::missing_safety_doc, unused_must_use)]
pub unsafe extern "C" fn z_config_set(mut config: z_config_t, key: c_ulong, value: z_string_t) {
    let value = CStr::from_ptr(value);
    config
        .as_mut()
        .as_mut()
        .expect("invalid config")
        .iset(key as u64, value.to_string_lossy());
}

/// Frees `config`, invalidating it for double-free safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_config_free(config: &mut z_owned_config_t) {
    std::mem::drop(config.as_mut().take())
}
/// Returns `true` if `config` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_config_check(config: &z_owned_config_t) -> bool {
    config.as_ref().is_some()
}

/// Creates an empty, zenoh-allocated, configuration.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_config_empty() -> z_owned_config_t {
    z_config_new()
}

/// Creates a default, zenoh-allocated, configuration.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_config_default() -> z_owned_config_t {
    z_config_new()
}

/// Reads a configuration from a properties-formated string, such as "mode=client;peer=tcp/127.0.0.1:7447".
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_config_from_str(s: *const c_char) -> z_owned_config_t {
    if s.is_null() {
        z_config_empty()
    } else {
        let conf_str = CStr::from_ptr(s);
        let props: Option<Config> = json5::from_str(&conf_str.to_string_lossy()).ok();
        z_owned_config_t(std::mem::transmute(props.map(Box::new)))
    }
}

/// Converts `config` into a properties-formated string, such as "mode=client;peer=tcp/127.0.0.1:7447".
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_config_to_str(config: z_config_t) -> z_owned_string_t {
    let config = match config.as_ref() {
        Some(c) => c,
        None => return z_owned_string_t::default(),
    };
    ConfigProperties::from(config).to_string().into()
}

/// Constructs a configuration by parsing a file at `path`. Currently supported format is JSON5, a superset of JSON.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_config_from_file(path: *const c_char) -> z_owned_config_t {
    let path_str = CStr::from_ptr(path);
    z_owned_config_t(std::mem::transmute(match path_str.to_str() {
        Ok(path) => match zenoh::config::Config::from_file(path) {
            Ok(c) => Some(Box::new(c)),
            Err(e) => {
                log::error!("Couldn't read config from {}: {}", path, e);
                None
            }
        },
        Err(e) => {
            log::error!("Invalid path '{}': {}", path_str.to_string_lossy(), e);
            None
        }
    }))
}

/// Constructs a default configuration peer mode zenoh session.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_config_peer() -> z_owned_config_t {
    unsafe { z_owned_config_t(std::mem::transmute(Some(Box::new(zenoh::config::peer())))) }
}

/// Constructs a default configuration client mode zenoh session.
/// If `peer` is not null, it is added to the configuration as remote peer.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_config_client(
    peers: *const *const c_char,
    n_peers: usize,
) -> z_owned_config_t {
    let locators = if peers.is_null() {
        Vec::new()
    } else if let Ok(locators) = std::slice::from_raw_parts(peers, n_peers)
        .iter()
        .map(|&s| CStr::from_ptr(s).to_string_lossy().parse())
        .fold(
            Ok(Vec::<zenoh::prelude::Locator>::new()),
            |acc, it| match (acc, it) {
                (Err(_), _) | (_, Err(_)) => Err(()),
                (Ok(mut vec), Ok(loc)) => {
                    vec.push(loc);
                    Ok(vec)
                }
            },
        )
    {
        locators
    } else {
        return z_owned_config_t(std::mem::transmute(None::<Box<Config>>));
    };
    z_owned_config_t(std::mem::transmute(Some(Box::new(zenoh::config::client(
        locators,
    )))))
}

/// Gets the key expression of a received query as a non null-terminated string.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_query_key_expr(query: &z_query_t) -> z_keyexpr_t {
    let (scope, s) = query.0.key_selector().as_id_and_suffix();
    let suffix = z_bytes_t {
        start: s.as_ptr(),
        len: s.len(),
    };
    z_keyexpr_t {
        id: scope as c_ulong,
        suffix,
    }
}

/// Gets the predicate of a received query as a non null-terminated string.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_query_predicate(query: &z_query_t) -> z_bytes_t {
    let s = query.0.selector().value_selector;
    z_bytes_t {
        start: s.as_ptr(),
        len: s.len(),
    }
}

/// Scout for routers and/or peers.
///
/// Parameters:
///     `what`: A whatami bitmask of zenoh entities kind to scout for.
///     `config`: A set of properties to configure the scouting.
///     `scout_period`: The time that should be spent scouting before returning the results.
///
/// Returns:
///     An array of `z_hello_t` messages.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_scout(
    what: c_uint,
    config: &mut z_owned_config_t,
    scout_period: c_ulong,
) -> z_owned_hello_array_t {
    let what = WhatAmIMatcher::try_from(what as ZInt).unwrap_or(WhatAmI::Router | WhatAmI::Peer);
    let config = config.as_mut().take().expect("invalid config");

    let hellos = task::block_on(async move {
        let mut hs = std::vec::Vec::<Hello>::new();
        let mut stream = zenoh::scout(what, config).wait().unwrap();
        let scout = async {
            while let Some(hello) = stream.next().await {
                hs.push(hello)
            }
        };
        let timeout = async_std::task::sleep(std::time::Duration::from_millis(scout_period as u64));
        FutureExt::race(scout, timeout).await;
        hs
    });
    hellos.into()
}

/// Initialises the zenoh runtime logger
#[no_mangle]
pub extern "C" fn z_init_logger() {
    env_logger::init();
}

/// Opens a zenoh session. Should the session opening fail, `z_check`ing the returned value will return `false`.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_open(config: &mut z_owned_config_t) -> z_owned_session_t {
    let config = match config.as_mut().take() {
        Some(c) => c,
        None => return z_owned_session_t(std::mem::transmute(None::<Session>)),
    };
    let s = task::block_on(async move { zenoh::open(config).await });
    match s {
        Ok(v) => v.into(),
        Err(e) => {
            log::error!("Error opening session: {}", e);
            z_owned_session_t(std::mem::transmute(None::<Session>))
        }
    }
}

/// Returns `true` if `session` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_session_check(session: &z_owned_session_t) -> bool {
    session.as_ref().is_some()
}

/// Gets informations about an zenoh session.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_info(session: z_session_t) -> z_owned_info_t {
    let session = (&*session.0).as_ref();
    match session {
        Some(s) => z_owned_info_t(std::mem::transmute(task::block_on(s.info()))),
        None => z_owned_info_t(std::mem::transmute(None::<InfoProperties>)),
    }
}

/// Gets informations about an zenoh session as a properties-formatted string.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_info_as_str(session: z_session_t) -> z_owned_string_t {
    let session = (&*session.0).as_ref();
    match session {
        Some(s) => task::block_on(s.info()).to_string().into(),
        None => z_owned_string_t::default(),
    }
}

/// Closes a zenoh session. This frees and invalidates `session` for double-free safety.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_close(session: &mut z_owned_session_t) {
    session.as_mut().take().map(|s| task::block_on(s.close()));
}

/// Associates a numerical id with the given key expression. The id is returned as a :c:type:`z_keyexpr_t` with a nullptr suffix.
///
/// This numerical id will be used on the network to save bandwidth and
/// ease the retrieval of the concerned resource in the routing tables.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_declare_expr(session: z_session_t, keyexpr: z_keyexpr_t) -> z_keyexpr_t {
    let result = session
        .as_ref()
        .as_ref()
        .expect("invalid session")
        .declare_expr(keyexpr)
        .wait()
        .unwrap() as c_ulong;
    z_id(result)
}

/// Unbinds the numerical id key generated by a call to :c:func:`z_declare_expr`.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_undeclare_expr(session: z_session_t, keyexpr: z_keyexpr_t) {
    session
        .as_ref()
        .as_ref()
        .expect("invalid session")
        .undeclare_expr(keyexpr.id as ZInt)
        .wait()
        .unwrap();
}

/// Write data.
///
/// Parameters:
///     session: The zenoh session.
///     keyexpr: The key expression to write.
///     payload: The value to write.
///     len: The length of the value to write.
/// Returns:
///     ``0`` in case of success, ``1`` in case of failure.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_put(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    payload: *const u8,
    len: c_uint,
) -> c_int {
    let r = session
        .as_ref()
        .as_ref()
        .expect("invalid session")
        .put(
            keyexpr,
            slice::from_raw_parts(payload as *const u8, len as usize),
        )
        .wait();

    match r {
        Ok(()) => 0,
        _ => 1,
    }
}
#[derive(Default)]
struct WriteOptions {
    encoding: Encoding,
    congestion_control: CongestionControl,
    kind: SampleKind,
    priority: Priority,
}
pub const Z_WRITE_OPTIONS_PADDING_U64: usize = 6;

/// Options passed to the :c:func:`z_put_ext` function.  
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_put_options_t([u64; Z_WRITE_OPTIONS_PADDING_U64]);

#[repr(C)]
#[allow(non_camel_case_types)]
/// The different kind of options in a :c:type:`z_put_options_t`.
///
///     - **z_put_options_field_t_ENCODING**
///     - **z_put_options_field_t_CONGESTION_CONTROL**
///     - **z_put_options_field_t_KIND**
///     - **z_put_options_field_t_PRIORITY**
pub enum z_put_options_field_t {
    ENCODING,
    CONGESTION_CONTROL,
    KIND,
    PRIORITY,
}

/// Constructs the default value for write options
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_put_options_default() -> z_put_options_t {
    z_put_options_t(std::mem::transmute(WriteOptions::default()))
}

/// Sets the value for the required field of a `z_put_options_t`.  
/// Returns `false` if the value insertion failed.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_put_options_set(
    options: &mut z_put_options_t,
    key: z_put_options_field_t,
    value: c_uint,
) -> bool {
    let options: &mut WriteOptions = std::mem::transmute(options);
    match key {
        z_put_options_field_t::ENCODING => options.encoding = Encoding::from(value as ZInt),
        z_put_options_field_t::CONGESTION_CONTROL => {
            if value < 2 {
                options.congestion_control = std::mem::transmute(value as u8)
            } else {
                return false;
            }
        }
        z_put_options_field_t::KIND => options.kind = (value as ZInt).into(),
        z_put_options_field_t::PRIORITY => {
            if 0 < value && value < 8 {
                options.priority = std::mem::transmute(value as u8)
            } else {
                return false;
            }
        }
    };
    true
}

/// Write data with extended options.
///
/// Parameters:
///     session: The zenoh session.
///     keyexpr: The key expression to write.
///     payload: The value to write.
///     len: The length of the value to write.
///     options: The write options
/// Returns:
///     ``0`` in case of success, ``1`` in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_put_ext(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    payload: *const u8,
    len: c_uint,
    options: &z_put_options_t,
) -> c_int {
    let options: &WriteOptions = std::mem::transmute(options);
    let result = match session
        .as_ref()
        .as_ref()
        .expect("invalid session")
        .put(
            keyexpr,
            slice::from_raw_parts(payload as *const u8, len as usize),
        )
        .encoding(options.encoding.clone())
        .kind(options.kind)
        .congestion_control(options.congestion_control)
        .priority(options.priority)
        .wait()
    {
        Ok(()) => 0,
        _ => 1,
    };
    result
}

/// Declares a publication for the given key expression, returning `true` on success.
///
/// Written resources that match the given key will only be sent on the network
/// if matching subscribers exist in the system.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_declare_publication(session: z_session_t, keyexpr: z_keyexpr_t) -> bool {
    session
        .as_ref()
        .as_ref()
        .map(|s| s.declare_publication(keyexpr).wait().ok())
        .flatten()
        .is_some()
}

/// Undeclares a publication for the given key expression.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_undeclare_publication(session: z_session_t, keyexpr: z_keyexpr_t) {
    session
        .as_ref()
        .as_ref()
        .map(|s| s.undeclare_publication(keyexpr).wait().ok());
}
/// Subscribes to the given key expression.
///
/// Parameters:
///     session: The zenoh session.
///     keyexpr: The key expression to subscribe.
///     sub_info: The :c:type:`z_subinfo_t` to configure the subscriber.
///     callback: The callback function that will be called each time a data matching the subscribed expression is received.
///     arg: A pointer that will be passed to the **callback** on each call.
///
/// Returns:
///    A :c:type:`z_owned_subscriber_t`.
///
///    To check if the subscription succeeded and if the subscriber is still valid,
///    you may use `z_subscriber_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
///
///    Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
///    To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
///    After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_subscribe(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    sub_info: z_subinfo_t,
    callback: extern "C" fn(*const z_sample_t, *const c_void),
    arg: *mut c_void,
) -> z_owned_subscriber_t {
    let arg = Box::from_raw(arg);
    let (tx, rx) = bounded::<SubOps>(8);
    let rsub = z_owned_subscriber_t(std::mem::transmute(Some(Arc::new(tx))));
    let sub = session
        .as_ref()
        .as_ref()
        .expect("invalid session")
        .subscribe(keyexpr)
        .period(sub_info.period.into())
        .reliability(sub_info.reliability.into())
        .mode(sub_info.mode.into());
    let sub = sub.wait().unwrap();
    let mut sub = std::mem::transmute::<_, zenoh::subscriber::Subscriber<'static>>(sub);

    // Note: This is done to ensure that even if the call-back into C
    // does any blocking call we do not incur the risk of blocking
    // any of the task resolving futures.
    task::spawn_blocking(move || {
        task::block_on(async move {
            let key = z_keyexpr_t {
                id: 0,
                suffix: z_bytes_t {
                    start: std::ptr::null(),
                    len: 0,
                },
            };
            let mut sample = z_sample_t {
                key,
                value: z_bytes_t {
                    start: std::ptr::null(),
                    len: 0,
                },
            };
            let arg = Box::into_raw(arg);
            loop {
                select!(
                    s = sub.receiver().next().fuse() => {
                        // This is a bit brutal but avoids an allocation and
                        // a copy that would be otherwise required to add the
                        // C string terminator. See the test_sub.c to find out how to deal
                        // with non null terminated strings.
                        let us = s.unwrap();
                        let data = us.value.payload.to_vec();
                        sample.key = (&us.key_expr).into();
                        sample.value.start = data.as_ptr() as *const c_uchar;
                        sample.value.len = data.len() as size_t;
                        callback(&sample, arg)
                    },
                    op = rx.recv().fuse() => {
                        match op {
                            Ok(SubOps::Pull) => {
                                let _ = sub.pull().await;
                            },

                            Ok(SubOps::Close) => {
                                let _ = sub.close().await;
                                return
                            },
                            _ => return
                        }
                    }
                )
            }
        })
    });
    rsub
}

/// Pull data for a pull mode :c:type:`z_owned_subscriber_t`. The pulled data will be provided
/// by calling the **callback** function provided to the :c:func:`z_subscribe` function.
///
/// Parameters:
///     sub: The :c:type:`z_owned_subscriber_t` to pull from.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_pull(sub: &z_owned_subscriber_t) {
    match sub.as_ref() {
        Some(tx) => {
            let _ = async_std::task::block_on(tx.send(SubOps::Pull));
        }
        None => (),
    }
}

/// Unsubscribes from the passed `sub`, freeing it and invalidating it for double-free safety.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_subscriber_close(sub: &mut z_owned_subscriber_t) {
    let sub = sub.as_mut();
    match sub {
        Some(tx) => {
            let _ = async_std::task::block_on(tx.send(SubOps::Close));
            *sub = None;
        }
        None => (),
    }
}

/// Returns `true` if `sub` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_subscriber_check(sub: &z_owned_subscriber_t) -> bool {
    sub.as_ref().is_some()
}

/// Query data from the matching queryables in the system.
/// Replies are provided through a callback function.
///
/// Parameters:
///     session: The zenoh session.
///     keyexpr: The key expression matching resources to query.
///     predicate: An indication to matching queryables about the queried data.
///     target: The kind of queryables that should be target of this query.
///     consolidation: The kind of consolidation that should be applied on replies.
///     callback: The callback function that will be called on reception of replies for this query.
///     arg: A pointer that will be passed to the **callback** on each call.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_get(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    predicate: *const c_char,
    target: z_query_target_t,
    consolidation: z_query_consolidation_t,
    callback: extern "C" fn(z_owned_reply_t, *const c_void),
    arg: *mut c_void,
) {
    let p = CStr::from_ptr(predicate).to_str().unwrap();
    let arg = Box::from_raw(arg);
    let mut q = session
        .as_ref()
        .as_ref()
        .expect("invalid session")
        .get(Selector {
            key_selector: keyexpr.into(),
            value_selector: p,
        })
        .target(target.into())
        .consolidation(consolidation.into())
        .wait()
        .unwrap();

    task::spawn_blocking(move || {
        task::block_on(async move {
            let arg = Box::into_raw(arg);
            while let Some(reply) = q.next().await {
                callback(
                    z_owned_reply_t {
                        tag: z_reply_t_Tag::DATA,
                        data: reply.into(),
                    },
                    arg,
                )
            }
            callback(
                z_owned_reply_t {
                    tag: z_reply_t_Tag::FINAL,
                    data: z_owned_reply_data_t::empty(),
                },
                arg,
            )
            // while let Some(reply) = q.next().await {
            //     callback(z_reply_t::DATA { data: reply.into() }, arg)
            // }
            // callback(z_reply_t::FINAL, arg)
        })
    });
}

/// Query data from the matching queryables in the system.
/// Replies are collected in an array.
///
/// Parameters:
///     session: The zenoh session.
///     keyexpr: The key expression matching resources to query.
///     predicate: An indication to matching queryables about the queried data.
///     target: The kind of queryables that should be target of this query.
///     consolidation: The kind of consolidation that should be applied on replies.
///
/// Returns:
///    An array containing all the replies for this query.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_get_collect(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    predicate: *const c_char,
    target: z_query_target_t,
    consolidation: z_query_consolidation_t,
) -> z_owned_reply_data_array_t {
    let p = CStr::from_ptr(predicate).to_str().unwrap();
    let mut replies = task::block_on(async {
        let q = session
            .as_ref()
            .as_ref()
            .expect("invalid session")
            .get(Selector {
                key_selector: keyexpr.into(),
                value_selector: p,
            })
            .target(target.into())
            .consolidation(consolidation.into())
            .await
            .unwrap();
        q.collect::<Vec<_>>().await
    })
    .into_iter()
    .map(|r| r.into())
    .collect::<Vec<z_owned_reply_data_t>>();

    replies.shrink_to_fit();
    //TODO replace when stable https://github.com/rust-lang/rust/issues/65816
    let (val, len, _cap) = vec_into_raw_parts(replies);
    z_owned_reply_data_array_t { val, len }
}

/// Creates a Queryable for the given key expression.
///
/// Parameters:
///     session: The zenoh session.
///     keyexpr: The key expression the Queryable will reply to.
///     kind: The kind of Queryable.
///     callback: The callback function that will be called each time a matching query is received.
///     arg: A pointer that will be passed to the **callback** on each call.
///
/// Returns:
///    The created :c:type:`z_owned_queryable_t` or null if the creation failed.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_queryable_new(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    kind: c_uint,
    callback: extern "C" fn(&z_query_t, *const c_void),
    arg: *mut c_void,
) -> z_owned_queryable_t {
    let arg = Box::from_raw(arg);
    let (tx, rx) = bounded::<bool>(1);
    let r = z_owned_queryable_t(std::mem::transmute(Some(Arc::new(tx))));
    let queryable = session
        .as_ref()
        .as_ref()
        .expect("invalid session")
        .queryable(keyexpr)
        .kind(kind as ZInt)
        .wait()
        .unwrap();
    let mut queryable: zenoh::queryable::Queryable<'static> = std::mem::transmute(queryable);

    // Note: This is done to ensure that even if the call-back into C
    // does any blocking call we do not incour the risk of blocking
    // any of the task resolving futures.
    task::spawn_blocking(move || {
        task::block_on(async move {
            let arg = Box::into_raw(arg);
            loop {
                select!(
                query = queryable.receiver().next().fuse() => {
                  // This is a bit brutal but avoids an allocation and
                  // a copy that would be otherwise required to add the
                  // C string terminator. See the test_sub.c to find out how to deal
                  // with non null terminated strings.
                  let query = z_query_t(query.unwrap());
                  callback(&query, arg);
                },
                _ = rx.recv().fuse() => {
                    let _ = queryable.close().await;
                    return
                })
            }
        })
    });
    r
}

/// Close a `z_owned_queryable_t`, freeing it and invalidating it for doube-free safety.
///
/// Parameters:
///     qable: The :c:type:`z_owned_queryable_t` to close.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_queryable_close(qable: &mut z_owned_queryable_t) {
    let qable = qable.as_mut();
    match qable {
        Some(tx) => {
            let _ = async_std::task::block_on(tx.send(true));
            *qable = None;
        }
        None => (),
    }
}

/// Returns `true` if `qable` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_queryable_check(qable: &z_owned_queryable_t) -> bool {
    qable.as_ref().is_some()
}

/// Send a reply to a query.
///
/// This function must be called inside of a Queryable callback passing the
/// query received as parameters of the callback function. This function can
/// be called multiple times to send multiple replies to a query. The reply
/// will be considered complete when the Queryable callback returns.
///
/// Parameters:
///     query: The query to reply to.
///     key: The key of this reply.
///     payload: The value of this reply.
///     len: The length of the value of this reply.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_send_reply(
    query: &z_query_t,
    key: *const c_char,
    payload: *const u8,
    len: c_uint,
) {
    let name = CStr::from_ptr(key).to_str().unwrap();
    let s = Sample::new(
        name.to_string(),
        slice::from_raw_parts(payload as *const u8, len as usize),
    );
    query.0.replies_sender.send(s);
}

//TODO replace when stable https://github.com/rust-lang/rust/issues/65816
#[inline]
pub(crate) fn vec_into_raw_parts<T>(v: Vec<T>) -> (*mut T, usize, usize) {
    let mut me = ManuallyDrop::new(v);
    (me.as_mut_ptr(), me.len(), me.capacity())
}
