// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::{GglError, GglList, GglObjectType};

unsafe extern "C" {
    pub fn ggl_list_type_check(list: GglList, type_: GglObjectType) -> GglError;
}
