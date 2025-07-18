// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef GGL_CBMC_H
#define GGL_CBMC_H

//! Macros for CBMC function contracts

#ifndef __CPROVER__
#define CBMC_CONTRACT(...)
#else

#include <ggl/macro_util.h>

#define CBMC_CONTRACT(...) GGL_MACRO_FOREACH(CBMC_CONTRACT__STMT, , __VA_ARGS__)
#define CBMC_CONTRACT__STMT(arg) CBMC_CONTRACT__##arg

#define CBMC_CONTRACT__requires __CPROVER_requires
#define CBMC_CONTRACT__ensures __CPROVER_ensures
#define CBMC_CONTRACT__assigns __CPROVER_assigns
#define CBMC_CONTRACT__invariant __CPROVER_loop_invariant
#define CBMC_CONTRACT__decreases __CPROVER_decreases

#define cbmc_ptr_unique(...) \
    (GGL_MACRO_FOREACH(cbmc_ptr_unique__, &&, __VA_ARGS__))
#define cbmc_ptr_unique__(ptr) __CPROVER_is_fresh(ptr, sizeof(*ptr))
#define cbmc_arr_unique(ptr, n) __CPROVER_is_fresh(ptr, sizeof(*ptr) * n)

#define cbmc_enum_valid(e) __CPROVER_enum_is_in_range(e)

#define cbmc_old(...) __CPROVER_old(__VA_ARGS__)
#define cbmc_return __CPROVER_return_value

#endif

#endif
