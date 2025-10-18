// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use core::ffi::c_char;

/// GGL error codes, representing class of error.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum GglError {
    /// Success
    GGL_ERR_OK = 0,
    /// Generic failure
    GGL_ERR_FAILURE = 1,
    /// Failure, can be retried
    GGL_ERR_RETRY = 2,
    /// Request cannot be handled at the time
    GGL_ERR_BUSY = 3,
    /// System is in irrecoverably broken state
    GGL_ERR_FATAL = 4,
    /// Request is invalid or malformed
    GGL_ERR_INVALID = 5,
    /// Request is unsupported
    GGL_ERR_UNSUPPORTED = 6,
    /// Request data invalid
    GGL_ERR_PARSE = 7,
    /// Request or data outside of allowable range
    GGL_ERR_RANGE = 8,
    /// Insufficient memory
    GGL_ERR_NOMEM = 9,
    /// No connection
    GGL_ERR_NOCONN = 10,
    /// No more data available
    GGL_ERR_NODATA = 11,
    /// Unknown entry or target requested
    GGL_ERR_NOENTRY = 12,
    /// Invalid or missing configuration
    GGL_ERR_CONFIG = 13,
    /// Received remote error
    GGL_ERR_REMOTE = 14,
    /// Expected non-ok status
    GGL_ERR_EXPECTED = 15,
    /// Request timed out
    GGL_ERR_TIMEOUT = 16,
}

unsafe extern "C" {
    /// Get string representation of error
    pub fn ggl_strerror(err: GglError) -> *const c_char;
}
