//
// Copyright (c) 2017, 2022 ZettaScale Technology.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh team, <zenoh@zettascale.tech>
//

use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ops::DerefMut;
use std::str::FromStr;

use crate::collections::*;
use crate::decl_transmute_copy;
use crate::keyexpr::*;
use crate::transmute::Inplace;
use crate::transmute::InplaceDefault;
use crate::transmute::StaticRef;
use crate::transmute::TransmuteCopy;
use crate::transmute::TransmuteRef;
use crate::z_congestion_control_t;
use crate::z_id_t;
use crate::z_owned_buffer_t;
use crate::z_priority_t;
use crate::zc_owned_payload_t;
use crate::zc_payload_t;
use crate::{impl_guarded_transmute, GuardedTransmute};
use libc::c_void;
use libc::{c_char, c_ulong};
use std::convert::Infallible;
use unwrap_infallible::UnwrapInfallible;
use zenoh::buffers::ZBuf;
use zenoh::encoding::Encoding;
use zenoh::payload::Deserialize;
use zenoh::payload::ZSerde;
use zenoh::prelude::SampleKind;
use zenoh::query::ReplyKeyExpr;
use zenoh::sample::Locality;
use zenoh::sample::Sample;
use zenoh::time::Timestamp;

use crate::attachment::{attachment_iteration_driver, z_attachment_null, z_attachment_t};

/// A zenoh unsigned integer
#[allow(non_camel_case_types)]
pub type z_zint_t = c_ulong;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum z_sample_kind_t {
    PUT = 0,
    DELETE = 1,
}

impl From<SampleKind> for z_sample_kind_t {
    fn from(k: SampleKind) -> Self {
        match k {
            SampleKind::Put => z_sample_kind_t::PUT,
            SampleKind::Delete => z_sample_kind_t::DELETE,
        }
    }
}

impl From<z_sample_kind_t> for SampleKind {
    fn from(k: z_sample_kind_t) -> Self {
        match k {
            z_sample_kind_t::PUT => SampleKind::Put,
            z_sample_kind_t::DELETE => SampleKind::Delete,
        }
    }
}

#[repr(C)]
pub struct z_timestamp_t {
    time: u64,
    id: z_id_t,
}

/// Returns ``true`` if `ts` is a valid timestamp
#[no_mangle]
pub extern "C" fn z_timestamp_check(ts: z_timestamp_t) -> bool {
    ts.id.id.iter().any(|byte| *byte != 0)
}
impl From<Option<&Timestamp>> for z_timestamp_t {
    fn from(ts: Option<&Timestamp>) -> Self {
        if let Some(ts) = ts {
            z_timestamp_t {
                time: ts.get_time().as_u64(),
                id: z_id_t {
                    id: ts.get_id().to_le_bytes(),
                },
            }
        } else {
            z_timestamp_t {
                time: 0,
                id: z_id_t { id: [0u8; 16] },
            }
        }
    }
}

/// A data sample.
///
/// A sample is the value associated to a given resource at a given point in time.
#[repr(C)]
pub struct z_sample_t<'a> {
    _inner: &'a (),
}
impl<'a> core::ops::Deref for z_sample_t<'a> {
    type Target = Sample;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute::<&(), &Sample>(self._inner) }
    }
}

impl<'a> z_sample_t<'a> {
    pub fn new(sample: &'a Sample) -> Self {
        z_sample_t {
            _inner: unsafe { core::mem::transmute(sample) },
        }
    }
}

/// The Key Expression of the sample.
///
/// `sample` is aliased by its return value.
#[no_mangle]
pub extern "C" fn z_sample_keyexpr(sample: &z_sample_t) -> z_keyexpr_t {
    sample.key_expr().into()
}
/// The encoding of the payload.
#[no_mangle]
pub extern "C" fn z_sample_encoding(sample: z_sample_t) -> z_encoding_t {
    // let encoding = sample.encoding();
    // sample.encoding().transmute()
}
/// The sample's data, the return value aliases the sample.
///
/// If you need ownership of the buffer, you may use `z_sample_owned_payload`.
#[no_mangle]
pub extern "C" fn z_sample_payload(sample: &z_sample_t) -> zc_payload_t {
    // TODO: here returning reference not to sample's payload, but to temporary copy
    // THIS WILL CRASH FOR SURE, MADE IT ONLY TO MAKE IT COMPILE
    // Need a way to get reference to sample's payload
    let buffer: ZBuf = ZSerde.deserialize(sample.payload()).unwrap_infallible();
    let owned_buffer: z_owned_buffer_t = Some(buffer).into();
    owned_buffer.as_ref().into()
}

/// Returns the sample's payload after incrementing its internal reference count.
///
/// Note that other samples may have received the same buffer, meaning that mutating this buffer may
/// affect the samples received by other subscribers.
#[no_mangle]
pub extern "C" fn z_sample_owned_payload(sample: &z_sample_t) -> zc_owned_payload_t {
    let buffer: ZBuf = ZSerde.deserialize(sample.payload()).unwrap_infallible();
    let owned_buffer: z_owned_buffer_t = Some(buffer).into();
    owned_buffer.into()
}
/// The sample's kind (put or delete).
#[no_mangle]
pub extern "C" fn z_sample_kind(sample: &z_sample_t) -> z_sample_kind_t {
    sample.kind().into()
}
/// The samples timestamp
#[no_mangle]
pub extern "C" fn z_sample_timestamp(sample: &z_sample_t) -> z_timestamp_t {
    sample.timestamp().into()
}
/// The qos with which the sample was received.
/// TODO: split to methods (priority, congestion_control, express)

/// The sample's attachment.
///
/// `sample` is aliased by the return value.
#[no_mangle]
pub extern "C" fn z_sample_attachment(sample: &z_sample_t) -> z_attachment_t {
    match sample.attachment() {
        Some(attachment) => z_attachment_t {
            data: attachment as *const _ as *mut c_void,
            iteration_driver: Some(attachment_iteration_driver),
        },
        None => z_attachment_null(),
    }
}

pub use crate::zc_owned_sample_t;
impl_guarded_transmute!(Option<Sample>, zc_owned_sample_t);

/// Clone a sample in the cheapest way available.
#[no_mangle]
pub extern "C" fn zc_sample_clone(sample: &z_sample_t) -> zc_owned_sample_t {
    Some(sample.deref().clone()).into()
}

/// Returns `true` if `sample` is valid.
///
/// Note that there exist no fallinle constructors for `zc_owned_sample_t`, so validity is always guaranteed
/// unless the value has been dropped already.
#[no_mangle]
pub extern "C" fn zc_sample_check(sample: &zc_owned_sample_t) -> bool {
    sample.is_some()
}

/// Borrow the sample, allowing calling its accessor methods.
///
/// Calling this function using a dropped sample is undefined behaviour.
#[no_mangle]
pub extern "C" fn zc_sample_loan(sample: &zc_owned_sample_t) -> z_sample_t {
    z_sample_t::new(unsafe { sample.as_ref().unwrap_unchecked() })
}

/// Destroy the sample.
#[no_mangle]
pub extern "C" fn zc_sample_drop(sample: &mut zc_owned_sample_t) {
    core::mem::drop(sample.take());
}

#[no_mangle]
pub extern "C" fn zc_sample_null() -> zc_owned_sample_t {
    None.into()
}

/// The encoding of a payload, in a MIME-like format.
///
/// For wire and matching efficiency, common MIME types are represented using an integer as `prefix`, and a `suffix` may be used to either provide more detail, or in combination with the `Empty` prefix to write arbitrary MIME types.
///
pub use crate::z_encoding_t;
decl_transmute_copy!(StaticRef<Encoding>, z_encoding_t);

/// An owned payload encoding.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
pub use crate::z_owned_encoding_t;
decl_transmute_ref!(default_inplace_init Encoding, z_owned_encoding_t);

/// Constructs a null safe-to-drop value of 'z_owned_encoding_t' type
#[no_mangle]
pub extern "C" fn z_encoding_null(encoding: *mut MaybeUninit<z_owned_encoding_t>) {
    Inplace::empty(encoding);
}

/// Constructs a specific :c:type:`z_encoding_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_from_str(
    encoding: *mut MaybeUninit<z_owned_encoding_t>,
    s: *const c_char,
) -> i8 {
    if s.is_null() {
        Inplace::empty(encoding);
        0
    } else {
        let s = CStr::from_ptr(s).to_string_lossy().as_ref();
        let value = Encoding::from_str(s).unwrap_infallible();
        Inplace::init(encoding, value);
        0
    }
}

/// Constructs a default :c:type:`z_encoding_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_encoding_default() -> z_encoding_t {
    StaticRef::new(&Encoding::ZENOH_BYTES).transmute()
}

/// Frees `encoding`, invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_drop(encoding: &mut z_owned_encoding_t) {}

/// Returns ``true`` if `encoding` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_encoding_check(encoding: &z_owned_encoding_t) -> bool {
    let encoding = encoding.deref();
    *encoding != Encoding::default()
}

/// Returns a :c:type:`z_encoding_t` loaned from `encoding`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_encoding_loan(encoding: &z_owned_encoding_t) -> z_encoding_t {
    let encoding = encoding.deref();
    encoding.into()
}

/// The wrapper type for null-terminated string values allocated by zenoh. The instances of `z_owned_str_t`
/// should be released with `z_drop` macro or with `z_str_drop` function and checked to validity with
/// `z_check` and `z_str_check` correspondently
#[repr(C)]
pub struct z_owned_str_t {
    pub _cstr: *mut libc::c_char,
}

impl z_owned_str_t {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn preallocate(len: usize) -> z_owned_str_t {
        let cstr = libc::malloc(len + 1) as *mut libc::c_char;
        *cstr.add(len) = 0;
        z_owned_str_t { _cstr: cstr }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn insert_unchecked(&mut self, start: usize, value: &[u8]) {
        std::ptr::copy_nonoverlapping(
            value.as_ptr(),
            (self._cstr as *mut u8).add(start),
            value.len(),
        );
    }
}

impl From<&[u8]> for z_owned_str_t {
    fn from(value: &[u8]) -> Self {
        unsafe {
            let mut cstr = Self::preallocate(value.len());
            cstr.insert_unchecked(0, value);
            cstr
        }
    }
}

impl Drop for z_owned_str_t {
    fn drop(&mut self) {
        unsafe { z_str_drop(self) }
    }
}

/// Frees `z_owned_str_t`, invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_str_drop(s: &mut z_owned_str_t) {
    if s._cstr.is_null() {
        return;
    }
    libc::free(std::mem::transmute(s._cstr));
    s._cstr = std::ptr::null_mut();
}

/// Returns ``true`` if `s` is a valid string
#[no_mangle]
pub extern "C" fn z_str_check(s: &z_owned_str_t) -> bool {
    !s._cstr.is_null()
}

/// Returns undefined `z_owned_str_t`
#[no_mangle]
pub extern "C" fn z_str_null() -> z_owned_str_t {
    z_owned_str_t {
        _cstr: std::ptr::null_mut(),
    }
}

/// Returns :c:type:`z_str_t` structure loaned from :c:type:`z_owned_str_t`.
#[no_mangle]
pub extern "C" fn z_str_loan(s: &z_owned_str_t) -> *const libc::c_char {
    s._cstr
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum zcu_locality_t {
    ANY = 0,
    SESSION_LOCAL = 1,
    REMOTE = 2,
}

impl From<Locality> for zcu_locality_t {
    fn from(k: Locality) -> Self {
        match k {
            Locality::Any => zcu_locality_t::ANY,
            Locality::SessionLocal => zcu_locality_t::SESSION_LOCAL,
            Locality::Remote => zcu_locality_t::REMOTE,
        }
    }
}

impl From<zcu_locality_t> for Locality {
    fn from(k: zcu_locality_t) -> Self {
        match k {
            zcu_locality_t::ANY => Locality::Any,
            zcu_locality_t::SESSION_LOCAL => Locality::SessionLocal,
            zcu_locality_t::REMOTE => Locality::Remote,
        }
    }
}

#[no_mangle]
pub extern "C" fn zcu_locality_default() -> zcu_locality_t {
    Locality::default().into()
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum zcu_reply_keyexpr_t {
    ANY = 0,
    MATCHING_QUERY = 1,
}

impl From<ReplyKeyExpr> for zcu_reply_keyexpr_t {
    fn from(k: ReplyKeyExpr) -> Self {
        match k {
            ReplyKeyExpr::Any => zcu_reply_keyexpr_t::ANY,
            ReplyKeyExpr::MatchingQuery => zcu_reply_keyexpr_t::MATCHING_QUERY,
        }
    }
}

impl From<zcu_reply_keyexpr_t> for ReplyKeyExpr {
    fn from(k: zcu_reply_keyexpr_t) -> Self {
        match k {
            zcu_reply_keyexpr_t::ANY => ReplyKeyExpr::Any,
            zcu_reply_keyexpr_t::MATCHING_QUERY => ReplyKeyExpr::MatchingQuery,
        }
    }
}

#[no_mangle]
pub extern "C" fn zcu_reply_keyexpr_default() -> zcu_reply_keyexpr_t {
    ReplyKeyExpr::default().into()
}
