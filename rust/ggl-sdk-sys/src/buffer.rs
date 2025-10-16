// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::GglError;
use core::ffi::c_char;

/// A fixed buffer of bytes. Possibly a string.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GglBuffer {
    pub data: *mut u8,
    pub len: usize,
}

/// An array of `GglBuffer`
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GglBufList {
    pub bufs: *mut GglBuffer,
    pub len: usize,
}

unsafe extern "C" {
    /// Convert null-terminated string to buffer
    pub fn ggl_buffer_from_null_term(str: *const c_char) -> GglBuffer;

    /// Returns whether two buffers have identical content.
    pub fn ggl_buffer_eq(buf1: GglBuffer, buf2: GglBuffer) -> bool;

    /// Returns whether the buffer has the given prefix.
    pub fn ggl_buffer_has_prefix(buf: GglBuffer, prefix: GglBuffer) -> bool;

    /// Removes a prefix. Returns whether the prefix was removed.
    pub fn ggl_buffer_remove_prefix(
        buf: *mut GglBuffer,
        prefix: GglBuffer,
    ) -> bool;

    /// Returns whether the buffer has the given suffix.
    pub fn ggl_buffer_has_suffix(buf: GglBuffer, suffix: GglBuffer) -> bool;

    /// Removes a suffix. Returns whether the suffix was removed.
    pub fn ggl_buffer_remove_suffix(
        buf: *mut GglBuffer,
        suffix: GglBuffer,
    ) -> bool;

    /// Returns whether the buffer contains the given substring.
    /// Outputs start index if non-null.
    pub fn ggl_buffer_contains(
        buf: GglBuffer,
        substring: GglBuffer,
        start: *mut usize,
    ) -> bool;

    /// Returns substring of buffer from start to end.
    /// The result is the overlap between the start to end range and the input
    /// bounds.
    pub fn ggl_buffer_substr(
        buf: GglBuffer,
        start: usize,
        end: usize,
    ) -> GglBuffer;

    /// Parse an integer from a string
    pub fn ggl_str_to_int64(str: GglBuffer, value: *mut i64) -> GglError;
}
