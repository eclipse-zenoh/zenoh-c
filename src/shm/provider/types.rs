//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

use prebindgen_proc_macro::prebindgen;
use std::mem::MaybeUninit;

use zenoh::shm::{
    AllocAlignment, BufAllocResult, BufLayoutAllocResult, ChunkAllocResult, MemoryLayout,
    ZAllocError, ZLayoutError,
};

use super::chunk::z_allocated_chunk_t;
use crate::{
    result::{z_result_t, Z_EINVAL, Z_OK},
    shm::buffer::zshmmut::z_internal_shm_mut_null,
    transmute::{IntoCType, LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_loaned_memory_layout_t, z_moved_chunk_alloc_result_t, z_moved_memory_layout_t,
    z_owned_chunk_alloc_result_t, z_owned_memory_layout_t, z_owned_shm_mut_t,
};

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Allocation errors
#[prebindgen]
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_alloc_error_t {
    /// Defragmentation needed.
    NEED_DEFRAGMENT,
    /// The provider is out of memory.
    OUT_OF_MEMORY,
    /// Other error.
    OTHER,
}

impl From<ZAllocError> for z_alloc_error_t {
    #[inline]
    fn from(value: ZAllocError) -> Self {
        match value {
            ZAllocError::NeedDefragment => z_alloc_error_t::NEED_DEFRAGMENT,
            ZAllocError::OutOfMemory => z_alloc_error_t::OUT_OF_MEMORY,
            ZAllocError::Other => z_alloc_error_t::OTHER,
        }
    }
}

impl From<z_alloc_error_t> for ZAllocError {
    #[inline]
    fn from(value: z_alloc_error_t) -> Self {
        match value {
            z_alloc_error_t::NEED_DEFRAGMENT => ZAllocError::NeedDefragment,
            z_alloc_error_t::OUT_OF_MEMORY => ZAllocError::OutOfMemory,
            z_alloc_error_t::OTHER => ZAllocError::Other,
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Layouting errors
#[prebindgen]
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_layout_error_t {
    /// Layout arguments are incorrect.
    INCORRECT_LAYOUT_ARGS,
    /// Layout incompatible with provider.
    PROVIDER_INCOMPATIBLE_LAYOUT,
}

impl From<ZLayoutError> for z_layout_error_t {
    #[inline]
    fn from(value: ZLayoutError) -> Self {
        match value {
            ZLayoutError::IncorrectLayoutArgs => z_layout_error_t::INCORRECT_LAYOUT_ARGS,
            ZLayoutError::ProviderIncompatibleLayout => {
                z_layout_error_t::PROVIDER_INCOMPATIBLE_LAYOUT
            }
        }
    }
}

impl From<z_layout_error_t> for ZLayoutError {
    #[inline]
    fn from(value: z_layout_error_t) -> Self {
        match value {
            z_layout_error_t::INCORRECT_LAYOUT_ARGS => ZLayoutError::IncorrectLayoutArgs,
            z_layout_error_t::PROVIDER_INCOMPATIBLE_LAYOUT => {
                ZLayoutError::ProviderIncompatibleLayout
            }
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An AllocAlignment.
#[prebindgen]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_alloc_alignment_t {
    pub pow: u8,
}

decl_c_type!(copy(z_alloc_alignment_t, AllocAlignment),);

decl_c_type_inequal!(
    owned(z_owned_memory_layout_t, option MemoryLayout),
    loaned(z_loaned_memory_layout_t),
);

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Creates a new Memory Layout.
#[prebindgen]
pub fn z_memory_layout_new(
    this: &mut MaybeUninit<z_owned_memory_layout_t>,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_result_t {
    fn create_memory_layout(
        size: usize,
        alignment: z_alloc_alignment_t,
    ) -> Result<MemoryLayout, ZLayoutError> {
        let alignment = AllocAlignment::new(alignment.pow)?;
        MemoryLayout::new(size, alignment)
    }

    match create_memory_layout(size, alignment) {
        Ok(layout) => {
            this.as_rust_type_mut_uninit().write(Some(layout));
            Z_OK
        }
        Err(e) => {
            crate::report_error!("{:?}", e);
            Z_EINVAL
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs Memory Layout in its gravestone value.
#[prebindgen]
pub fn z_internal_memory_layout_null(this_: &mut MaybeUninit<z_owned_memory_layout_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns ``true`` if `this` is valid.
#[prebindgen]
pub fn z_internal_memory_layout_check(this_: &z_owned_memory_layout_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Borrows Memory Layout.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_memory_layout_loan(
    this_: &z_owned_memory_layout_t,
) -> &z_loaned_memory_layout_t {
    this_.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Moves Memory Layout.
#[prebindgen("move")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_memory_layout_move(
    this_: &mut z_owned_memory_layout_t,
) -> &mut z_moved_memory_layout_t {
    std::mem::transmute(this_)
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Deletes Memory Layout.
#[prebindgen]
pub fn z_memory_layout_drop(this_: &mut z_moved_memory_layout_t) {
    let _ = this_.take_rust_type();
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Extract data from Memory Layout.
#[prebindgen]
pub fn z_memory_layout_get_data(
    this: &z_loaned_memory_layout_t,
    out_size: &mut MaybeUninit<usize>,
    out_alignment: &mut MaybeUninit<z_alloc_alignment_t>,
) {
    let layout = this.as_rust_type_ref();
    out_size.write(layout.size().into());
    out_alignment.write(layout.alignment().into_c_type());
}

decl_c_type!(
    owned(z_owned_chunk_alloc_result_t, option ChunkAllocResult),
);

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Creates a new Chunk Alloc Result with Ok value.
#[prebindgen]
pub fn z_chunk_alloc_result_new_ok(
    this: &mut MaybeUninit<z_owned_chunk_alloc_result_t>,
    allocated_chunk: z_allocated_chunk_t,
) -> z_result_t {
    match allocated_chunk.try_into() {
        Ok(chunk) => {
            this.as_rust_type_mut_uninit().write(Some(Ok(chunk)));
            Z_OK
        }
        Err(_) => Z_EINVAL,
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Creates a new Chunk Alloc Result with Error value.
#[prebindgen]
pub fn z_chunk_alloc_result_new_error(
    this: &mut MaybeUninit<z_owned_chunk_alloc_result_t>,
    alloc_error: z_alloc_error_t,
) {
    this.as_rust_type_mut_uninit()
        .write(Some(Err(alloc_error.into())));
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs Chunk Alloc Result in its gravestone value.
#[prebindgen]
pub fn z_internal_chunk_alloc_result_null(
    this_: &mut MaybeUninit<z_owned_chunk_alloc_result_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @return ``true`` if `this` is valid.
#[prebindgen]
pub fn z_internal_chunk_alloc_result_check(
    this_: &z_owned_chunk_alloc_result_t,
) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Deletes Chunk Alloc Result.
#[prebindgen]
pub fn z_chunk_alloc_result_drop(this_: &mut z_moved_chunk_alloc_result_t) {
    let _ = this_.take_rust_type();
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Status of SHM buffer allocation operation.
#[prebindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum zc_buf_alloc_status_t {
    /// Allocation ok
    OK = 0,
    /// Allocation error
    ALLOC_ERROR = 1,
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A result of SHM buffer allocation operation.
#[prebindgen]
#[repr(C)]
pub struct z_buf_alloc_result_t {
    status: zc_buf_alloc_status_t,
    buf: z_owned_shm_mut_t,
    error: z_alloc_error_t,
}

impl From<BufAllocResult> for z_buf_alloc_result_t {
    fn from(value: BufAllocResult) -> Self {
        let mut buf: MaybeUninit<z_owned_shm_mut_t> = MaybeUninit::uninit();
        match value {
            Ok(val) => {
                buf.as_rust_type_mut_uninit().write(Some(val));
                Self {
                    status: zc_buf_alloc_status_t::OK,
                    // SAFETY: this is safe because buf is gravestone-initialized above
                    buf: unsafe { buf.assume_init() },
                    error: z_alloc_error_t::OTHER,
                }
            }
            Err(error) => {
                z_internal_shm_mut_null(&mut buf);
                Self {
                    status: zc_buf_alloc_status_t::ALLOC_ERROR,
                    // SAFETY: this is safe because buf is gravestone-initialized above
                    buf: unsafe { buf.assume_init() },
                    error: error.into(),
                }
            }
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Status of SHM buffer layouting + allocation operation.
#[prebindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum zc_buf_layout_alloc_status_t {
    /// Allocation ok
    OK = 0,
    /// Allocation error
    ALLOC_ERROR = 1,
    /// Layouting error
    LAYOUT_ERROR = 2,
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A result of SHM buffer layouting + allocation operation.
#[prebindgen]
#[repr(C)]
pub struct z_buf_layout_alloc_result_t {
    status: zc_buf_layout_alloc_status_t,
    buf: z_owned_shm_mut_t,
    alloc_error: z_alloc_error_t,
    layout_error: z_layout_error_t,
}

impl From<BufLayoutAllocResult> for z_buf_layout_alloc_result_t {
    fn from(value: BufLayoutAllocResult) -> Self {
        let mut buf: MaybeUninit<z_owned_shm_mut_t> = MaybeUninit::uninit();
        match value {
            Ok(val) => {
                buf.as_rust_type_mut_uninit().write(Some(val));
                Self {
                    status: zc_buf_layout_alloc_status_t::OK,
                    // SAFETY: this is safe because buf is initialized above
                    buf: unsafe { buf.assume_init() },
                    alloc_error: z_alloc_error_t::OTHER,
                    layout_error: z_layout_error_t::PROVIDER_INCOMPATIBLE_LAYOUT,
                }
            }
            Err(error) => {
                z_internal_shm_mut_null(&mut buf);
                match error {
                    zenoh::shm::ZLayoutAllocError::Alloc(alloc) => {
                        Self {
                            status: zc_buf_layout_alloc_status_t::ALLOC_ERROR,
                            // SAFETY: this is safe because buf is gravestone-initialized above
                            buf: unsafe { buf.assume_init() },
                            alloc_error: alloc.into(),
                            layout_error: z_layout_error_t::PROVIDER_INCOMPATIBLE_LAYOUT,
                        }
                    }
                    zenoh::shm::ZLayoutAllocError::Layout(layout) => {
                        Self {
                            status: zc_buf_layout_alloc_status_t::LAYOUT_ERROR,
                            // SAFETY: this is safe because buf is gravestone-initialized above
                            buf: unsafe { buf.assume_init() },
                            alloc_error: z_alloc_error_t::OTHER,
                            layout_error: layout.into(),
                        }
                    }
                }
            }
        }
    }
}
