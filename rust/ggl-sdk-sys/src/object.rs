// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::{GglBuffer, GglError};
use core::mem::size_of;
use core::ffi::c_void;

/// Maximum depth of an object
pub const GGL_MAX_OBJECT_DEPTH: u32 = 15;

/// Maximum subobject count for an object
pub const GGL_MAX_OBJECT_SUBOBJECTS: u32 = 255;

/// A generic object.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GglObject {
    _private: [u8; if cfg!(target_pointer_width = "32") {
        9
    } else {
        11
    }],
}

/// Type tag for `GglObject`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum GglObjectType {
    GGL_TYPE_NULL = 0,
    GGL_TYPE_BOOLEAN,
    GGL_TYPE_I64,
    GGL_TYPE_F64,
    GGL_TYPE_BUF,
    GGL_TYPE_LIST,
    GGL_TYPE_MAP,
}

/// An array of `GglObject`.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GglList {
    pub items: *mut GglObject,
    pub len: usize,
}

/// A key-value pair used for `GglMap`.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GglKV {
    _private: [u8; size_of::<*const c_void>() + 2 + size_of::<GglObject>()],
}

/// A map of UTF-8 strings to `GglObject`s.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GglMap {
    pub pairs: *mut GglKV,
    pub len: usize,
}

unsafe extern "C" {
    /// Get type of an GglObject
    pub fn ggl_obj_type(obj: GglObject) -> GglObjectType;
}

/// Null object constant
pub const GGL_OBJ_NULL: GglObject = GglObject {
    _private: [0; if cfg!(target_pointer_width = "32") {
        9
    } else {
        11
    }],
};

unsafe extern "C" {
    /// Create bool object.
    pub fn ggl_obj_bool(value: bool) -> GglObject;

    /// Get the bool represented by an object.
    /// The GglObject must be of type GGL_TYPE_BOOLEAN.
    pub fn ggl_obj_into_bool(boolean: GglObject) -> bool;

    /// Create signed integer object.
    pub fn ggl_obj_i64(value: i64) -> GglObject;

    /// Get the i64 represented by an object.
    /// The GglObject must be of type GGL_TYPE_I64.
    pub fn ggl_obj_into_i64(i64: GglObject) -> i64;

    /// Create floating point object.
    pub fn ggl_obj_f64(value: f64) -> GglObject;

    /// Get the f64 represented by an object.
    /// The GglObject must be of type GGL_TYPE_F64.
    pub fn ggl_obj_into_f64(f64: GglObject) -> f64;

    /// Create buffer object.
    pub fn ggl_obj_buf(value: GglBuffer) -> GglObject;

    /// Get the buffer represented by an object.
    /// The GglObject must be of type GGL_TYPE_BUF.
    pub fn ggl_obj_into_buf(buf: GglObject) -> GglBuffer;

    /// Create map object.
    pub fn ggl_obj_map(value: GglMap) -> GglObject;

    /// Get the map represented by an object.
    /// The GglObject must be of type GGL_TYPE_MAP.
    pub fn ggl_obj_into_map(map: GglObject) -> GglMap;

    /// Create list object.
    pub fn ggl_obj_list(value: GglList) -> GglObject;

    /// Get the list represented by an object.
    /// The GglObject must be of type GGL_TYPE_LIST.
    pub fn ggl_obj_into_list(list: GglObject) -> GglList;

    /// Calculates the memory required to claim this object with
    /// ggl_arena_claim_obj.
    pub fn ggl_obj_mem_usage(obj: GglObject, size: *mut usize) -> GglError;
}
