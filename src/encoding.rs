//
// Copyright (c) 2017, 2024 ZettaScale Technology.
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

use std::{
    borrow::Cow,
    mem::MaybeUninit,
    ptr::null,
    slice::from_raw_parts,
    str::{from_utf8, FromStr},
};

use libc::c_char;
use prebindgen_proc_macro::prebindgen;
use unwrap_infallible::UnwrapInfallible;
use zenoh::bytes::Encoding;

pub use zenoh_ffi_opaque_types::opaque_types::{z_loaned_encoding_t, z_owned_encoding_t};
use crate::{
    result::{self, z_result_t},
    strlen_or_zero,
    transmute::{Gravestone, LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_moved_encoding_t, z_owned_string_t, z_string_copy_from_substr,
};

decl_c_type!(
    owned(z_owned_encoding_t, Encoding),
    loaned(z_loaned_encoding_t, Encoding),
);

impl Gravestone for Encoding {
    fn gravestone() -> Self {
        Encoding::default()
    }
    fn is_gravestone(&self) -> bool {
        self == &Self::default()
    }
}

/// Constructs a `z_owned_encoding_t` from a specified substring.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_encoding_from_substr(
    this: &mut MaybeUninit<z_owned_encoding_t>,
    s: *const c_char,
    len: usize,
) -> result::z_result_t {
    let encoding = this.as_rust_type_mut_uninit();
    if s.is_null() {
        encoding.write(Encoding::default());
        result::Z_OK
    } else {
        #[allow(clippy::unnecessary_cast)]
        let s = from_raw_parts(s as *const u8, len);
        match from_utf8(s) {
            Ok(s) => {
                encoding.write(Encoding::from_str(s).unwrap_infallible());
                result::Z_OK
            }
            Err(e) => {
                crate::report_error!("Can not create encoding from non UTF-8 string: {}", e);
                encoding.write(Encoding::default());
                result::Z_EINVAL
            }
        }
    }
}

/// Set a schema to this encoding from a c substring. Zenoh does not define what a schema is and its semantichs is left to the implementer.
/// E.g. a common schema for `text/plain` encoding is `utf-8`.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_encoding_set_schema_from_substr(
    this: &mut z_loaned_encoding_t,
    s: *const c_char,
    len: usize,
) -> result::z_result_t {
    let encoding = this.as_rust_type_mut();
    if len == 0 {
        *encoding = std::mem::take(encoding).with_schema(String::new());
        return result::Z_OK;
    } else if s.is_null() {
        crate::report_error!("Non-zero length string should not be null");
        return result::Z_EINVAL;
    }
    #[allow(clippy::unnecessary_cast)]
    let schema_bytes = from_raw_parts(s as *const u8, len);
    match from_utf8(schema_bytes) {
        Ok(schema_str) => {
            *encoding = std::mem::take(encoding).with_schema(schema_str);
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("{}", e);
            result::Z_EINVAL
        }
    }
}

/// Set a schema to this encoding from a c string. Zenoh does not define what a schema is and its semantichs is left to the implementer.
/// E.g. a common schema for `text/plain` encoding is `utf-8`.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_encoding_set_schema_from_str(
    this: &mut z_loaned_encoding_t,
    s: *const c_char,
) -> z_result_t {
    z_encoding_set_schema_from_substr(this, s, strlen_or_zero(s))
}

/// Constructs a `z_owned_encoding_t` from a specified string.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_encoding_from_str(
    this: &mut MaybeUninit<z_owned_encoding_t>,
    s: *const c_char,
) -> result::z_result_t {
    z_encoding_from_substr(this, s, strlen_or_zero(s))
}

/// Constructs an owned non-null-terminated string from encoding
///
/// @param this_: Encoding.
/// @param out_str: Uninitialized memory location where a string to be constructed.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_encoding_to_string(
    this: &z_loaned_encoding_t,
    out_str: &mut MaybeUninit<z_owned_string_t>,
) {
    let s: Cow<'static, str> = this.as_rust_type_ref().into();
    z_string_copy_from_substr(out_str, s.as_bytes().as_ptr() as _, s.len());
}

/// Returns a loaned default `z_loaned_encoding_t`.
#[allow(clippy::missing_safety_doc)]
#[prebindgen]
pub fn z_encoding_loan_default() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_BYTES.as_loaned_c_type_ref()
}

/// Constructs a default `z_owned_encoding_t`.
#[prebindgen]
pub fn z_internal_encoding_null(this_: &mut MaybeUninit<z_owned_encoding_t>) {
    this_.as_rust_type_mut_uninit().write(Encoding::default());
}

/// Frees the memory and resets the encoding it to its default value.
#[prebindgen]
pub fn z_encoding_drop(this_: &mut z_moved_encoding_t) {
    let _ = this_.take_rust_type();
}

/// Returns ``true`` if encoding is in non-default state, ``false`` otherwise.
#[prebindgen]
pub fn z_internal_encoding_check(this_: &'static z_owned_encoding_t) -> bool {
    *this_.as_rust_type_ref() != Encoding::default()
}

/// Borrows encoding.
#[prebindgen]
pub fn z_encoding_loan(this_: &z_owned_encoding_t) -> &z_loaned_encoding_t {
    this_.as_rust_type_ref().as_loaned_c_type_ref()
}

/// Moves encoding.
#[prebindgen("move")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_encoding_move(this_: &mut z_owned_encoding_t) -> &mut z_moved_encoding_t {
    std::mem::transmute(this_)
}

/// Mutably borrows encoding.
#[prebindgen]
pub fn z_encoding_loan_mut(this_: &mut z_owned_encoding_t) -> &mut z_loaned_encoding_t {
    this_.as_rust_type_mut().as_loaned_c_type_mut()
}

/// Constructs an owned copy of the encoding in provided uninitilized memory location.
#[prebindgen]
pub fn z_encoding_clone(dst: &mut MaybeUninit<z_owned_encoding_t>, this: &z_loaned_encoding_t) {
    dst.as_rust_type_mut_uninit()
        .write(this.as_rust_type_ref().clone());
}

/// Returns ``true`` if `this_` equals to `other`, ``false`` otherwise.
#[prebindgen]
pub fn z_encoding_equals(this_: &z_loaned_encoding_t, other: &z_loaned_encoding_t) -> bool {
    this_.as_rust_type_ref() == other.as_rust_type_ref()
}

/// Just some bytes.
///
/// Constant alias for string: `"zenoh/bytes"`.
///
/// This encoding supposes that the payload was created with `z_bytes_from_buf()`, `z_bytes_from_slice()` or
/// similar functions and its data can be accessed via `z_bytes_to_slice()`.
#[prebindgen]
pub fn z_encoding_zenoh_bytes() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_BYTES.as_loaned_c_type_ref()
}
/// A UTF-8 string.
///
/// Constant alias for string: `"zenoh/string"`.
///
/// This encoding supposes that the payload was created with `z_bytes_from_str()`, `z_bytes_from_string()` or
/// similar functions and its data can be accessed via `z_bytes_to_string()`.
#[prebindgen]
pub fn z_encoding_zenoh_string() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_STRING.as_loaned_c_type_ref()
}

/// Zenoh serialized data.
///
/// Constant alias for string: `"zenoh/serialized"`.
///
/// This encoding supposes that the payload was created with serialization functions.
/// The `schema` field may contain the details of serialziation format.
#[prebindgen]
pub fn z_encoding_zenoh_serialized() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_SERIALIZED.as_loaned_c_type_ref()
}

// - Advanced types may be supported in some of the Zenoh bindings.
/// An application-specific stream of bytes.
///
/// Constant alias for string: `"application/octet-stream"`.
#[prebindgen]
pub fn z_encoding_application_octet_stream() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_OCTET_STREAM.as_loaned_c_type_ref()
}
/// A textual file.
///
/// Constant alias for string: `"text/plain"`.
#[prebindgen]
pub fn z_encoding_text_plain() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_PLAIN.as_loaned_c_type_ref()
}
/// JSON data intended to be consumed by an application.
///
/// Constant alias for string: `"application/json"`.
#[prebindgen]
pub fn z_encoding_application_json() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_JSON.as_loaned_c_type_ref()
}
/// JSON data intended to be human readable.
///
/// Constant alias for string: `"text/json"`.
#[prebindgen]
pub fn z_encoding_text_json() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_JSON.as_loaned_c_type_ref()
}
/// A Common Data Representation (CDR)-encoded data.
///
/// Constant alias for string: `"application/cdr"`.
#[prebindgen]
pub fn z_encoding_application_cdr() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_CDR.as_loaned_c_type_ref()
}
/// A Concise Binary Object Representation (CBOR)-encoded data.
///
/// Constant alias for string: `"application/cbor"`.
#[prebindgen]
pub fn z_encoding_application_cbor() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_CBOR.as_loaned_c_type_ref()
}
/// YAML data intended to be consumed by an application.
///
/// Constant alias for string: `"application/yaml"`.
#[prebindgen]
pub fn z_encoding_application_yaml() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_YAML.as_loaned_c_type_ref()
}
/// YAML data intended to be human readable.
///
/// Constant alias for string: `"text/yaml"`.
#[prebindgen]
pub fn z_encoding_text_yaml() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_YAML.as_loaned_c_type_ref()
}
/// JSON5 encoded data that are human readable.
///
/// Constant alias for string: `"text/json5"`.
#[prebindgen]
pub fn z_encoding_text_json5() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_JSON5.as_loaned_c_type_ref()
}
/// A Python object serialized using [pickle](https://docs.python.org/3/library/pickle.html).
///
/// Constant alias for string: `"application/python-serialized-object"`.
#[prebindgen]
pub fn z_encoding_application_python_serialized_object() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_PYTHON_SERIALIZED_OBJECT.as_loaned_c_type_ref()
}
/// An application-specific protobuf-encoded data.
///
/// Constant alias for string: `"application/protobuf"`.
#[prebindgen]
pub fn z_encoding_application_protobuf() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_PROTOBUF.as_loaned_c_type_ref()
}
/// A Java serialized object.
///
/// Constant alias for string: `"application/java-serialized-object"`.
#[prebindgen]
pub fn z_encoding_application_java_serialized_object() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_JAVA_SERIALIZED_OBJECT.as_loaned_c_type_ref()
}
/// An [openmetrics](https://github.com/OpenObservability/OpenMetrics) data, common used by [Prometheus](https://prometheus.io/).
///
/// Constant alias for string: `"application/openmetrics-text"`.
#[prebindgen]
pub fn z_encoding_application_openmetrics_text() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_OPENMETRICS_TEXT.as_loaned_c_type_ref()
}
/// A Portable Network Graphics (PNG) image.
///
/// Constant alias for string: `"image/png"`.
#[prebindgen]
pub fn z_encoding_image_png() -> &'static z_loaned_encoding_t {
    Encoding::IMAGE_PNG.as_loaned_c_type_ref()
}
/// A Joint Photographic Experts Group (JPEG) image.
///
/// Constant alias for string: `"image/jpeg"`.
#[prebindgen]
pub fn z_encoding_image_jpeg() -> &'static z_loaned_encoding_t {
    Encoding::IMAGE_JPEG.as_loaned_c_type_ref()
}
/// A Graphics Interchange Format (GIF) image.
///
/// Constant alias for string: `"image/gif"`.
#[prebindgen]
pub fn z_encoding_image_gif() -> &'static z_loaned_encoding_t {
    Encoding::IMAGE_GIF.as_loaned_c_type_ref()
}
/// A BitMap (BMP) image.
///
/// Constant alias for string: `"image/bmp"`.
#[prebindgen]
pub fn z_encoding_image_bmp() -> &'static z_loaned_encoding_t {
    Encoding::IMAGE_BMP.as_loaned_c_type_ref()
}
/// A Web Portable (WebP) image.
///
///  Constant alias for string: `"image/webp"`.
#[prebindgen]
pub fn z_encoding_image_webp() -> &'static z_loaned_encoding_t {
    Encoding::IMAGE_WEBP.as_loaned_c_type_ref()
}
/// An XML file intended to be consumed by an application..
///
/// Constant alias for string: `"application/xml"`.
#[prebindgen]
pub fn z_encoding_application_xml() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_XML.as_loaned_c_type_ref()
}
/// An encoded a list of tuples, each consisting of a name and a value.
///
/// Constant alias for string: `"application/x-www-form-urlencoded"`.
#[prebindgen]
pub fn z_encoding_application_x_www_form_urlencoded() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_X_WWW_FORM_URLENCODED.as_loaned_c_type_ref()
}
/// An HTML file.
///
/// Constant alias for string: `"text/html"`.
#[prebindgen]
pub fn z_encoding_text_html() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_HTML.as_loaned_c_type_ref()
}
/// An XML file that is human readable.
///
/// Constant alias for string: `"text/xml"`.
#[prebindgen]
pub fn z_encoding_text_xml() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_XML.as_loaned_c_type_ref()
}
/// A CSS file.
///
/// Constant alias for string: `"text/css"`.
#[prebindgen]
pub fn z_encoding_text_css() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_CSS.as_loaned_c_type_ref()
}
/// A JavaScript file.
///
/// Constant alias for string: `"text/javascript"`.
#[prebindgen]
pub fn z_encoding_text_javascript() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_JAVASCRIPT.as_loaned_c_type_ref()
}
/// A MarkDown file.
///
/// Constant alias for string: `"text/markdown"`.
#[prebindgen]
pub fn z_encoding_text_markdown() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_MARKDOWN.as_loaned_c_type_ref()
}
/// A CSV file.
///
/// Constant alias for string: `"text/csv"`.
#[prebindgen]
pub fn z_encoding_text_csv() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_CSV.as_loaned_c_type_ref()
}
/// An application-specific SQL query.
///
/// Constant alias for string: `"application/sql"`.
#[prebindgen]
pub fn z_encoding_application_sql() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_SQL.as_loaned_c_type_ref()
}
/// Constrained Application Protocol (CoAP) data intended for CoAP-to-HTTP and HTTP-to-CoAP proxies.
///
/// Constant alias for string: `"application/coap-payload"`.
#[prebindgen]
pub fn z_encoding_application_coap_payload() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_COAP_PAYLOAD.as_loaned_c_type_ref()
}
/// Defines a JSON document structure for expressing a sequence of operations to apply to a JSON document.
///
/// Constant alias for string: `"application/json-patch+json"`.
#[prebindgen]
pub fn z_encoding_application_json_patch_json() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_JSON_PATCH_JSON.as_loaned_c_type_ref()
}
/// A JSON text sequence consists of any number of JSON texts, all encoded in UTF-8.
///
/// Constant alias for string: `"application/json-seq"`.
#[prebindgen]
pub fn z_encoding_application_json_seq() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_JSON_SEQ.as_loaned_c_type_ref()
}
/// A JSONPath defines a string syntax for selecting and extracting JSON values from within a given JSON value.
///
/// Constant alias for string: `"application/jsonpath"`.
#[prebindgen]
pub fn z_encoding_application_jsonpath() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_JSONPATH.as_loaned_c_type_ref()
}
/// A JSON Web Token (JWT).
///
/// Constant alias for string: `"application/jwt"`.
#[prebindgen]
pub fn z_encoding_application_jwt() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_JWT.as_loaned_c_type_ref()
}
/// An application-specific MPEG-4 encoded data, either audio or video.
///
/// Constant alias for string: `"application/mp4"`.
#[prebindgen]
pub fn z_encoding_application_mp4() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_MP4.as_loaned_c_type_ref()
}
/// A SOAP 1.2 message serialized as XML 1.0.
///
/// Constant alias for string: `"application/soap+xml"`.
#[prebindgen]
pub fn z_encoding_application_soap_xml() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_SOAP_XML.as_loaned_c_type_ref()
}
/// A YANG-encoded data commonly used by the Network Configuration Protocol (NETCONF).
///
/// Constant alias for string: `"application/yang"`.
#[prebindgen]
pub fn z_encoding_application_yang() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_YANG.as_loaned_c_type_ref()
}
/// A MPEG-4 Advanced Audio Coding (AAC) media.
///
/// Constant alias for string: `"audio/aac"`.
#[prebindgen]
pub fn z_encoding_audio_aac() -> &'static z_loaned_encoding_t {
    Encoding::AUDIO_AAC.as_loaned_c_type_ref()
}
/// A Free Lossless Audio Codec (FLAC) media.
///
/// Constant alias for string: `"audio/flac"`.
#[prebindgen]
pub fn z_encoding_audio_flac() -> &'static z_loaned_encoding_t {
    Encoding::AUDIO_FLAC.as_loaned_c_type_ref()
}
/// An audio codec defined in MPEG-1, MPEG-2, MPEG-4, or registered at the MP4 registration authority.
///
/// Constant alias for string: `"audio/mp4"`.
#[prebindgen]
pub fn z_encoding_audio_mp4() -> &'static z_loaned_encoding_t {
    Encoding::AUDIO_MP4.as_loaned_c_type_ref()
}
/// An Ogg-encapsulated audio stream.
///
/// Constant alias for string: `"audio/ogg"`.
#[prebindgen]
pub fn z_encoding_audio_ogg() -> &'static z_loaned_encoding_t {
    Encoding::AUDIO_OGG.as_loaned_c_type_ref()
}
/// A Vorbis-encoded audio stream.
///
/// Constant alias for string: `"audio/vorbis"`.
#[prebindgen]
pub fn z_encoding_audio_vorbis() -> &'static z_loaned_encoding_t {
    Encoding::AUDIO_VORBIS.as_loaned_c_type_ref()
}
/// A h261-encoded video stream.
///
/// Constant alias for string: `"video/h261"`.
#[prebindgen]
pub fn z_encoding_video_h261() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_H261.as_loaned_c_type_ref()
}
/// A h263-encoded video stream.
///
/// Constant alias for string: `"video/h263"`.
#[prebindgen]
pub fn z_encoding_video_h263() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_H263.as_loaned_c_type_ref()
}
/// A h264-encoded video stream.
///
/// Constant alias for string: `"video/h264"`.
#[prebindgen]
pub fn z_encoding_video_h264() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_H264.as_loaned_c_type_ref()
}
/// A h265-encoded video stream.
///
/// Constant alias for string: `"video/h265"`.
#[prebindgen]
pub fn z_encoding_video_h265() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_H265.as_loaned_c_type_ref()
}
/// A h266-encoded video stream.
///
/// Constant alias for string: `"video/h266"`.
#[prebindgen]
pub fn z_encoding_video_h266() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_H266.as_loaned_c_type_ref()
}
/// A video codec defined in MPEG-1, MPEG-2, MPEG-4, or registered at the MP4 registration authority.
///
/// Constant alias for string: `"video/mp4"`.
#[prebindgen]
pub fn z_encoding_video_mp4() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_MP4.as_loaned_c_type_ref()
}
/// An Ogg-encapsulated video stream.
///
/// Constant alias for string: `"video/ogg"`.
#[prebindgen]
pub fn z_encoding_video_ogg() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_OGG.as_loaned_c_type_ref()
}
/// An uncompressed, studio-quality video stream.
///
/// Constant alias for string: `"video/raw"`.
#[prebindgen]
pub fn z_encoding_video_raw() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_RAW.as_loaned_c_type_ref()
}
/// A VP8-encoded video stream.
///
/// Constant alias for string: `"video/vp8"`.
#[prebindgen]
pub fn z_encoding_video_vp8() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_VP8.as_loaned_c_type_ref()
}
/// A VP9-encoded video stream.
///
/// Constant alias for string: `"video/vp9"`.
#[prebindgen]
pub fn z_encoding_video_vp9() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_VP9.as_loaned_c_type_ref()
}

#[prebindgen]
#[repr(C)]
pub struct zc_internal_encoding_data_t {
    id: u16,
    schema_ptr: *const u8,
    schema_len: usize,
}

#[prebindgen]
pub fn zc_internal_encoding_get_data(
    this: &'static z_loaned_encoding_t,
) -> zc_internal_encoding_data_t {
    let encoding = this.as_rust_type_ref();
    let schema = encoding.schema();
    match schema {
        Some(s) => zc_internal_encoding_data_t {
            id: encoding.id(),
            schema_ptr: s.as_ptr(),
            schema_len: s.len(),
        },
        None => zc_internal_encoding_data_t {
            id: encoding.id(),
            schema_ptr: null(),
            schema_len: 0,
        },
    }
}

#[allow(clippy::missing_safety_doc)]
#[prebindgen]
pub unsafe fn zc_internal_encoding_from_data(
    this: &mut MaybeUninit<z_owned_encoding_t>,
    data: zc_internal_encoding_data_t,
) {
    let schema = (!data.schema_ptr.is_null() && data.schema_len > 0).then_some(
        from_raw_parts(data.schema_ptr, data.schema_len)
            .to_vec()
            .into(),
    );
    this.as_rust_type_mut_uninit()
        .write(Encoding::new(data.id, schema));
}
