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
use libc::c_ulong;
use zenoh::prelude::SampleKind;
use zenoh_protocol_core::Timestamp;

/// A zenoh unsigned integer
#[allow(non_camel_case_types)]
pub type z_zint_t = c_ulong;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum z_sample_kind {
    PUT = 0,
    DELETE = 1,
}

impl From<SampleKind> for z_sample_kind {
    fn from(k: SampleKind) -> Self {
        match k {
            SampleKind::Put => z_sample_kind::PUT,
            SampleKind::Delete => z_sample_kind::DELETE,
        }
    }
}

impl From<z_sample_kind> for SampleKind {
    fn from(k: z_sample_kind) -> Self {
        match k {
            z_sample_kind::PUT => SampleKind::Put,
            z_sample_kind::DELETE => SampleKind::Delete,
        }
    }
}

#[repr(C)]
pub struct z_timestamp_t {
    time: u64,
    id: z_bytes_t,
}

/// Returns `true` if `ts` is a valid timestamp
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
///   `z_string_t key`: The resource key of this data sample.
///   `z_bytes_t value`: The value of this data sample.
#[repr(C)]
pub struct z_sample_t {
    pub keyexpr: z_keyexpr_t,
    pub payload: z_bytes_t,
    pub encoding: z_encoding_t,
    pub kind: z_sample_kind,
    pub timestamp: z_timestamp_t,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum z_encoding_prefix {
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

impl From<z_encoding_prefix> for zenoh_protocol_core::KnownEncoding {
    fn from(val: z_encoding_prefix) -> Self {
        if cfg!(debug_assertions) {
            match val {
                z_encoding_prefix::Empty => zenoh_protocol_core::KnownEncoding::Empty,
                z_encoding_prefix::AppOctetStream => {
                    zenoh_protocol_core::KnownEncoding::AppOctetStream
                }
                z_encoding_prefix::AppCustom => zenoh_protocol_core::KnownEncoding::AppCustom,
                z_encoding_prefix::TextPlain => zenoh_protocol_core::KnownEncoding::TextPlain,
                z_encoding_prefix::AppProperties => {
                    zenoh_protocol_core::KnownEncoding::AppProperties
                }
                z_encoding_prefix::AppJson => zenoh_protocol_core::KnownEncoding::AppJson,
                z_encoding_prefix::AppSql => zenoh_protocol_core::KnownEncoding::AppSql,
                z_encoding_prefix::AppInteger => zenoh_protocol_core::KnownEncoding::AppInteger,
                z_encoding_prefix::AppFloat => zenoh_protocol_core::KnownEncoding::AppFloat,
                z_encoding_prefix::AppXml => zenoh_protocol_core::KnownEncoding::AppXml,
                z_encoding_prefix::AppXhtmlXml => zenoh_protocol_core::KnownEncoding::AppXhtmlXml,
                z_encoding_prefix::AppXWwwFormUrlencoded => {
                    zenoh_protocol_core::KnownEncoding::AppXWwwFormUrlencoded
                }
                z_encoding_prefix::TextJson => zenoh_protocol_core::KnownEncoding::TextJson,
                z_encoding_prefix::TextHtml => zenoh_protocol_core::KnownEncoding::TextHtml,
                z_encoding_prefix::TextXml => zenoh_protocol_core::KnownEncoding::TextXml,
                z_encoding_prefix::TextCss => zenoh_protocol_core::KnownEncoding::TextCss,
                z_encoding_prefix::TextCsv => zenoh_protocol_core::KnownEncoding::TextCsv,
                z_encoding_prefix::TextJavascript => {
                    zenoh_protocol_core::KnownEncoding::TextJavascript
                }
                z_encoding_prefix::ImageJpeg => zenoh_protocol_core::KnownEncoding::ImageJpeg,
                z_encoding_prefix::ImagePng => zenoh_protocol_core::KnownEncoding::ImagePng,
                z_encoding_prefix::ImageGif => zenoh_protocol_core::KnownEncoding::ImageGif,
            }
        } else {
            unsafe { std::mem::transmute(val as u8) }
        }
    }
}

impl From<zenoh_protocol_core::KnownEncoding> for z_encoding_prefix {
    fn from(val: zenoh_protocol_core::KnownEncoding) -> Self {
        if cfg!(debug_assertions) {
            match val {
                zenoh_protocol_core::KnownEncoding::Empty => z_encoding_prefix::Empty,
                zenoh_protocol_core::KnownEncoding::AppOctetStream => {
                    z_encoding_prefix::AppOctetStream
                }
                zenoh_protocol_core::KnownEncoding::AppCustom => z_encoding_prefix::AppCustom,
                zenoh_protocol_core::KnownEncoding::TextPlain => z_encoding_prefix::TextPlain,
                zenoh_protocol_core::KnownEncoding::AppProperties => {
                    z_encoding_prefix::AppProperties
                }
                zenoh_protocol_core::KnownEncoding::AppJson => z_encoding_prefix::AppJson,
                zenoh_protocol_core::KnownEncoding::AppSql => z_encoding_prefix::AppSql,
                zenoh_protocol_core::KnownEncoding::AppInteger => z_encoding_prefix::AppInteger,
                zenoh_protocol_core::KnownEncoding::AppFloat => z_encoding_prefix::AppFloat,
                zenoh_protocol_core::KnownEncoding::AppXml => z_encoding_prefix::AppXml,
                zenoh_protocol_core::KnownEncoding::AppXhtmlXml => z_encoding_prefix::AppXhtmlXml,
                zenoh_protocol_core::KnownEncoding::AppXWwwFormUrlencoded => {
                    z_encoding_prefix::AppXWwwFormUrlencoded
                }
                zenoh_protocol_core::KnownEncoding::TextJson => z_encoding_prefix::TextJson,
                zenoh_protocol_core::KnownEncoding::TextHtml => z_encoding_prefix::TextHtml,
                zenoh_protocol_core::KnownEncoding::TextXml => z_encoding_prefix::TextXml,
                zenoh_protocol_core::KnownEncoding::TextCss => z_encoding_prefix::TextCss,
                zenoh_protocol_core::KnownEncoding::TextCsv => z_encoding_prefix::TextCsv,
                zenoh_protocol_core::KnownEncoding::TextJavascript => {
                    z_encoding_prefix::TextJavascript
                }
                zenoh_protocol_core::KnownEncoding::ImageJpeg => z_encoding_prefix::ImageJpeg,
                zenoh_protocol_core::KnownEncoding::ImagePng => z_encoding_prefix::ImagePng,
                zenoh_protocol_core::KnownEncoding::ImageGif => z_encoding_prefix::ImageGif,
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
/// `suffix` MUST be a valid UTF-8 string.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct z_encoding_t {
    pub prefix: z_encoding_prefix,
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

#[repr(C)]
pub struct z_owned_encoding_t {
    pub prefix: z_encoding_prefix,
    pub suffix: z_bytes_t,
    pub _dropped: bool,
}

/// Frees `encoding`, invalidating it for double-drop safety.
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

/// Returns `true` if `encoding` is valid.
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
