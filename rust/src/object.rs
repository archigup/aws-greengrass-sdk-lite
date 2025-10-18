// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::c;
use std::marker::PhantomData;

#[repr(transparent)]
pub struct Object<'a> {
    inner: c::GglObject,
    _marker: PhantomData<(&'a [u8], &'a [c::GglKV], &'a [c::GglObject])>,
}

impl<'a> Object<'a> {
    pub fn null() -> Self {
        Self {
            inner: c::GGL_OBJ_NULL,
            _marker: PhantomData,
        }
    }

    pub fn bool(value: bool) -> Self {
        Self {
            inner: unsafe { c::ggl_obj_bool(value) },
            _marker: PhantomData,
        }
    }

    pub fn i64(value: i64) -> Self {
        Self {
            inner: unsafe { c::ggl_obj_i64(value) },
            _marker: PhantomData,
        }
    }

    pub fn f64(value: f64) -> Self {
        Self {
            inner: unsafe { c::ggl_obj_f64(value) },
            _marker: PhantomData,
        }
    }

    pub fn buf(value: &'a str) -> Self {
        let buf = c::GglBuffer {
            data: value.as_ptr() as *mut u8,
            len: value.len(),
        };
        Self {
            inner: unsafe { c::ggl_obj_buf(buf) },
            _marker: PhantomData,
        }
    }

    pub fn map(value: Map<'a>) -> Self {
        Self {
            inner: unsafe { c::ggl_obj_map(value.inner) },
            _marker: PhantomData,
        }
    }

    pub fn list(value: List<'a>) -> Self {
        Self {
            inner: unsafe { c::ggl_obj_list(value.inner) },
            _marker: PhantomData,
        }
    }

    pub(crate) fn as_raw(&self) -> c::GglObject {
        self.inner
    }
}

#[repr(transparent)]
pub struct Map<'a> {
    inner: c::GglMap,
    _marker: PhantomData<&'a mut [c::GglKV]>,
}

impl<'a> Map<'a> {
    pub fn new(pairs: &'a mut [KV<'a>]) -> Self {
        Self {
            inner: c::GglMap {
                pairs: pairs.as_mut_ptr() as *mut c::GglKV,
                len: pairs.len(),
            },
            _marker: PhantomData,
        }
    }

    pub(crate) fn as_raw(&self) -> c::GglMap {
        self.inner
    }
}

#[repr(transparent)]
pub struct KV<'a> {
    inner: c::GglKV,
    _marker: PhantomData<(&'a str, Object<'a>)>,
}

impl<'a> KV<'a> {
    pub fn new(key: &'a str, val: Object<'a>) -> Self {
        let key_buf = c::GglBuffer {
            data: key.as_ptr() as *mut u8,
            len: key.len(),
        };
        Self {
            inner: unsafe { c::ggl_kv(key_buf, val.as_raw()) },
            _marker: PhantomData,
        }
    }
}

#[repr(transparent)]
pub struct List<'a> {
    inner: c::GglList,
    _marker: PhantomData<&'a mut [c::GglObject]>,
}

impl<'a> List<'a> {
    pub fn new(items: &'a mut [Object<'a>]) -> Self {
        Self {
            inner: c::GglList {
                items: items.as_mut_ptr() as *mut c::GglObject,
                len: items.len(),
            },
            _marker: PhantomData,
        }
    }
}
