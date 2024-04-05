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

use std::any::Any;
use std::ops::Deref;
use std::slice;

use crate::collections::*;
use crate::keyexpr::*;
use crate::z_congestion_control_t;
use crate::z_id_t;
use crate::z_priority_t;
use crate::{impl_guarded_transmute, GuardedTransmute};
use libc::c_void;
use libc::{c_char, c_ulong};
use zenoh::buffers::buffer::SplitBuffer;
use zenoh::buffers::ZBuf;
use zenoh::buffers::ZSliceBuffer;
use zenoh::prelude::SampleKind;
use zenoh::query::ReplyKeyExpr;
use zenoh::sample::Locality;
use zenoh::sample::QoS;
use zenoh::sample::Sample;
use zenoh_protocol::core::Timestamp;

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

/// An owned payload, backed by a reference counted owner.
///
/// The `payload` field may be modified, and Zenoh will take the new values into account.
#[allow(non_camel_case_types)]
pub type zc_owned_payload_t = z_owned_buffer_t;

/// Clones the `payload` by incrementing its reference counter.
#[no_mangle]
pub extern "C" fn zc_payload_rcinc(payload: &zc_owned_payload_t) -> zc_owned_payload_t {
    z_buffer_clone(z_buffer_loan(payload))
}
/// Returns `false` if `payload` is the gravestone value.
#[no_mangle]
pub extern "C" fn zc_payload_check(payload: &zc_owned_payload_t) -> bool {
    z_buffer_check(payload)
}
/// Decrements `payload`'s backing refcount, releasing the memory if appropriate.
#[no_mangle]
pub extern "C" fn zc_payload_drop(payload: &mut zc_owned_payload_t) {
    z_buffer_drop(payload)
}
/// Constructs `zc_owned_payload_t`'s gravestone value.
#[no_mangle]
pub extern "C" fn zc_payload_null() -> zc_owned_payload_t {
    z_buffer_null()
}

/// Returns a :c:type:`zc_payload_t` loaned from `payload`.
#[no_mangle]
pub extern "C" fn zc_payload_loan(payload: &zc_owned_payload_t) -> zc_payload_t {
    z_buffer_loan(payload)
}

#[allow(non_camel_case_types)]
pub type zc_payload_t = z_buffer_t;

/// Increments internal payload reference count, returning owned payload.
#[no_mangle]
pub extern "C" fn zc_payload_clone(payload: zc_payload_t) -> zc_owned_payload_t {
    z_buffer_clone(payload)
}

/// Decodes payload into null-terminated string
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_payload_decode_into_string(
    payload: zc_payload_t,
    cstr: &mut z_owned_str_t,
) -> i8 {
    let payload: Option<&ZBuf> = payload.into();
    if payload.is_none() {
        *cstr = z_str_null();
        return 0;
    }
    *cstr = z_owned_str_t::preallocate(zc_payload_len(payload.into()));
    let payload = payload.unwrap();

    let mut pos = 0;
    for s in payload.slices() {
        cstr.insert_unchecked(pos, s);
        pos += s.len();
    }
    0
}

/// Decodes payload into null-terminated string
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_payload_decode_into_bytes(
    payload: zc_payload_t,
    b: &mut z_owned_bytes_t,
) -> i8 {
    let payload: Option<&ZBuf> = payload.into();
    if payload.is_none() {
        *b = z_bytes_null();
        return 0;
    }
    *b = z_owned_bytes_t::preallocate(zc_payload_len(payload.into()));
    let payload = payload.unwrap();

    let mut pos = 0;
    for s in payload.slices() {
        b.insert_unchecked(pos, s);
        pos += s.len();
    }
    0
}

unsafe impl Send for z_bytes_t {}
unsafe impl Sync for z_bytes_t {}

impl ZSliceBuffer for z_bytes_t {
    fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.start, self.len) }
    }
    fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.start as *mut u8, self.len) }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Encodes byte sequence by aliasing.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_payload_encode_from_bytes(bytes: z_bytes_t) -> zc_owned_payload_t {
    ZBuf::from(bytes).into()
}

/// Encodes a null-terminated string by aliasing.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_payload_encode_from_string(
    cstr: *const libc::c_char,
) -> zc_owned_payload_t {
    let bytes = z_bytes_t {
        start: cstr as *const u8,
        len: libc::strlen(cstr),
    };
    zc_payload_encode_from_bytes(bytes)
}

/// Returns total number bytes in the payload.
#[no_mangle]
pub extern "C" fn zc_payload_len(payload: zc_payload_t) -> usize {
    z_buffer_len(payload)
}

/// QoS settings of zenoh message.
///
#[repr(C)]
pub struct z_qos_t(u8);

impl_guarded_transmute!(QoS, z_qos_t);

/// Returns message priority.
#[no_mangle]
pub extern "C" fn z_qos_get_priority(qos: z_qos_t) -> z_priority_t {
    qos.transmute().priority().into()
}
/// Returns message congestion control.
#[no_mangle]
pub extern "C" fn z_qos_get_congestion_control(qos: z_qos_t) -> z_congestion_control_t {
    qos.transmute().congestion_control().into()
}
/// Returns message express flag. If set to true, the message is not batched to reduce the latency.
#[no_mangle]
pub extern "C" fn z_qos_get_express(qos: z_qos_t) -> bool {
    qos.transmute().express()
}
/// Returns default qos settings.
#[no_mangle]
pub extern "C" fn z_qos_default() -> z_qos_t {
    QoS::default().transmute()
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
    (&sample.key_expr).into()
}
/// The encoding of the payload.
#[no_mangle]
pub extern "C" fn z_sample_encoding(sample: &z_sample_t) -> z_encoding_t {
    (&sample.encoding).into()
}
/// The sample's data, the return value aliases the sample.
///
/// If you need ownership of the buffer, you may use `z_sample_owned_payload`.
#[no_mangle]
pub extern "C" fn z_sample_payload(sample: &z_sample_t) -> z_buffer_t {
    Some(&sample.payload).into()
}
/// Returns the sample's payload after incrementing its internal reference count.
///
/// Note that other samples may have received the same buffer, meaning that mutating this buffer may
/// affect the samples received by other subscribers.
#[no_mangle]
pub extern "C" fn z_sample_owned_payload(sample: &z_sample_t) -> z_owned_buffer_t {
    sample.payload.clone().into()
}
/// The sample's kind (put or delete).
#[no_mangle]
pub extern "C" fn z_sample_kind(sample: &z_sample_t) -> z_sample_kind_t {
    sample.kind.into()
}
/// The samples timestamp
#[no_mangle]
pub extern "C" fn z_sample_timestamp(sample: &z_sample_t) -> z_timestamp_t {
    sample.timestamp.as_ref().into()
}
/// The qos with which the sample was received.
#[no_mangle]
pub extern "C" fn z_sample_qos(sample: &z_sample_t) -> z_qos_t {
    sample.qos.into()
}
/// The sample's attachment.
///
/// `sample` is aliased by the return value.
#[no_mangle]
pub extern "C" fn z_sample_attachment(sample: &z_sample_t) -> z_attachment_t {
    match &sample.attachment {
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

/// A :c:type:`z_encoding_t` integer `prefix`.
///
///     - **Z_ENCODING_PREFIX_EMPTY**
///     - **Z_ENCODING_PREFIX_APP_OCTET_STREAM**
///     - **Z_ENCODING_PREFIX_APP_CUSTOM**
///     - **Z_ENCODING_PREFIX_TEXT_PLAIN**
///     - **Z_ENCODING_PREFIX_APP_PROPERTIES**
///     - **Z_ENCODING_PREFIX_APP_JSON**
///     - **Z_ENCODING_PREFIX_APP_SQL**
///     - **Z_ENCODING_PREFIX_APP_INTEGER**
///     - **Z_ENCODING_PREFIX_APP_FLOAT**
///     - **Z_ENCODING_PREFIX_APP_XML**
///     - **Z_ENCODING_PREFIX_APP_XHTML_XML**
///     - **Z_ENCODING_PREFIX_APP_X_WWW_FORM_URLENCODED**
///     - **Z_ENCODING_PREFIX_TEXT_JSON**
///     - **Z_ENCODING_PREFIX_TEXT_HTML**
///     - **Z_ENCODING_PREFIX_TEXT_XML**
///     - **Z_ENCODING_PREFIX_TEXT_CSS**
///     - **Z_ENCODING_PREFIX_TEXT_CSV**
///     - **Z_ENCODING_PREFIX_TEXT_JAVASCRIPT**
///     - **Z_ENCODING_PREFIX_IMAGE_JPEG**
///     - **Z_ENCODING_PREFIX_IMAGE_PNG**
///     - **Z_ENCODING_PREFIX_IMAGE_GIF**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum z_encoding_prefix_t {
    Empty = 0,
    AppOctetStream = 1,
    AppCustom = 2,
    TextPlain = 3,
    AppProperties = 4,
    AppJson = 5,
    AppSql = 6,
    AppInteger = 7,
    AppFloat = 8,
    AppXml = 9,
    AppXhtmlXml = 10,
    AppXWwwFormUrlencoded = 11,
    TextJson = 12,
    TextHtml = 13,
    TextXml = 14,
    TextCss = 15,
    TextCsv = 16,
    TextJavascript = 17,
    ImageJpeg = 18,
    ImagePng = 19,
    ImageGif = 20,
}

impl From<z_encoding_prefix_t> for zenoh_protocol::core::KnownEncoding {
    fn from(val: z_encoding_prefix_t) -> Self {
        if cfg!(debug_assertions) {
            match val {
                z_encoding_prefix_t::Empty => zenoh_protocol::core::KnownEncoding::Empty,
                z_encoding_prefix_t::AppOctetStream => {
                    zenoh_protocol::core::KnownEncoding::AppOctetStream
                }
                z_encoding_prefix_t::AppCustom => zenoh_protocol::core::KnownEncoding::AppCustom,
                z_encoding_prefix_t::TextPlain => zenoh_protocol::core::KnownEncoding::TextPlain,
                z_encoding_prefix_t::AppProperties => {
                    zenoh_protocol::core::KnownEncoding::AppProperties
                }
                z_encoding_prefix_t::AppJson => zenoh_protocol::core::KnownEncoding::AppJson,
                z_encoding_prefix_t::AppSql => zenoh_protocol::core::KnownEncoding::AppSql,
                z_encoding_prefix_t::AppInteger => zenoh_protocol::core::KnownEncoding::AppInteger,
                z_encoding_prefix_t::AppFloat => zenoh_protocol::core::KnownEncoding::AppFloat,
                z_encoding_prefix_t::AppXml => zenoh_protocol::core::KnownEncoding::AppXml,
                z_encoding_prefix_t::AppXhtmlXml => {
                    zenoh_protocol::core::KnownEncoding::AppXhtmlXml
                }
                z_encoding_prefix_t::AppXWwwFormUrlencoded => {
                    zenoh_protocol::core::KnownEncoding::AppXWwwFormUrlencoded
                }
                z_encoding_prefix_t::TextJson => zenoh_protocol::core::KnownEncoding::TextJson,
                z_encoding_prefix_t::TextHtml => zenoh_protocol::core::KnownEncoding::TextHtml,
                z_encoding_prefix_t::TextXml => zenoh_protocol::core::KnownEncoding::TextXml,
                z_encoding_prefix_t::TextCss => zenoh_protocol::core::KnownEncoding::TextCss,
                z_encoding_prefix_t::TextCsv => zenoh_protocol::core::KnownEncoding::TextCsv,
                z_encoding_prefix_t::TextJavascript => {
                    zenoh_protocol::core::KnownEncoding::TextJavascript
                }
                z_encoding_prefix_t::ImageJpeg => zenoh_protocol::core::KnownEncoding::ImageJpeg,
                z_encoding_prefix_t::ImagePng => zenoh_protocol::core::KnownEncoding::ImagePng,
                z_encoding_prefix_t::ImageGif => zenoh_protocol::core::KnownEncoding::ImageGif,
            }
        } else {
            unsafe { std::mem::transmute(val as u8) }
        }
    }
}

impl From<zenoh_protocol::core::KnownEncoding> for z_encoding_prefix_t {
    fn from(val: zenoh_protocol::core::KnownEncoding) -> Self {
        if cfg!(debug_assertions) {
            match val {
                zenoh_protocol::core::KnownEncoding::Empty => z_encoding_prefix_t::Empty,
                zenoh_protocol::core::KnownEncoding::AppOctetStream => {
                    z_encoding_prefix_t::AppOctetStream
                }
                zenoh_protocol::core::KnownEncoding::AppCustom => z_encoding_prefix_t::AppCustom,
                zenoh_protocol::core::KnownEncoding::TextPlain => z_encoding_prefix_t::TextPlain,
                zenoh_protocol::core::KnownEncoding::AppProperties => {
                    z_encoding_prefix_t::AppProperties
                }
                zenoh_protocol::core::KnownEncoding::AppJson => z_encoding_prefix_t::AppJson,
                zenoh_protocol::core::KnownEncoding::AppSql => z_encoding_prefix_t::AppSql,
                zenoh_protocol::core::KnownEncoding::AppInteger => z_encoding_prefix_t::AppInteger,
                zenoh_protocol::core::KnownEncoding::AppFloat => z_encoding_prefix_t::AppFloat,
                zenoh_protocol::core::KnownEncoding::AppXml => z_encoding_prefix_t::AppXml,
                zenoh_protocol::core::KnownEncoding::AppXhtmlXml => {
                    z_encoding_prefix_t::AppXhtmlXml
                }
                zenoh_protocol::core::KnownEncoding::AppXWwwFormUrlencoded => {
                    z_encoding_prefix_t::AppXWwwFormUrlencoded
                }
                zenoh_protocol::core::KnownEncoding::TextJson => z_encoding_prefix_t::TextJson,
                zenoh_protocol::core::KnownEncoding::TextHtml => z_encoding_prefix_t::TextHtml,
                zenoh_protocol::core::KnownEncoding::TextXml => z_encoding_prefix_t::TextXml,
                zenoh_protocol::core::KnownEncoding::TextCss => z_encoding_prefix_t::TextCss,
                zenoh_protocol::core::KnownEncoding::TextCsv => z_encoding_prefix_t::TextCsv,
                zenoh_protocol::core::KnownEncoding::TextJavascript => {
                    z_encoding_prefix_t::TextJavascript
                }
                zenoh_protocol::core::KnownEncoding::ImageJpeg => z_encoding_prefix_t::ImageJpeg,
                zenoh_protocol::core::KnownEncoding::ImagePng => z_encoding_prefix_t::ImagePng,
                zenoh_protocol::core::KnownEncoding::ImageGif => z_encoding_prefix_t::ImageGif,
            }
        } else {
            unsafe { std::mem::transmute(val as u32) }
        }
    }
}

/// The encoding of a payload, in a MIME-like format.
///
/// For wire and matching efficiency, common MIME types are represented using an integer as `prefix`, and a `suffix` may be used to either provide more detail, or in combination with the `Empty` prefix to write arbitrary MIME types.
///
/// Members:
///   z_encoding_prefix_t prefix: The integer prefix of this encoding.
///   z_bytes_t suffix: The suffix of this encoding. `suffix` MUST be a valid UTF-8 string.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct z_encoding_t {
    pub prefix: z_encoding_prefix_t,
    pub suffix: z_bytes_t,
}

impl From<z_encoding_t> for zenoh_protocol::core::Encoding {
    fn from(enc: z_encoding_t) -> Self {
        if enc.suffix.len == 0 {
            zenoh_protocol::core::Encoding::Exact(enc.prefix.into())
        } else {
            let suffix = unsafe {
                let slice: &'static [u8] =
                    std::slice::from_raw_parts(enc.suffix.start, enc.suffix.len);
                std::str::from_utf8_unchecked(slice)
            };
            zenoh_protocol::core::Encoding::WithSuffix(enc.prefix.into(), suffix.into())
        }
    }
}

impl From<&zenoh_protocol::core::Encoding> for z_encoding_t {
    fn from(val: &zenoh_protocol::core::Encoding) -> Self {
        let suffix = val.suffix();
        z_encoding_t {
            prefix: (*val.prefix()).into(),
            suffix: z_bytes_t {
                start: suffix.as_ptr(),
                len: suffix.len(),
            },
        }
    }
}

/// An owned payload encoding.
///
/// Members:
///   z_encoding_prefix_t prefix: The integer prefix of this encoding.
///   z_bytes_t suffix: The suffix of this encoding. `suffix` MUST be a valid UTF-8 string.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[repr(C)]
pub struct z_owned_encoding_t {
    pub prefix: z_encoding_prefix_t,
    pub suffix: z_owned_bytes_t,
    pub _dropped: bool,
}

impl z_owned_encoding_t {
    pub fn null() -> Self {
        z_owned_encoding_t {
            prefix: z_encoding_prefix_t::Empty,
            suffix: z_bytes_null(),
            _dropped: true,
        }
    }
}

/// Constructs a null safe-to-drop value of 'z_owned_encoding_t' type
#[no_mangle]
pub extern "C" fn z_encoding_null() -> z_owned_encoding_t {
    z_owned_encoding_t::null()
}

/// Constructs a specific :c:type:`z_encoding_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding(
    prefix: z_encoding_prefix_t,
    suffix: *const c_char,
) -> z_encoding_t {
    let suffix = if suffix.is_null() {
        z_bytes_t::empty()
    } else {
        z_bytes_t {
            start: suffix as *const u8,
            len: libc::strlen(suffix),
        }
    };
    z_encoding_t { prefix, suffix }
}

/// Constructs a default :c:type:`z_encoding_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_encoding_default() -> z_encoding_t {
    (&zenoh_protocol::core::Encoding::default()).into()
}

/// Frees `encoding`, invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_drop(encoding: &mut z_owned_encoding_t) {
    z_bytes_drop(&mut encoding.suffix);
    encoding._dropped = true
}

/// Returns ``true`` if `encoding` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_encoding_check(encoding: &z_owned_encoding_t) -> bool {
    !encoding._dropped
}

/// Returns a :c:type:`z_encoding_t` loaned from `encoding`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_encoding_loan(encoding: &z_owned_encoding_t) -> z_encoding_t {
    z_encoding_t {
        prefix: encoding.prefix,
        suffix: z_bytes_loan(&encoding.suffix),
    }
}

impl From<z_encoding_t> for z_owned_encoding_t {
    fn from(val: z_encoding_t) -> Self {
        z_owned_encoding_t {
            prefix: val.prefix,
            suffix: z_bytes_clone(&val.suffix),
            _dropped: false,
        }
    }
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
