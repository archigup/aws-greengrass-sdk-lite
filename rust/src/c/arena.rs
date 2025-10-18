// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::{GglBuffer, GglError, GglObject};
use core::ffi::c_void;

/// Arena allocator backed by fixed buffer
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GglArena {
    pub mem: *mut u8,
    pub capacity: u32,
    pub index: u32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GglArenaState {
    pub index: u32,
}

/// Obtain an initialized `GglArena` backed by `buf`.
#[inline]
pub fn ggl_arena_init(buf: GglBuffer) -> GglArena {
    GglArena {
        mem: buf.data,
        capacity: if buf.len <= u32::MAX as usize {
            buf.len as u32
        } else {
            u32::MAX
        },
        index: 0,
    }
}

unsafe extern "C" {
    /// Allocate `size` bytes with given alignment from an arena.
    pub fn ggl_arena_alloc(
        arena: *mut GglArena,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Resize ptr's allocation (must be the last allocated ptr).
    pub fn ggl_arena_resize_last(
        arena: *mut GglArena,
        ptr: *const c_void,
        old_size: usize,
        size: usize,
    ) -> GglError;

    /// Returns true if arena's mem contains ptr.
    pub fn ggl_arena_owns(arena: *const GglArena, ptr: *const c_void) -> bool;

    /// Allocates remaining space into a buffer.
    pub fn ggl_arena_alloc_rest(arena: *mut GglArena) -> GglBuffer;

    /// Modifies all of an object's references to point into a given arena
    pub fn ggl_arena_claim_obj(
        obj: *mut GglObject,
        arena: *mut GglArena,
    ) -> GglError;

    /// Modifies an buffer to point into a given arena
    pub fn ggl_arena_claim_buf(
        buf: *mut GglBuffer,
        arena: *mut GglArena,
    ) -> GglError;

    /// Modifies only the buffers of an object to point into a given arena
    pub fn ggl_arena_claim_obj_bufs(
        obj: *mut GglObject,
        arena: *mut GglArena,
    ) -> GglError;
}
