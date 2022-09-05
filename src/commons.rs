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
use libc::{c_char, c_ulong};
use zenoh::prelude::SampleKind;
use zenoh_protocol_core::Timestamp;

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
    id: z_bytes_t,
}

/// Returns ``true`` if `ts` is a valid timestamp
#[no_mangle]
pub extern "C" fn z_timestamp_check(ts: z_timestamp_t) -> bool {
    let id = unsafe { std::slice::from_raw_parts(ts.id.start, ts.id.len) };
    id.iter().any(|byte| *byte != 0)
}
impl From<Option<&Timestamp>> for z_timestamp_t {
    fn from(ts: Option<&Timestamp>) -> Self {
        if let Some(ts) = ts {
            let id = ts.get_id().as_slice();
            z_timestamp_t {
                time: ts.get_time().as_u64(),
                id: z_bytes_t {
                    start: id.as_ptr(),
                    len: id.len(),
                },
            }
        } else {
            z_timestamp_t {
                time: 0,
                id: z_bytes_t {
                    start: std::ptr::null(),
                    len: 0,
                },
            }
        }
    }
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
#[repr(C)]
pub struct z_sample_t {
    pub keyexpr: z_keyexpr_t,
    pub payload: z_bytes_t,
    pub encoding: z_encoding_t,
    pub kind: z_sample_kind_t,
    pub timestamp: z_timestamp_t,
}

/// A :c:type:`z_encoding_t` integer `prefix`.
///
///     - **Z_ENCODING_EMPTY**
///     - **Z_ENCODING_APP_OCTET_STREAM**
///     - **Z_ENCODING_APP_CUSTOM**
///     - **Z_ENCODING_TEXT_PLAIN**
///     - **Z_ENCODING_APP_PROPERTIES**
///     - **Z_ENCODING_APP_JSON**
///     - **Z_ENCODING_APP_SQL**
///     - **Z_ENCODING_APP_INTEGER**
///     - **Z_ENCODING_APP_FLOAT**
///     - **Z_ENCODING_APP_XML**
///     - **Z_ENCODING_APP_XHTML_XML**
///     - **Z_ENCODING_APP_X_WWW_FORM_URLENCODED**
///     - **Z_ENCODING_TEXT_JSON**
///     - **Z_ENCODING_TEXT_HTML**
///     - **Z_ENCODING_TEXT_XML**
///     - **Z_ENCODING_TEXT_CSS**
///     - **Z_ENCODING_TEXT_CSV**
///     - **Z_ENCODING_TEXT_JAVASCRIPT**
///     - **Z_ENCODING_IMAGE_JPEG**
///     - **Z_ENCODING_IMAGE_PNG**
///     - **Z_ENCODING_IMAGE_GIF**
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

impl From<z_encoding_prefix_t> for zenoh_protocol_core::KnownEncoding {
    fn from(val: z_encoding_prefix_t) -> Self {
        if cfg!(debug_assertions) {
            match val {
                z_encoding_prefix_t::Empty => zenoh_protocol_core::KnownEncoding::Empty,
                z_encoding_prefix_t::AppOctetStream => {
                    zenoh_protocol_core::KnownEncoding::AppOctetStream
                }
                z_encoding_prefix_t::AppCustom => zenoh_protocol_core::KnownEncoding::AppCustom,
                z_encoding_prefix_t::TextPlain => zenoh_protocol_core::KnownEncoding::TextPlain,
                z_encoding_prefix_t::AppProperties => {
                    zenoh_protocol_core::KnownEncoding::AppProperties
                }
                z_encoding_prefix_t::AppJson => zenoh_protocol_core::KnownEncoding::AppJson,
                z_encoding_prefix_t::AppSql => zenoh_protocol_core::KnownEncoding::AppSql,
                z_encoding_prefix_t::AppInteger => zenoh_protocol_core::KnownEncoding::AppInteger,
                z_encoding_prefix_t::AppFloat => zenoh_protocol_core::KnownEncoding::AppFloat,
                z_encoding_prefix_t::AppXml => zenoh_protocol_core::KnownEncoding::AppXml,
                z_encoding_prefix_t::AppXhtmlXml => zenoh_protocol_core::KnownEncoding::AppXhtmlXml,
                z_encoding_prefix_t::AppXWwwFormUrlencoded => {
                    zenoh_protocol_core::KnownEncoding::AppXWwwFormUrlencoded
                }
                z_encoding_prefix_t::TextJson => zenoh_protocol_core::KnownEncoding::TextJson,
                z_encoding_prefix_t::TextHtml => zenoh_protocol_core::KnownEncoding::TextHtml,
                z_encoding_prefix_t::TextXml => zenoh_protocol_core::KnownEncoding::TextXml,
                z_encoding_prefix_t::TextCss => zenoh_protocol_core::KnownEncoding::TextCss,
                z_encoding_prefix_t::TextCsv => zenoh_protocol_core::KnownEncoding::TextCsv,
                z_encoding_prefix_t::TextJavascript => {
                    zenoh_protocol_core::KnownEncoding::TextJavascript
                }
                z_encoding_prefix_t::ImageJpeg => zenoh_protocol_core::KnownEncoding::ImageJpeg,
                z_encoding_prefix_t::ImagePng => zenoh_protocol_core::KnownEncoding::ImagePng,
                z_encoding_prefix_t::ImageGif => zenoh_protocol_core::KnownEncoding::ImageGif,
            }
        } else {
            unsafe { std::mem::transmute(val as u8) }
        }
    }
}

impl From<zenoh_protocol_core::KnownEncoding> for z_encoding_prefix_t {
    fn from(val: zenoh_protocol_core::KnownEncoding) -> Self {
        if cfg!(debug_assertions) {
            match val {
                zenoh_protocol_core::KnownEncoding::Empty => z_encoding_prefix_t::Empty,
                zenoh_protocol_core::KnownEncoding::AppOctetStream => {
                    z_encoding_prefix_t::AppOctetStream
                }
                zenoh_protocol_core::KnownEncoding::AppCustom => z_encoding_prefix_t::AppCustom,
                zenoh_protocol_core::KnownEncoding::TextPlain => z_encoding_prefix_t::TextPlain,
                zenoh_protocol_core::KnownEncoding::AppProperties => {
                    z_encoding_prefix_t::AppProperties
                }
                zenoh_protocol_core::KnownEncoding::AppJson => z_encoding_prefix_t::AppJson,
                zenoh_protocol_core::KnownEncoding::AppSql => z_encoding_prefix_t::AppSql,
                zenoh_protocol_core::KnownEncoding::AppInteger => z_encoding_prefix_t::AppInteger,
                zenoh_protocol_core::KnownEncoding::AppFloat => z_encoding_prefix_t::AppFloat,
                zenoh_protocol_core::KnownEncoding::AppXml => z_encoding_prefix_t::AppXml,
                zenoh_protocol_core::KnownEncoding::AppXhtmlXml => z_encoding_prefix_t::AppXhtmlXml,
                zenoh_protocol_core::KnownEncoding::AppXWwwFormUrlencoded => {
                    z_encoding_prefix_t::AppXWwwFormUrlencoded
                }
                zenoh_protocol_core::KnownEncoding::TextJson => z_encoding_prefix_t::TextJson,
                zenoh_protocol_core::KnownEncoding::TextHtml => z_encoding_prefix_t::TextHtml,
                zenoh_protocol_core::KnownEncoding::TextXml => z_encoding_prefix_t::TextXml,
                zenoh_protocol_core::KnownEncoding::TextCss => z_encoding_prefix_t::TextCss,
                zenoh_protocol_core::KnownEncoding::TextCsv => z_encoding_prefix_t::TextCsv,
                zenoh_protocol_core::KnownEncoding::TextJavascript => {
                    z_encoding_prefix_t::TextJavascript
                }
                zenoh_protocol_core::KnownEncoding::ImageJpeg => z_encoding_prefix_t::ImageJpeg,
                zenoh_protocol_core::KnownEncoding::ImagePng => z_encoding_prefix_t::ImagePng,
                zenoh_protocol_core::KnownEncoding::ImageGif => z_encoding_prefix_t::ImageGif,
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

impl From<z_encoding_t> for zenoh_protocol_core::Encoding {
    fn from(enc: z_encoding_t) -> Self {
        if enc.suffix.len == 0 {
            zenoh_protocol_core::Encoding::Exact(enc.prefix.into())
        } else {
            let suffix = unsafe {
                let slice: &'static [u8] =
                    std::slice::from_raw_parts(enc.suffix.start, enc.suffix.len);
                std::str::from_utf8_unchecked(slice)
            };
            zenoh_protocol_core::Encoding::WithSuffix(enc.prefix.into(), suffix.into())
        }
    }
}

impl From<&zenoh_protocol_core::Encoding> for z_encoding_t {
    fn from(val: &zenoh_protocol_core::Encoding) -> Self {
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
    pub suffix: z_bytes_t,
    pub _dropped: bool,
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
pub unsafe extern "C" fn z_encoding_default() -> z_encoding_t {
    (&zenoh_protocol_core::Encoding::default()).into()
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
pub unsafe extern "C" fn z_encoding_check(encoding: &z_owned_encoding_t) -> bool {
    !encoding._dropped
}

/// Returns a :c:type:`z_encoding_t` loaned from `encoding`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_loan(encoding: &z_owned_encoding_t) -> z_encoding_t {
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
