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

use std::mem::MaybeUninit;

use zenoh::{
    internal::zerror,
    shm::{
        AllocAlignment, BufAllocResult, BufLayoutAllocResult, ChunkAllocResult, MemoryLayout,
        ZAllocError, ZLayoutError,
    },
};

use super::chunk::z_allocated_chunk_t;
use crate::{
    result::{z_result_t, Z_EINVAL, Z_OK},
    shm::buffer::zshmmut::z_shm_mut_null,
    transmute::{IntoCType, LoanedCTypeRef, RustTypeRef, RustTypeRefUninit},
    z_loaned_chunk_alloc_result_t, z_loaned_memory_layout_t, z_moved_chunk_alloc_result_t,
    z_moved_memory_layout_t, z_owned_chunk_alloc_result_t, z_owned_memory_layout_t,
    z_owned_shm_mut_t,
};

/// Allocation errors
///
///     - **NEED_DEFRAGMENT**: defragmentation needed
///     - **OUT_OF_MEMORY**: the provider is out of memory
///     - **OTHER**: other error
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_alloc_error_t {
    NEED_DEFRAGMENT,
    OUT_OF_MEMORY,
    OTHER,
}

impl From<ZAllocError> for z_alloc_error_t {
    #[inline]
    fn from(value: ZAllocError) -> Self {
        match value {
            ZAllocError::NeedDefragment => z_alloc_error_t::NEED_DEFRAGMENT,
            ZAllocError::OutOfMemory => z_alloc_error_t::OUT_OF_MEMORY,
            ZAllocError::Other(_) => z_alloc_error_t::OTHER,
        }
    }
}

impl From<z_alloc_error_t> for ZAllocError {
    #[inline]
    fn from(value: z_alloc_error_t) -> Self {
        match value {
            z_alloc_error_t::NEED_DEFRAGMENT => ZAllocError::NeedDefragment,
            z_alloc_error_t::OUT_OF_MEMORY => ZAllocError::OutOfMemory,
            z_alloc_error_t::OTHER => ZAllocError::Other(zerror!("other error").into()),
        }
    }
}

/// Layouting errors
///
/// INCORRECT_LAYOUT_ARGS: layout arguments are incorrect
/// PROVIDER_INCOMPATIBLE_LAYOUT: layout incompatible with provider
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_layout_error_t {
    INCORRECT_LAYOUT_ARGS,
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

/// An AllocAlignment.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_alloc_alignment_t {
    pow: u8,
}

decl_c_type!(copy(z_alloc_alignment_t, AllocAlignment),);

decl_c_type_inequal!(
    owned(z_owned_memory_layout_t, option MemoryLayout),
    loaned(z_loaned_memory_layout_t),
    moved(z_moved_memory_layout_t)
);

/// Creates a new Memory Layout
#[no_mangle]
pub extern "C" fn z_memory_layout_new(
    this: &mut MaybeUninit<z_owned_memory_layout_t>,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_result_t {
    match create_memory_layout(size, alignment) {
        Ok(layout) => {
            this.as_rust_type_mut_uninit().write(Some(layout));
            Z_OK
        }
        Err(e) => {
            tracing::error!("{:?}", e);
            Z_EINVAL
        }
    }
}

fn create_memory_layout(
    size: usize,
    alignment: z_alloc_alignment_t,
) -> Result<MemoryLayout, ZLayoutError> {
    let alignment = AllocAlignment::new(alignment.pow)?;
    MemoryLayout::new(size, alignment)
}

/// Constructs Memory Layout in its gravestone value.
#[no_mangle]
pub extern "C" fn z_memory_layout_null(this: &mut MaybeUninit<z_owned_memory_layout_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_memory_layout_check(this: &z_owned_memory_layout_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Borrows Memory Layout
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_memory_layout_loan(
    this: &z_owned_memory_layout_t,
) -> &z_loaned_memory_layout_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Deletes Memory Layout
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn z_memory_layout_drop(this: z_moved_memory_layout_t) {}

/// Extract data from Memory Layout
#[no_mangle]
pub extern "C" fn z_memory_layout_get_data(
    out_size: &mut MaybeUninit<usize>,
    out_alignment: &mut MaybeUninit<z_alloc_alignment_t>,
    this: &z_loaned_memory_layout_t,
) {
    let layout = this.as_rust_type_ref();
    out_size.write(layout.size().into());
    out_alignment.write(layout.alignment().into_c_type());
}

decl_c_type!(
    owned(z_owned_chunk_alloc_result_t, option ChunkAllocResult),
    loaned(z_loaned_chunk_alloc_result_t),
    moved(z_moved_chunk_alloc_result_t)
);

/// Creates a new Chunk Alloc Result with Ok value
#[no_mangle]
pub extern "C" fn z_chunk_alloc_result_new_ok(
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

/// Creates a new Chunk Alloc Result with Error value
#[no_mangle]
pub extern "C" fn z_chunk_alloc_result_new_error(
    this: &mut MaybeUninit<z_owned_chunk_alloc_result_t>,
    alloc_error: z_alloc_error_t,
) {
    this.as_rust_type_mut_uninit()
        .write(Some(Err(alloc_error.into())));
}

/// Constructs Chunk Alloc Result in its gravestone value.
#[no_mangle]
pub extern "C" fn z_chunk_alloc_result_null(this: &mut MaybeUninit<z_owned_chunk_alloc_result_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_chunk_alloc_result_check(this: &z_owned_chunk_alloc_result_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Borrows Chunk Alloc Result
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_chunk_alloc_result_loan(
    this: &z_owned_chunk_alloc_result_t,
) -> &z_loaned_chunk_alloc_result_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Deletes Chunk Alloc Result
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn z_chunk_alloc_result_drop(this: z_moved_chunk_alloc_result_t) {}

#[repr(C)]
pub struct z_buf_alloc_result_t {
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
                    // SAFETY: this is safe because buf is gravestone-initialized above
                    buf: unsafe { buf.assume_init() },
                    error: z_alloc_error_t::OTHER,
                }
            }
            Err(error) => {
                z_shm_mut_null(&mut buf);
                Self {
                    // SAFETY: this is safe because buf is gravestone-initialized above
                    buf: unsafe { buf.assume_init() },
                    error: error.into(),
                }
            }
        }
    }
}

#[repr(C)]
pub struct z_buf_layout_alloc_result_t {
    buf: z_owned_shm_mut_t,
    error_is_alloc: bool,
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
                    // SAFETY: this is safe because buf is gravestone-initialized above
                    buf: unsafe { buf.assume_init() },
                    error_is_alloc: false,
                    alloc_error: z_alloc_error_t::OTHER,
                    layout_error: z_layout_error_t::PROVIDER_INCOMPATIBLE_LAYOUT,
                }
            }
            Err(error) => {
                z_shm_mut_null(&mut buf);
                match error {
                    zenoh::shm::ZLayoutAllocError::Alloc(alloc) => {
                        Self {
                            // SAFETY: this is safe because buf is gravestone-initialized above
                            buf: unsafe { buf.assume_init() },
                            error_is_alloc: true,
                            alloc_error: alloc.into(),
                            layout_error: z_layout_error_t::PROVIDER_INCOMPATIBLE_LAYOUT,
                        }
                    }
                    zenoh::shm::ZLayoutAllocError::Layout(layout) => {
                        Self {
                            // SAFETY: this is safe because buf is gravestone-initialized above
                            buf: unsafe { buf.assume_init() },
                            error_is_alloc: false,
                            alloc_error: z_alloc_error_t::OTHER,
                            layout_error: layout.into(),
                        }
                    }
                }
            }
        }
    }
}
