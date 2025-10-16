// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::{GgIpcSubscriptionHandle, GglBuffer, GglError, GglMap};
use core::ffi::c_void;

pub type GgIpcResultCallback =
    unsafe extern "C" fn(ctx: *mut c_void, result: GglMap) -> GglError;

pub type GgIpcErrorCallback = unsafe extern "C" fn(
    ctx: *mut c_void,
    error_code: GglBuffer,
    message: GglBuffer,
) -> GglError;

unsafe extern "C" {
    pub fn ggipc_call(
        operation: GglBuffer,
        service_model_type: GglBuffer,
        params: GglMap,
        result_callback: GgIpcResultCallback,
        error_callback: GgIpcErrorCallback,
        response_ctx: *mut c_void,
    ) -> GglError;
}

pub type GgIpcSubscribeCallback = unsafe extern "C" fn(
    ctx: *mut c_void,
    aux_ctx: *mut c_void,
    handle: GgIpcSubscriptionHandle,
    service_model_type: GglBuffer,
    data: GglMap,
) -> GglError;

unsafe extern "C" {
    pub fn ggipc_subscribe(
        operation: GglBuffer,
        service_model_type: GglBuffer,
        params: GglMap,
        result_callback: GgIpcResultCallback,
        error_callback: GgIpcErrorCallback,
        response_ctx: *mut c_void,
        sub_callback: GgIpcSubscribeCallback,
        sub_callback_ctx: *mut c_void,
        sub_callback_aux_ctx: *mut c_void,
        sub_handle: *mut GgIpcSubscriptionHandle,
    ) -> GglError;
}
