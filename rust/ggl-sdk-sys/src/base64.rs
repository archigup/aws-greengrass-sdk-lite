// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::{GglArena, GglBuffer, GglError};

unsafe extern "C" {
    /// Convert a base64 buffer to its decoded data.
    /// Target must be large enough to hold decoded value.
    pub fn ggl_base64_decode(base64: GglBuffer, target: *mut GglBuffer)
        -> bool;

    /// Convert a base64 buffer to its decoded data in place.
    pub fn ggl_base64_decode_in_place(target: *mut GglBuffer) -> bool;

    /// Encode a buffer into base64.
    pub fn ggl_base64_encode(
        buf: GglBuffer,
        alloc: *mut GglArena,
        result: *mut GglBuffer,
    ) -> GglError;
}
