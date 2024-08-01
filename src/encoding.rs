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
    slice::from_raw_parts,
    str::{from_utf8, FromStr},
};

use libc::{c_char, strlen};
use unwrap_infallible::UnwrapInfallible;
use zenoh::bytes::Encoding;

pub use crate::opaque_types::{z_loaned_encoding_t, z_owned_encoding_t};
use crate::{
    result::{self, z_result_t},
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit},
    z_owned_string_t, z_string_copy_from_substr,
};

decl_c_type!(
    owned(z_owned_encoding_t, Encoding),
    loaned(z_loaned_encoding_t, Encoding),
);

/// Constructs a `z_owned_encoding_t` from a specified substring.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_from_substr(
    this: &mut MaybeUninit<z_owned_encoding_t>,
    s: *const c_char,
    len: usize,
) -> result::z_result_t {
    let encoding = this.as_rust_type_mut_uninit();
    if s.is_null() {
        encoding.write(Encoding::default());
        result::Z_OK
    } else {
        let s = from_raw_parts(s as *const u8, len);
        match from_utf8(s) {
            Ok(s) => {
                encoding.write(Encoding::from_str(s).unwrap_infallible());
                result::Z_OK
            }
            Err(e) => {
                tracing::error!("Can not create encoding from non UTF-8 string: {}", e);
                encoding.write(Encoding::default());
                result::Z_EINVAL
            }
        }
    }
}

/// Set a schema to this encoding from a c substring. Zenoh does not define what a schema is and its semantichs is left to the implementer.
/// E.g. a common schema for `text/plain` encoding is `utf-8`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_set_schema_from_substr(
    this: &mut z_loaned_encoding_t,
    s: *const c_char,
    len: usize,
) -> result::z_result_t {
    let encoding = this.as_rust_type_mut();
    if len == 0 {
        *encoding = std::mem::take(encoding).with_schema(String::new());
        return result::Z_OK;
    } else if s.is_null() {
        return result::Z_EINVAL;
    }
    let schema_bytes = from_raw_parts(s as *const u8, len);
    match from_utf8(schema_bytes) {
        Ok(schema_str) => {
            *encoding = std::mem::take(encoding).with_schema(schema_str);
            result::Z_OK
        }
        Err(e) => {
            tracing::error!("{}", e);
            result::Z_EINVAL
        }
    }
}

/// Set a schema to this encoding from a c string. Zenoh does not define what a schema is and its semantichs is left to the implementer.
/// E.g. a common schema for `text/plain` encoding is `utf-8`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_set_schema_from_str(
    this: &mut z_loaned_encoding_t,
    s: *const c_char,
) -> z_result_t {
    z_encoding_set_schema_from_substr(this, s, strlen(s))
}

/// Constructs a `z_owned_encoding_t` from a specified string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_from_str(
    this: &mut MaybeUninit<z_owned_encoding_t>,
    s: *const c_char,
) -> result::z_result_t {
    z_encoding_from_substr(this, s, libc::strlen(s))
}

/// Constructs an owned non-null-terminated string from encoding
///
/// @param this_: Encoding.
/// @param out_str: Uninitialized memory location where a string to be constructed.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_to_string(
    this: &z_loaned_encoding_t,
    out_str: &mut MaybeUninit<z_owned_string_t>,
) {
    let s: Cow<'static, str> = this.as_rust_type_ref().into();
    z_string_copy_from_substr(out_str, s.as_bytes().as_ptr() as _, s.as_bytes().len());
}

/// Returns a loaned default `z_loaned_encoding_t`.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_encoding_loan_default() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_BYTES.as_loaned_c_type_ref()
}

/// Constructs a default `z_owned_encoding_t`.
#[no_mangle]
pub extern "C" fn z_encoding_null(this: &mut MaybeUninit<z_owned_encoding_t>) {
    this.as_rust_type_mut_uninit().write(Encoding::default());
}

/// Frees the memory and resets the encoding it to its default value.
#[no_mangle]
pub extern "C" fn z_encoding_drop(this: &mut z_owned_encoding_t) {
    *this.as_rust_type_mut() = Encoding::default();
}

/// Returns ``true`` if encoding is in non-default state, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_encoding_check(this: &'static z_owned_encoding_t) -> bool {
    *this.as_rust_type_ref() != Encoding::default()
}

/// Borrows encoding.
#[no_mangle]
pub extern "C" fn z_encoding_loan(this: &z_owned_encoding_t) -> &z_loaned_encoding_t {
    this.as_rust_type_ref().as_loaned_c_type_ref()
}

/// Mutably borrows encoding.
#[no_mangle]
pub extern "C" fn z_encoding_loan_mut(this: &mut z_owned_encoding_t) -> &mut z_loaned_encoding_t {
    this.as_rust_type_mut().as_loaned_c_type_mut()
}

/// Constructs an owned copy of the encoding in provided uninitilized memory location.
#[no_mangle]
pub extern "C" fn z_encoding_clone(
    dst: &mut MaybeUninit<z_owned_encoding_t>,
    this: &z_loaned_encoding_t,
) {
    dst.as_rust_type_mut_uninit()
        .write(this.as_rust_type_ref().clone());
}

/// Just some bytes.
///
/// Constant alias for string: `"zenoh/bytes"`.
///
/// Usually used for types: `uint8_t[]`.
#[no_mangle]
pub extern "C" fn z_encoding_zenoh_bytes() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_BYTES.as_loaned_c_type_ref()
}
/// A VLE-encoded signed little-endian 8bit integer. Binary representation uses two's complement.
///
/// Constant alias for string: `"zenoh/int8"`.
///
/// Usually used for types: `int8_t`.
#[no_mangle]
pub extern "C" fn z_encoding_zenoh_int8() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_INT8.as_loaned_c_type_ref()
}
/// A VLE-encoded signed little-endian 16bit integer. Binary representation uses two's complement.
///
/// Constant alias for string: `"zenoh/int16"`.
///
/// Usually used for types: `int16_t`.
#[no_mangle]
pub extern "C" fn z_encoding_zenoh_int16() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_INT16.as_loaned_c_type_ref()
}
/// A VLE-encoded signed little-endian 32bit integer. Binary representation uses two's complement.
///
/// Constant alias for string: `"zenoh/int32"`.
///
/// Usually used for types: `int32_t`.
#[no_mangle]
pub extern "C" fn z_encoding_zenoh_int32() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_INT32.as_loaned_c_type_ref()
}
/// A VLE-encoded signed little-endian 64bit integer. Binary representation uses two's complement.
///
/// Constant alias for string: `"zenoh/int64"`.
///
/// Usually used for types: `int64_t`.
#[no_mangle]
pub extern "C" fn z_encoding_zenoh_int64() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_INT64.as_loaned_c_type_ref()
}
/// A VLE-encoded signed little-endian 128bit integer. Binary representation uses two's complement.
///
/// Constant alias for string: `"zenoh/int128"`.
///
/// Usually used for types: `int128_t`.
#[no_mangle]
pub extern "C" fn z_encoding_zenoh_int128() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_INT128.as_loaned_c_type_ref()
}
/// A VLE-encoded unsigned little-endian 8bit integer.
///
/// Constant alias for string: `"zenoh/uint8"`.
///
/// Usually used for types: `uint8_t`.
#[no_mangle]
pub extern "C" fn z_encoding_zenoh_uint8() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_UINT8.as_loaned_c_type_ref()
}
/// A VLE-encoded unsigned little-endian 16bit integer.
///
/// Constant alias for string: `"zenoh/uint16"`.
///
/// Usually used for types: `uint16_t`.
#[no_mangle]
pub extern "C" fn z_encoding_zenoh_uint16() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_UINT16.as_loaned_c_type_ref()
}
/// A VLE-encoded unsigned little-endian 32bit integer.
///
/// Constant alias for string: `"zenoh/uint32"`.
///
/// Usually used for types: `uint32_t`.
#[no_mangle]
pub extern "C" fn z_encoding_zenoh_uint32() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_UINT32.as_loaned_c_type_ref()
}
/// A VLE-encoded unsigned little-endian 64bit integer.
///
/// Constant alias for string: `"zenoh/uint64"`.
///
/// Usually used for types: `uint64_t`.
#[no_mangle]
pub extern "C" fn z_encoding_zenoh_uint64() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_UINT64.as_loaned_c_type_ref()
}
/// A VLE-encoded unsigned little-endian 128bit integer.
///
/// Constant alias for string: `"zenoh/uint128"`.
///
/// Usually used for types: `uint128_t`.
#[no_mangle]
pub extern "C" fn z_encoding_zenoh_uint128() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_UINT128.as_loaned_c_type_ref()
}
/// A VLE-encoded 32bit float. Binary representation uses *IEEE 754-2008* *binary32* .
///
/// Constant alias for string: `"zenoh/float32"`.
///
/// Usually used for types: `float`.
#[no_mangle]
pub extern "C" fn z_encoding_zenoh_float32() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_FLOAT32.as_loaned_c_type_ref()
}
/// A VLE-encoded 64bit float. Binary representation uses *IEEE 754-2008* *binary64*.
///
/// Constant alias for string: `"zenoh/float64"`.
///
/// Usually used for types: `double`.
#[no_mangle]
pub extern "C" fn z_encoding_zenoh_float64() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_FLOAT64.as_loaned_c_type_ref()
}
/// A boolean. `0` is `false`, `1` is `true`. Other values are invalid.
///
/// Constant alias for string: `"zenoh/bool"`.
///
/// Usually used for types: `bool`.
#[no_mangle]
pub extern "C" fn z_encoding_zenoh_bool() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_BOOL.as_loaned_c_type_ref()
}
/// A UTF-8 string.
///
/// Constant alias for string: `"zenoh/string"`.
///
/// Usually used for types: `const char*`.
#[no_mangle]
pub extern "C" fn z_encoding_zenoh_string() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_STRING.as_loaned_c_type_ref()
}
/// A zenoh error.
///
/// Constant alias for string: `"zenoh/error"`.
///
/// Usually used for types: `z_reply_error_t`.
#[no_mangle]
pub extern "C" fn z_encoding_zenoh_error() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_ERROR.as_loaned_c_type_ref()
}

// - Advanced types may be supported in some of the Zenoh bindings.
/// An application-specific stream of bytes.
///
/// Constant alias for string: `"application/octet-stream"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_octet_stream() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_OCTET_STREAM.as_loaned_c_type_ref()
}
/// A textual file.
///
/// Constant alias for string: `"text/plain"`.
#[no_mangle]
pub extern "C" fn z_encoding_text_plain() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_PLAIN.as_loaned_c_type_ref()
}
/// JSON data intended to be consumed by an application.
///
/// Constant alias for string: `"application/json"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_json() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_JSON.as_loaned_c_type_ref()
}
/// JSON data intended to be human readable.
///
/// Constant alias for string: `"text/json"`.
#[no_mangle]
pub extern "C" fn z_encoding_text_json() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_JSON.as_loaned_c_type_ref()
}
/// A Common Data Representation (CDR)-encoded data.
///
/// Constant alias for string: `"application/cdr"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_cdr() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_CDR.as_loaned_c_type_ref()
}
/// A Concise Binary Object Representation (CBOR)-encoded data.
///
/// Constant alias for string: `"application/cbor"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_cbor() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_CBOR.as_loaned_c_type_ref()
}
/// YAML data intended to be consumed by an application.
///
/// Constant alias for string: `"application/yaml"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_yaml() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_YAML.as_loaned_c_type_ref()
}
/// YAML data intended to be human readable.
///
/// Constant alias for string: `"text/yaml"`.
#[no_mangle]
pub extern "C" fn z_encoding_text_yaml() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_YAML.as_loaned_c_type_ref()
}
/// JSON5 encoded data that are human readable.
///
/// Constant alias for string: `"text/json5"`.
#[no_mangle]
pub extern "C" fn z_encoding_text_json5() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_JSON5.as_loaned_c_type_ref()
}
/// A Python object serialized using [pickle](https://docs.python.org/3/library/pickle.html).
///
/// Constant alias for string: `"application/python-serialized-object"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_python_serialized_objects() -> &'static z_loaned_encoding_t
{
    Encoding::APPLICATION_PYTHON_SERIALIZED_OBJECT.as_loaned_c_type_ref()
}
/// An application-specific protobuf-encoded data.
///
/// Constant alias for string: `"application/protobuf"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_protobuf() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_PROTOBUF.as_loaned_c_type_ref()
}
/// A Java serialized object.
///
/// Constant alias for string: `"application/java-serialized-object"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_java_serialized_object() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_JAVA_SERIALIZED_OBJECT.as_loaned_c_type_ref()
}
/// An [openmetrics](https://github.com/OpenObservability/OpenMetrics) data, common used by [Prometheus](https://prometheus.io/).
///
/// Constant alias for string: `"application/openmetrics-text"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_openmetrics_text() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_OPENMETRICS_TEXT.as_loaned_c_type_ref()
}
/// A Portable Network Graphics (PNG) image.
///
/// Constant alias for string: `"image/png"`.
#[no_mangle]
pub extern "C" fn z_encoding_image_png() -> &'static z_loaned_encoding_t {
    Encoding::IMAGE_PNG.as_loaned_c_type_ref()
}
/// A Joint Photographic Experts Group (JPEG) image.
///
/// Constant alias for string: `"image/jpeg"`.
#[no_mangle]
pub extern "C" fn z_encoding_image_jpeg() -> &'static z_loaned_encoding_t {
    Encoding::IMAGE_JPEG.as_loaned_c_type_ref()
}
/// A Graphics Interchange Format (GIF) image.
///
/// Constant alias for string: `"image/gif"`.
#[no_mangle]
pub extern "C" fn z_encoding_image_gif() -> &'static z_loaned_encoding_t {
    Encoding::IMAGE_GIF.as_loaned_c_type_ref()
}
/// A BitMap (BMP) image.
///
/// Constant alias for string: `"image/bmp"`.
#[no_mangle]
pub extern "C" fn z_encoding_image_bmp() -> &'static z_loaned_encoding_t {
    Encoding::IMAGE_BMP.as_loaned_c_type_ref()
}
/// A Web Portable (WebP) image.
///
///  Constant alias for string: `"image/webp"`.
#[no_mangle]
pub extern "C" fn z_encoding_image_webp() -> &'static z_loaned_encoding_t {
    Encoding::IMAGE_WEBP.as_loaned_c_type_ref()
}
/// An XML file intended to be consumed by an application..
///
/// Constant alias for string: `"application/xml"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_xml() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_XML.as_loaned_c_type_ref()
}
/// An encoded a list of tuples, each consisting of a name and a value.
///
/// Constant alias for string: `"application/x-www-form-urlencoded"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_x_www_form_urlencoded() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_X_WWW_FORM_URLENCODED.as_loaned_c_type_ref()
}
/// An HTML file.
///
/// Constant alias for string: `"text/html"`.
#[no_mangle]
pub extern "C" fn z_encoding_text_html() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_HTML.as_loaned_c_type_ref()
}
/// An XML file that is human readable.
///
/// Constant alias for string: `"text/xml"`.
#[no_mangle]
pub extern "C" fn z_encoding_text_xml() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_XML.as_loaned_c_type_ref()
}
/// A CSS file.
///
/// Constant alias for string: `"text/css"`.
#[no_mangle]
pub extern "C" fn z_encoding_text_css() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_CSS.as_loaned_c_type_ref()
}
/// A JavaScript file.
///
/// Constant alias for string: `"text/javascript"`.
#[no_mangle]
pub extern "C" fn z_encoding_text_javascript() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_JAVASCRIPT.as_loaned_c_type_ref()
}
/// A MarkDown file.
///
/// Constant alias for string: `"text/markdown"`.
#[no_mangle]
pub extern "C" fn z_encoding_text_markdown() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_MARKDOWN.as_loaned_c_type_ref()
}
/// A CSV file.
///
/// Constant alias for string: `"text/csv"`.
#[no_mangle]
pub extern "C" fn z_encoding_text_csv() -> &'static z_loaned_encoding_t {
    Encoding::TEXT_CSV.as_loaned_c_type_ref()
}
/// An application-specific SQL query.
///
/// Constant alias for string: `"application/sql"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_sql() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_SQL.as_loaned_c_type_ref()
}
/// Constrained Application Protocol (CoAP) data intended for CoAP-to-HTTP and HTTP-to-CoAP proxies.
///
/// Constant alias for string: `"application/coap-payload"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_coap_payload() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_COAP_PAYLOAD.as_loaned_c_type_ref()
}
/// Defines a JSON document structure for expressing a sequence of operations to apply to a JSON document.
///
/// Constant alias for string: `"application/json-patch+json"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_json_patch_json() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_JSON_PATCH_JSON.as_loaned_c_type_ref()
}
/// A JSON text sequence consists of any number of JSON texts, all encoded in UTF-8.
///
/// Constant alias for string: `"application/json-seq"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_json_seq() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_JSON_SEQ.as_loaned_c_type_ref()
}
/// A JSONPath defines a string syntax for selecting and extracting JSON values from within a given JSON value.
///
/// Constant alias for string: `"application/jsonpath"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_jsonpath() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_JSONPATH.as_loaned_c_type_ref()
}
/// A JSON Web Token (JWT).
///
/// Constant alias for string: `"application/jwt"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_jwt() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_JWT.as_loaned_c_type_ref()
}
/// An application-specific MPEG-4 encoded data, either audio or video.
///
/// Constant alias for string: `"application/mp4"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_mp4() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_MP4.as_loaned_c_type_ref()
}
/// A SOAP 1.2 message serialized as XML 1.0.
///
/// Constant alias for string: `"application/soap+xml"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_soap_xml() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_SOAP_XML.as_loaned_c_type_ref()
}
/// A YANG-encoded data commonly used by the Network Configuration Protocol (NETCONF).
///
/// Constant alias for string: `"application/yang"`.
#[no_mangle]
pub extern "C" fn z_encoding_application_yang() -> &'static z_loaned_encoding_t {
    Encoding::APPLICATION_YANG.as_loaned_c_type_ref()
}
/// A MPEG-4 Advanced Audio Coding (AAC) media.
///
/// Constant alias for string: `"audio/aac"`.
#[no_mangle]
pub extern "C" fn z_encoding_audio_aac() -> &'static z_loaned_encoding_t {
    Encoding::AUDIO_AAC.as_loaned_c_type_ref()
}
/// A Free Lossless Audio Codec (FLAC) media.
///
/// Constant alias for string: `"audio/flac"`.
#[no_mangle]
pub extern "C" fn z_encoding_audio_flac() -> &'static z_loaned_encoding_t {
    Encoding::AUDIO_FLAC.as_loaned_c_type_ref()
}
/// An audio codec defined in MPEG-1, MPEG-2, MPEG-4, or registered at the MP4 registration authority.
///
/// Constant alias for string: `"audio/mp4"`.
#[no_mangle]
pub extern "C" fn z_encoding_audio_mp4() -> &'static z_loaned_encoding_t {
    Encoding::AUDIO_MP4.as_loaned_c_type_ref()
}
/// An Ogg-encapsulated audio stream.
///
/// Constant alias for string: `"audio/ogg"`.
#[no_mangle]
pub extern "C" fn z_encoding_audio_ogg() -> &'static z_loaned_encoding_t {
    Encoding::AUDIO_OGG.as_loaned_c_type_ref()
}
/// A Vorbis-encoded audio stream.
///
/// Constant alias for string: `"audio/vorbis"`.
#[no_mangle]
pub extern "C" fn z_encoding_audio_vorbis() -> &'static z_loaned_encoding_t {
    Encoding::AUDIO_VORBIS.as_loaned_c_type_ref()
}
/// A h261-encoded video stream.
///
/// Constant alias for string: `"video/h261"`.
#[no_mangle]
pub extern "C" fn z_encoding_video_h261() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_H261.as_loaned_c_type_ref()
}
/// A h263-encoded video stream.
///
/// Constant alias for string: `"video/h263"`.
#[no_mangle]
pub extern "C" fn z_encoding_video_h263() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_H263.as_loaned_c_type_ref()
}
/// A h264-encoded video stream.
///
/// Constant alias for string: `"video/h264"`.
#[no_mangle]
pub extern "C" fn z_encoding_video_h264() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_H264.as_loaned_c_type_ref()
}
/// A h265-encoded video stream.
///
/// Constant alias for string: `"video/h265"`.
#[no_mangle]
pub extern "C" fn z_encoding_video_h265() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_H265.as_loaned_c_type_ref()
}
/// A h266-encoded video stream.
///
/// Constant alias for string: `"video/h266"`.
#[no_mangle]
pub extern "C" fn z_encoding_video_h266() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_H266.as_loaned_c_type_ref()
}
/// A video codec defined in MPEG-1, MPEG-2, MPEG-4, or registered at the MP4 registration authority.
///
/// Constant alias for string: `"video/mp4"`.
#[no_mangle]
pub extern "C" fn z_encoding_video_mp4() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_MP4.as_loaned_c_type_ref()
}
/// An Ogg-encapsulated video stream.
///
/// Constant alias for string: `"video/ogg"`.
#[no_mangle]
pub extern "C" fn z_encoding_video_ogg() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_OGG.as_loaned_c_type_ref()
}
/// An uncompressed, studio-quality video stream.
///
/// Constant alias for string: `"video/raw"`.
#[no_mangle]
pub extern "C" fn z_encoding_video_raw() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_RAW.as_loaned_c_type_ref()
}
/// A VP8-encoded video stream.
///
/// Constant alias for string: `"video/vp8"`.
#[no_mangle]
pub extern "C" fn z_encoding_video_vp8() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_VP8.as_loaned_c_type_ref()
}
/// A VP9-encoded video stream.
///
/// Constant alias for string: `"video/vp9"`.
#[no_mangle]
pub extern "C" fn z_encoding_video_vp9() -> &'static z_loaned_encoding_t {
    Encoding::VIDEO_VP9.as_loaned_c_type_ref()
}
