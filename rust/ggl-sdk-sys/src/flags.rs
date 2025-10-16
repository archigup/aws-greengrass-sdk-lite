// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum GglPresence {
    GGL_PRESENCE_REQUIRED = 0,
    GGL_PRESENCE_OPTIONAL = 1,
    GGL_PRESENCE_MISSING = 2,
}

pub const GGL_REQUIRED: GglPresence = GglPresence::GGL_PRESENCE_REQUIRED;
pub const GGL_OPTIONAL: GglPresence = GglPresence::GGL_PRESENCE_OPTIONAL;
pub const GGL_MISSING: GglPresence = GglPresence::GGL_PRESENCE_MISSING;
