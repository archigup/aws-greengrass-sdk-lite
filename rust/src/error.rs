// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::c;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Error {
    Failure = c::GglError::GGL_ERR_FAILURE as u32,
    Retry = c::GglError::GGL_ERR_RETRY as u32,
    Busy = c::GglError::GGL_ERR_BUSY as u32,
    Fatal = c::GglError::GGL_ERR_FATAL as u32,
    Invalid = c::GglError::GGL_ERR_INVALID as u32,
    Unsupported = c::GglError::GGL_ERR_UNSUPPORTED as u32,
    Parse = c::GglError::GGL_ERR_PARSE as u32,
    Range = c::GglError::GGL_ERR_RANGE as u32,
    Nomem = c::GglError::GGL_ERR_NOMEM as u32,
    Noconn = c::GglError::GGL_ERR_NOCONN as u32,
    Nodata = c::GglError::GGL_ERR_NODATA as u32,
    Noentry = c::GglError::GGL_ERR_NOENTRY as u32,
    Config = c::GglError::GGL_ERR_CONFIG as u32,
    Remote = c::GglError::GGL_ERR_REMOTE as u32,
    Expected = c::GglError::GGL_ERR_EXPECTED as u32,
    Timeout = c::GglError::GGL_ERR_TIMEOUT as u32,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = unsafe {
            let ptr = c::ggl_strerror((*self).into());
            let cstr = std::ffi::CStr::from_ptr(ptr);
            std::str::from_utf8_unchecked(cstr.to_bytes())
        };
        write!(f, "{}", s)
    }
}

impl From<Error> for c::GglError {
    fn from(val: Error) -> Self {
        match val {
            Error::Failure => c::GglError::GGL_ERR_FAILURE,
            Error::Retry => c::GglError::GGL_ERR_RETRY,
            Error::Busy => c::GglError::GGL_ERR_BUSY,
            Error::Fatal => c::GglError::GGL_ERR_FATAL,
            Error::Invalid => c::GglError::GGL_ERR_INVALID,
            Error::Unsupported => c::GglError::GGL_ERR_UNSUPPORTED,
            Error::Parse => c::GglError::GGL_ERR_PARSE,
            Error::Range => c::GglError::GGL_ERR_RANGE,
            Error::Nomem => c::GglError::GGL_ERR_NOMEM,
            Error::Noconn => c::GglError::GGL_ERR_NOCONN,
            Error::Nodata => c::GglError::GGL_ERR_NODATA,
            Error::Noentry => c::GglError::GGL_ERR_NOENTRY,
            Error::Config => c::GglError::GGL_ERR_CONFIG,
            Error::Remote => c::GglError::GGL_ERR_REMOTE,
            Error::Expected => c::GglError::GGL_ERR_EXPECTED,
            Error::Timeout => c::GglError::GGL_ERR_TIMEOUT,
        }
    }
}

impl From<c::GglError> for Result<(), Error> {
    fn from(err: c::GglError) -> Self {
        match err {
            c::GglError::GGL_ERR_OK => Ok(()),
            c::GglError::GGL_ERR_FAILURE => Err(Error::Failure),
            c::GglError::GGL_ERR_RETRY => Err(Error::Retry),
            c::GglError::GGL_ERR_BUSY => Err(Error::Busy),
            c::GglError::GGL_ERR_FATAL => Err(Error::Fatal),
            c::GglError::GGL_ERR_INVALID => Err(Error::Invalid),
            c::GglError::GGL_ERR_UNSUPPORTED => Err(Error::Unsupported),
            c::GglError::GGL_ERR_PARSE => Err(Error::Parse),
            c::GglError::GGL_ERR_RANGE => Err(Error::Range),
            c::GglError::GGL_ERR_NOMEM => Err(Error::Nomem),
            c::GglError::GGL_ERR_NOCONN => Err(Error::Noconn),
            c::GglError::GGL_ERR_NODATA => Err(Error::Nodata),
            c::GglError::GGL_ERR_NOENTRY => Err(Error::Noentry),
            c::GglError::GGL_ERR_CONFIG => Err(Error::Config),
            c::GglError::GGL_ERR_REMOTE => Err(Error::Remote),
            c::GglError::GGL_ERR_EXPECTED => Err(Error::Expected),
            c::GglError::GGL_ERR_TIMEOUT => Err(Error::Timeout),
        }
    }
}
