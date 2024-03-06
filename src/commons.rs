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

use crate::collections::*;
use crate::keyexpr::*;
use crate::z_congestion_control_t;
use crate::z_id_t;
use crate::z_priority_t;
use crate::{impl_guarded_transmute, GuardedTransmute};
use libc::c_void;
use libc::{c_char, c_ulong};
use zenoh::buffers::ZBuf;
use zenoh::prelude::SampleKind;
use zenoh::prelude::SplitBuffer;
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
/// tags{c.z_sample_kind_t, api.options.sample.kind}
pub enum z_sample_kind_t {
    /// tags{c.z_sample_kind_t.put, api.options.sample.kind.put}
    PUT = 0,
    /// tags{c.z_sample_kind_t.delete, api.options.sample.kind.delete}
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
/// tags{c.z_timestamp_t, api.timestamp}
pub struct z_timestamp_t {
    time: u64,
    id: z_id_t,
}

/// Returns ``true`` if `ts` is a valid timestamp
#[no_mangle]
/// tags{}
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
/// The `payload` field may be modified, and Zenoh will take the new values into account,
/// however, assuming `ostart` and `olen` are the respective values of `payload.start` and
/// `payload.len` when constructing the `zc_owned_payload_t payload` value was created,
/// then `payload.start` MUST remain within the `[ostart, ostart + olen[` interval, and
/// `payload.len` must remain within `[0, olen -(payload.start - ostart)]`.
///
/// Should this invariant be broken when the payload is passed to one of zenoh's `put_owned`
/// functions, then the operation will fail (but the passed value will still be consumed).
#[allow(non_camel_case_types)]
#[repr(C)]
/// tags{c.zc_owned_payload_t}
pub struct zc_owned_payload_t {
    pub payload: z_bytes_t,
    pub _owner: [usize; 5],
}
impl Default for zc_owned_payload_t {
    // tags{}
    fn default() -> Self {
        zc_payload_null()
    }
}
impl TryFrom<ZBuf> for zc_owned_payload_t {
    type Error = ();
    fn try_from(buf: ZBuf) -> Result<Self, Self::Error> {
        let std::borrow::Cow::Borrowed(payload) = buf.contiguous() else {
            return Err(());
        };
        Ok(Self {
            payload: payload.into(),
            _owner: unsafe { std::mem::transmute(buf) },
        })
    }
}
impl zc_owned_payload_t {
    // tags{}
    pub fn take(&mut self) -> Option<ZBuf> {
        if !z_bytes_check(&self.payload) {
            return None;
        }
        let start = std::mem::replace(&mut self.payload.start, std::ptr::null());
        let len = std::mem::replace(&mut self.payload.len, 0);
        let mut buf: ZBuf = unsafe { std::mem::transmute(self._owner) };
        {
            let mut slices = buf.zslices_mut();
            let slice = slices.next().unwrap();
            assert!(
                slices.next().is_none(),
                "A multi-slice buffer reached zenoh-c, which is definitely a bug, please report it."
            );
            let start_offset = unsafe { start.offset_from(slice.as_slice().as_ptr()) };
            let Ok(start_offset) = start_offset.try_into() else {
                return None;
            };
            *slice = match slice.subslice(start_offset, start_offset + len) {
                Some(s) => s,
                None => return None,
            };
        }
        Some(buf)
    }
    fn owner(&self) -> Option<&ZBuf> {
        if !z_bytes_check(&self.payload) {
            return None;
        }
        unsafe { std::mem::transmute(&self._owner) }
    }
}
impl Drop for zc_owned_payload_t {
    fn drop(&mut self) {
        self.take();
    }
}

/// Clones the `payload` by incrementing its reference counter.
/// tags{c.zc_payload_rcinc, api.buffer.rcinc}
#[no_mangle]
pub extern "C" fn zc_payload_rcinc(payload: &zc_owned_payload_t) -> zc_owned_payload_t {
    match payload.owner() {
        None => Default::default(),
        Some(payload) => payload.clone().try_into().unwrap_or_default(),
    }
}
/// Returns `false` if `payload` is the gravestone value.
/// tags{}
#[no_mangle]
pub extern "C" fn zc_payload_check(payload: &zc_owned_payload_t) -> bool {
    !payload.payload.start.is_null()
}
/// Decrements `payload`'s backing refcount, releasing the memory if appropriate.
/// tags{}
#[no_mangle]
pub extern "C" fn zc_payload_drop(payload: &mut zc_owned_payload_t) {
    unsafe { std::ptr::replace(payload, zc_payload_null()) };
}
/// Constructs `zc_owned_payload_t`'s gravestone value.
/// tags{}
#[no_mangle]
pub extern "C" fn zc_payload_null() -> zc_owned_payload_t {
    zc_owned_payload_t {
        payload: z_bytes_t {
            len: 0,
            start: std::ptr::null(),
        },
        _owner: unsafe { core::mem::MaybeUninit::zeroed().assume_init() },
    }
}

/// QoS settings of zenoh message.
///
#[repr(C)]
pub struct z_qos_t(u8);

impl_guarded_transmute!(QoS, z_qos_t);
impl_guarded_transmute!(z_qos_t, QoS);

impl From<QoS> for z_qos_t {
    fn from(qos: QoS) -> Self {
        qos.transmute()
    }
}

impl From<z_qos_t> for QoS {
    fn from(qos: z_qos_t) -> QoS {
        qos.transmute()
    }
}

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
///
/// Members:
///   z_keyexpr_t keyexpr: The resource key of this data sample.
///   z_bytes_t payload: The value of this data sample.
///   z_encoding_t encoding: The encoding of the value of this data sample.
///   z_sample_kind_t kind: The kind of this data sample (PUT or DELETE).
///   z_timestamp_t timestamp: The timestamp of this data sample.
///   z_attachment_t attachment: The attachment of this data sample.
/// tags{c.z_sample_t, api.sample}
#[repr(C)]
pub struct z_sample_t<'a> {
    /// tags{c.z_sample_t.keyexpr, api.sample.keyexpr}
    pub keyexpr: z_keyexpr_t,
    /// tags{c.z_sample_t.payload, api.sample.payload}
    pub payload: z_bytes_t,
    /// tags{c.z_sample_t.encoding, api.sample.encoding}
    pub encoding: z_encoding_t,
    pub _zc_buf: &'a c_void,
    /// tags{c.z_sample_t.kind, api.sample.kind}
    pub kind: z_sample_kind_t,
    /// tags{c.z_sample_t.timestamp, api.sample.timestamp}
    pub timestamp: z_timestamp_t,
    /// tags{c.z_sample_t.qos, api.sample.qos}
    pub qos: z_qos_t,
    /// tags{c.z_sample_t.attachment, api.sample.attachment}
    pub attachment: z_attachment_t,
}

impl<'a> z_sample_t<'a> {
    // tags{}
    pub fn new(sample: &'a Sample, owner: &'a ZBuf) -> Self {
        let std::borrow::Cow::Borrowed(payload) = owner.contiguous() else {
            panic!("Attempted to construct z_sample_t from discontiguous buffer, this is definitely a bug in zenoh-c, please report it.")
        };
        z_sample_t {
            keyexpr: (&sample.key_expr).into(),
            payload: z_bytes_t::from(payload),
            encoding: (&sample.encoding).into(),
            _zc_buf: unsafe { std::mem::transmute(owner) },
            kind: sample.kind.into(),
            timestamp: sample.timestamp.as_ref().into(),
            qos: sample.qos.into(),
            attachment: match &sample.attachment {
                Some(attachment) => z_attachment_t {
                    data: attachment as *const _ as *mut c_void,
                    iteration_driver: Some(attachment_iteration_driver),
                },
                None => z_attachment_null(),
            },
        }
    }
}

/// Clones the sample's payload by incrementing its backing refcount (this doesn't imply any copies).
/// tags{c.zc_sample_payload_rcinc, api.sample.payload.rcinc}
#[no_mangle]
pub extern "C" fn zc_sample_payload_rcinc(sample: Option<&z_sample_t>) -> zc_owned_payload_t {
    let Some(sample) = sample else {
        return zc_payload_null();
    };
    let buf = unsafe { std::mem::transmute::<_, &ZBuf>(sample._zc_buf).clone() };
    zc_owned_payload_t {
        payload: sample.payload,
        _owner: unsafe { std::mem::transmute(buf) },
    }
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
/// tags{c.z_encoding_prefix_t, api.encoding_prefix}
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
/// tags{c.z_encoding_t, api.encoding}
pub struct z_encoding_t {
    /// tags{c.z_encoding_t.prefix, api.encoding.prefix.get}
    pub prefix: z_encoding_prefix_t,
    /// tags{c.z_encoding_t.suffix, api.encoding.suffix.get}
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
/// tags{c.z_owned_encoding_t, api.encoding}
pub struct z_owned_encoding_t {
    /// tags{c.z_owned_encoding_t.prefix, api.encoding.prefix}
    pub prefix: z_encoding_prefix_t,
    /// tags{c.z_owned_encoding_t.suffix, api.encoding.suffix}
    pub suffix: z_bytes_t,
    pub _dropped: bool,
}

impl z_owned_encoding_t {
    // tags{}
    pub fn null() -> Self {
        z_owned_encoding_t {
            prefix: z_encoding_prefix_t::Empty,
            suffix: z_bytes_t::default(),
            _dropped: true,
        }
    }
}

/// Constructs a null safe-to-drop value of 'z_owned_encoding_t' type
#[no_mangle]
/// tags{}
pub extern "C" fn z_encoding_null() -> z_owned_encoding_t {
    z_owned_encoding_t::null()
}

/// Constructs a specific :c:type:`z_encoding_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
/// tags{c.z_encoding, api.encoding.create}
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
/// tags{c.z_encoding_default, api.encoding.create}
pub extern "C" fn z_encoding_default() -> z_encoding_t {
    (&zenoh_protocol::core::Encoding::default()).into()
}

/// Frees `encoding`, invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
/// tags{}
pub unsafe extern "C" fn z_encoding_drop(encoding: &mut z_owned_encoding_t) {
    z_bytes_drop(&mut encoding.suffix);
    encoding._dropped = true
}

/// Returns ``true`` if `encoding` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
/// tags{}
pub extern "C" fn z_encoding_check(encoding: &z_owned_encoding_t) -> bool {
    !encoding._dropped
}

/// Returns a :c:type:`z_encoding_t` loaned from `encoding`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
/// tags{}
pub extern "C" fn z_encoding_loan(encoding: &z_owned_encoding_t) -> z_encoding_t {
    z_encoding_t {
        prefix: encoding.prefix,
        suffix: encoding.suffix,
    }
}

impl From<z_encoding_t> for z_owned_encoding_t {
    fn from(val: z_encoding_t) -> Self {
        z_owned_encoding_t {
            prefix: val.prefix,
            suffix: val.suffix,
            _dropped: false,
        }
    }
}

/// The wrapper type for null-terminated string values allocated by zenoh. The instances of `z_owned_str_t`
/// should be released with `z_drop` macro or with `z_str_drop` function and checked to validity with
/// `z_check` and `z_str_check` correspondently
#[repr(C)]
/// tags{c.z_owned_str_t}
pub struct z_owned_str_t {
    pub _cstr: *mut libc::c_char,
}

impl From<&[u8]> for z_owned_str_t {
    // tags{}
    fn from(value: &[u8]) -> Self {
        unsafe {
            let cstr = libc::malloc(value.len() + 1) as *mut libc::c_char;
            std::ptr::copy_nonoverlapping(value.as_ptr(), cstr as _, value.len());
            *cstr.add(value.len()) = 0;
            z_owned_str_t { _cstr: cstr }
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
/// tags{}
pub unsafe extern "C" fn z_str_drop(s: &mut z_owned_str_t) {
    libc::free(std::mem::transmute(s._cstr));
    s._cstr = std::ptr::null_mut();
}

/// Returns ``true`` if `s` is a valid string
#[no_mangle]
/// tags{}
pub extern "C" fn z_str_check(s: &z_owned_str_t) -> bool {
    !s._cstr.is_null()
}

/// Returns undefined `z_owned_str_t`
#[no_mangle]
/// tags{}
pub extern "C" fn z_str_null() -> z_owned_str_t {
    z_owned_str_t {
        _cstr: std::ptr::null_mut(),
    }
}

/// Returns :c:type:`z_str_t` structure loaned from :c:type:`z_owned_str_t`.
#[no_mangle]
/// tags{}
pub extern "C" fn z_str_loan(s: &z_owned_str_t) -> *const libc::c_char {
    s._cstr
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// tags{c.zcu_locality_t, api.options.locality}
pub enum zcu_locality_t {
    /// tags{c.zcu_locality_t.any, api.options.locality.any}
    ANY = 0,
    /// tags{c.zcu_locality_t.session_local, api.options.locality.session_local}
    SESSION_LOCAL = 1,
    /// tags{c.zcu_locality_t.remote, api.options.locality.remote}
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
/// tags{c.zcu_locality_default, api.options.locality.default}
pub extern "C" fn zcu_locality_default() -> zcu_locality_t {
    Locality::default().into()
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// tags{c.zcu_reply_keyexpr_t, api.options.reply_key_expr}
pub enum zcu_reply_keyexpr_t {
    /// tags{c.zcu_reply_keyexpr_t.any, api.options.reply_key_expr.any}
    ANY = 0,
    /// tags{c.zcu_reply_keyexpr_t.matching_query, api.options.reply_key_expr.matching_query}
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
/// tags{c.zcu_reply_keyexpr_default, api.options.reply_key_expr.default}
pub extern "C" fn zcu_reply_keyexpr_default() -> zcu_reply_keyexpr_t {
    ReplyKeyExpr::default().into()
}
