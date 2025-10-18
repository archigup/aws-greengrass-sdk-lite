// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::{
    env,
    sync::{Mutex, OnceLock},
};

pub mod c;
mod error;
mod object;

pub use error::Error;
pub use object::{List, Map, Object, KV};

static INIT: OnceLock<()> = OnceLock::new();
static CONNECTED: Mutex<bool> = Mutex::new(false);

#[non_exhaustive]
pub struct Sdk {}

impl Sdk {
    pub fn init() -> Self {
        INIT.get_or_init(|| unsafe { c::ggl_sdk_init() });
        Self {}
    }

    pub fn connect(&self) -> Result<(), Error> {
        let svcuid = env::var("SVCUID").map_err(|_| Error::Config)?;
        let socket_path =
            env::var("AWS_GG_NUCLEUS_DOMAIN_SOCKET_FILEPATH_FOR_COMPONENT")
                .map_err(|_| Error::Config)?;

        self.connect_with_token(&socket_path, &svcuid)
    }

    pub fn connect_with_token(
        &self,
        socket_path: &str,
        auth_token: &str,
    ) -> Result<(), Error> {
        let mut connected = CONNECTED.lock().unwrap();
        if *connected {
            return Ok(());
        }

        let socket_buf = c::GglBuffer {
            data: socket_path.as_ptr() as *mut u8,
            len: socket_path.len(),
        };
        let token_buf = c::GglBuffer {
            data: auth_token.as_ptr() as *mut u8,
            len: auth_token.len(),
        };

        Result::<(), Error>::from(unsafe {
            c::ggipc_connect_with_token(socket_buf, token_buf)
        })?;

        *connected = true;
        Ok(())
    }

    pub fn publish_to_topic_json(
        &self,
        topic: &str,
        payload: Map,
    ) -> Result<(), Error> {
        let topic_buf = c::GglBuffer {
            data: topic.as_ptr() as *mut u8,
            len: topic.len(),
        };

        unsafe { c::ggipc_publish_to_topic_json(topic_buf, payload.as_raw()) }
            .into()
    }

    pub fn publish_to_topic_binary(
        &self,
        topic: &str,
        payload: &[u8],
    ) -> Result<(), Error> {
        let topic_buf = c::GglBuffer {
            data: topic.as_ptr() as *mut u8,
            len: topic.len(),
        };
        let payload_buf = c::GglBuffer {
            data: payload.as_ptr() as *mut u8,
            len: payload.len(),
        };

        unsafe { c::ggipc_publish_to_topic_binary(topic_buf, payload_buf) }
            .into()
    }
}
