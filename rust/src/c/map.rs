// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::{
    GglBufList, GglBuffer, GglError, GglKV, GglMap, GglObject, GglObjectType,
    GglPresence,
};

unsafe extern "C" {
    /// Get the value corresponding with a key.
    /// Returns whether the key was found in the map.
    /// If `result` is not NULL it is set to the found value or NULL.
    pub fn ggl_map_get(
        map: GglMap,
        key: GglBuffer,
        result: *mut *mut GglObject,
    ) -> bool;

    /// Get the value from a nested map corresponding with a key path.
    /// Returns whether the key was found in the map.
    /// If `result` is not NULL it is set to the found value or NULL.
    pub fn ggl_map_get_path(
        map: GglMap,
        path: GglBufList,
        result: *mut *mut GglObject,
    ) -> bool;

    /// Construct a GglKV
    pub fn ggl_kv(key: GglBuffer, val: GglObject) -> GglKV;

    /// Get a GglKV's key
    pub fn ggl_kv_key(kv: GglKV) -> GglBuffer;

    /// Set a GglKV's key
    pub fn ggl_kv_set_key(kv: *mut GglKV, key: GglBuffer);

    /// Get a GglKV's value
    pub fn ggl_kv_val(kv: *mut GglKV) -> *mut GglObject;
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GglMapSchemaEntry {
    pub key: GglBuffer,
    pub required: GglPresence,
    pub type_: GglObjectType,
    pub value: *mut *mut GglObject,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GglMapSchema {
    pub entries: *const GglMapSchemaEntry,
    pub entry_count: usize,
}

unsafe extern "C" {
    /// Validate a map against a schema
    pub fn ggl_map_validate(map: GglMap, schema: GglMapSchema) -> GglError;
}
