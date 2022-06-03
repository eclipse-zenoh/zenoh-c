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
use libc::{c_ulong, c_void};
use zenoh::prelude::SampleKind;

/// A zenoh unsigned integer
#[allow(non_camel_case_types)]
pub type z_zint_t = c_ulong;

// CallbackArgs captures optional arguments provided by the user that
// need to be passed to the user-provided callback at every call.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub(crate) struct CallbackArgs(*mut c_void);

unsafe impl Send for CallbackArgs {}
unsafe impl Sync for CallbackArgs {}

impl From<*mut c_void> for CallbackArgs {
    fn from(ptr: *mut c_void) -> Self {
        Self(ptr)
    }
}

impl From<CallbackArgs> for *mut c_void {
    fn from(cargs: CallbackArgs) -> Self {
        cargs.0
    }
}

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
            SampleKind::Patch => z_sample_kind::PUT, // @TODO: to be removed once removed in Rust
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
    // @TODO: add timestamp and source_info
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum z_known_encoding {
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

impl From<z_known_encoding> for zenoh_protocol_core::KnownEncoding {
    fn from(val: z_known_encoding) -> Self {
        if cfg!(debug_assertions) {
            match val {
                z_known_encoding::Empty => zenoh_protocol_core::KnownEncoding::Empty,
                z_known_encoding::AppOctetStream => {
                    zenoh_protocol_core::KnownEncoding::AppOctetStream
                }
                z_known_encoding::AppCustom => zenoh_protocol_core::KnownEncoding::AppCustom,
                z_known_encoding::TextPlain => zenoh_protocol_core::KnownEncoding::TextPlain,
                z_known_encoding::AppProperties => {
                    zenoh_protocol_core::KnownEncoding::AppProperties
                }
                z_known_encoding::AppJson => zenoh_protocol_core::KnownEncoding::AppJson,
                z_known_encoding::AppSql => zenoh_protocol_core::KnownEncoding::AppSql,
                z_known_encoding::AppInteger => zenoh_protocol_core::KnownEncoding::AppInteger,
                z_known_encoding::AppFloat => zenoh_protocol_core::KnownEncoding::AppFloat,
                z_known_encoding::AppXml => zenoh_protocol_core::KnownEncoding::AppXml,
                z_known_encoding::AppXhtmlXml => zenoh_protocol_core::KnownEncoding::AppXhtmlXml,
                z_known_encoding::AppXWwwFormUrlencoded => {
                    zenoh_protocol_core::KnownEncoding::AppXWwwFormUrlencoded
                }
                z_known_encoding::TextJson => zenoh_protocol_core::KnownEncoding::TextJson,
                z_known_encoding::TextHtml => zenoh_protocol_core::KnownEncoding::TextHtml,
                z_known_encoding::TextXml => zenoh_protocol_core::KnownEncoding::TextXml,
                z_known_encoding::TextCss => zenoh_protocol_core::KnownEncoding::TextCss,
                z_known_encoding::TextCsv => zenoh_protocol_core::KnownEncoding::TextCsv,
                z_known_encoding::TextJavascript => {
                    zenoh_protocol_core::KnownEncoding::TextJavascript
                }
                z_known_encoding::ImageJpeg => zenoh_protocol_core::KnownEncoding::ImageJpeg,
                z_known_encoding::ImagePng => zenoh_protocol_core::KnownEncoding::ImagePng,
                z_known_encoding::ImageGif => zenoh_protocol_core::KnownEncoding::ImageGif,
            }
        } else {
            unsafe { std::mem::transmute(val as u8) }
        }
    }
}

impl From<zenoh_protocol_core::KnownEncoding> for z_known_encoding {
    fn from(val: zenoh_protocol_core::KnownEncoding) -> Self {
        if cfg!(debug_assertions) {
            match val {
                zenoh_protocol_core::KnownEncoding::Empty => z_known_encoding::Empty,
                zenoh_protocol_core::KnownEncoding::AppOctetStream => {
                    z_known_encoding::AppOctetStream
                }
                zenoh_protocol_core::KnownEncoding::AppCustom => z_known_encoding::AppCustom,
                zenoh_protocol_core::KnownEncoding::TextPlain => z_known_encoding::TextPlain,
                zenoh_protocol_core::KnownEncoding::AppProperties => {
                    z_known_encoding::AppProperties
                }
                zenoh_protocol_core::KnownEncoding::AppJson => z_known_encoding::AppJson,
                zenoh_protocol_core::KnownEncoding::AppSql => z_known_encoding::AppSql,
                zenoh_protocol_core::KnownEncoding::AppInteger => z_known_encoding::AppInteger,
                zenoh_protocol_core::KnownEncoding::AppFloat => z_known_encoding::AppFloat,
                zenoh_protocol_core::KnownEncoding::AppXml => z_known_encoding::AppXml,
                zenoh_protocol_core::KnownEncoding::AppXhtmlXml => z_known_encoding::AppXhtmlXml,
                zenoh_protocol_core::KnownEncoding::AppXWwwFormUrlencoded => {
                    z_known_encoding::AppXWwwFormUrlencoded
                }
                zenoh_protocol_core::KnownEncoding::TextJson => z_known_encoding::TextJson,
                zenoh_protocol_core::KnownEncoding::TextHtml => z_known_encoding::TextHtml,
                zenoh_protocol_core::KnownEncoding::TextXml => z_known_encoding::TextXml,
                zenoh_protocol_core::KnownEncoding::TextCss => z_known_encoding::TextCss,
                zenoh_protocol_core::KnownEncoding::TextCsv => z_known_encoding::TextCsv,
                zenoh_protocol_core::KnownEncoding::TextJavascript => {
                    z_known_encoding::TextJavascript
                }
                zenoh_protocol_core::KnownEncoding::ImageJpeg => z_known_encoding::ImageJpeg,
                zenoh_protocol_core::KnownEncoding::ImagePng => z_known_encoding::ImagePng,
                zenoh_protocol_core::KnownEncoding::ImageGif => z_known_encoding::ImageGif,
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
    pub prefix: z_known_encoding,
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
    pub prefix: z_known_encoding,
    pub suffix: z_bytes_t,
    pub _freed: bool,
}

/// Frees `encoding`, invalidating it for double-free safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_default() -> z_encoding_t {
    (&zenoh_protocol_core::Encoding::default()).into()
}

/// Frees `encoding`, invalidating it for double-free safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_free(encoding: &mut z_owned_encoding_t) {
    z_bytes_free(&mut encoding.suffix);
    encoding._freed = true
}

/// Returns `true` if `encoding` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_check(encoding: &z_owned_encoding_t) -> bool {
    !encoding._freed
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
            _freed: false,
        }
    }
}
