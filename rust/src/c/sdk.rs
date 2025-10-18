// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

unsafe extern "C" {
    /// Initialize the GGL SDK
    /// Unused portions of sdk may not be initialized.
    /// Exits on error.
    pub fn ggl_sdk_init();
}
