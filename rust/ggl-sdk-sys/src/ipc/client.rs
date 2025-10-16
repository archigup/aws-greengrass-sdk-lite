// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    GglArena, GglBufList, GglBuffer, GglError, GglList, GglMap, GglObject,
};
use core::ffi::c_void;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

unsafe extern "C" {
    /// Connect to the Greengrass Nucleus from a component.
    /// Not thread-safe due to use of getenv.
    pub fn ggipc_connect() -> GglError;

    /// Connect to a GG-IPC socket with a given SVCUID token.
    pub fn ggipc_connect_with_token(
        socket_path: GglBuffer,
        auth_token: GglBuffer,
    ) -> GglError;
}

/// Handle for referring to a subscription created by an IPC call
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GgIpcSubscriptionHandle {
    pub val: u32,
}

unsafe extern "C" {
    /// Close a subscription returned by an IPC call
    pub fn ggipc_close_subscription(handle: GgIpcSubscriptionHandle);

    /// Publish a message to a local topic in JSON format
    pub fn ggipc_publish_to_topic_json(
        topic: GglBuffer,
        payload: GglMap,
    ) -> GglError;

    /// Publish a message to a local topic in binary format
    /// Usage may incur memory overhead over using `ggipc_publish_to_topic_b64`
    pub fn ggipc_publish_to_topic_binary(
        topic: GglBuffer,
        payload: GglBuffer,
    ) -> GglError;

    /// Publish a message to a local topic in binary format
    /// Payload must be already base64 encoded.
    pub fn ggipc_publish_to_topic_binary_b64(
        topic: GglBuffer,
        b64_payload: GglBuffer,
    ) -> GglError;
}

pub type GgIpcSubscribeToTopicCallback = unsafe extern "C" fn(
    ctx: *mut c_void,
    topic: GglBuffer,
    payload: GglObject,
    handle: GgIpcSubscriptionHandle,
);

unsafe extern "C" {
    /// Subscribe to messages on a local topic
    /// Payload will be a map for json messages and a buffer for binary
    /// messages.
    pub fn ggipc_subscribe_to_topic(
        topic: GglBuffer,
        callback: GgIpcSubscribeToTopicCallback,
        ctx: *mut c_void,
        handle: *mut GgIpcSubscriptionHandle,
    ) -> GglError;

    /// Publish an MQTT message to AWS IoT Core on a topic
    /// Usage may incur memory overhead over using
    /// `ggipc_publish_to_iot_core_b64`
    pub fn ggipc_publish_to_iot_core(
        topic_name: GglBuffer,
        payload: GglBuffer,
        qos: u8,
    ) -> GglError;

    /// Publish an MQTT message to AWS IoT Core on a topic
    /// Payload must be already base64 encoded.
    pub fn ggipc_publish_to_iot_core_b64(
        topic_name: GglBuffer,
        b64_payload: GglBuffer,
        qos: u8,
    ) -> GglError;
}

pub type GgIpcSubscribeToIotCoreCallback = unsafe extern "C" fn(
    ctx: *mut c_void,
    topic: GglBuffer,
    payload: GglBuffer,
    handle: GgIpcSubscriptionHandle,
);

unsafe extern "C" {
    /// Subscribe to MQTT messages from AWS IoT Core on a topic or topic filter
    pub fn ggipc_subscribe_to_iot_core(
        topic_filter: GglBuffer,
        qos: u8,
        callback: GgIpcSubscribeToIotCoreCallback,
        ctx: *mut c_void,
        handle: *mut GgIpcSubscriptionHandle,
    ) -> GglError;

    /// Get a configuration value for a component on the core device
    pub fn ggipc_get_config(
        key_path: GglBufList,
        component_name: *const GglBuffer,
        alloc: *mut GglArena,
        value: *mut GglObject,
    ) -> GglError;

    /// Get a string-typed configuration value for a component on the core
    /// device `value` must point to a buffer large enough to hold the result,
    /// and will be updated to the result string.
    /// Alternative API to ggipc_get_config for string type values.
    pub fn ggipc_get_config_str(
        key_path: GglBufList,
        component_name: *const GglBuffer,
        value: *mut GglBuffer,
    ) -> GglError;

    /// Update a configuration value for this component on the core device
    pub fn ggipc_update_config(
        key_path: GglBufList,
        timestamp: *const timespec,
        value_to_merge: GglObject,
    ) -> GglError;
}

/// Component state values for UpdateState
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum GglComponentState {
    GGL_COMPONENT_STATE_RUNNING = 0,
    GGL_COMPONENT_STATE_ERRORED = 1,
}

unsafe extern "C" {
    /// Update the state of this component
    pub fn ggipc_update_state(state: GglComponentState) -> GglError;

    /// Restart a component on the core device
    pub fn ggipc_restart_component(component_name: GglBuffer) -> GglError;
}

pub type GgIpcSubscribeToConfigurationUpdateCallback = unsafe extern "C" fn(
    ctx: *mut c_void,
    component_name: GglBuffer,
    key_path: GglList,
    handle: GgIpcSubscriptionHandle,
);

unsafe extern "C" {
    /// Subscribe to configuration updates for a component
    /// Pass NULL for component_name to refer to current component.
    pub fn ggipc_subscribe_to_configuration_update(
        component_name: *const GglBuffer,
        key_path: GglBufList,
        callback: GgIpcSubscribeToConfigurationUpdateCallback,
        ctx: *mut c_void,
        handle: *mut GgIpcSubscriptionHandle,
    ) -> GglError;
}
