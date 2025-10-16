// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_root =
        PathBuf::from(&manifest_dir).parent().unwrap().parent().unwrap().to_path_buf();

    let cmake_build_dir = PathBuf::from(&out_dir).join("cmake");

    if !cmake_build_dir.exists() {
        std::fs::create_dir_all(&cmake_build_dir).unwrap();

        let profile = env::var("PROFILE").unwrap();
        let opt_level = env::var("OPT_LEVEL").unwrap_or_default();
        let debug = env::var("DEBUG").unwrap_or_default() == "true";

        let cmake_build_type = match profile.as_str() {
            "debug" => "Debug",
            "release" => {
                if opt_level == "s" || opt_level == "z" {
                    "MinSizeRel"
                } else if debug {
                    "RelWithDebInfo"
                } else {
                    "Release"
                }
            }
            _ => "MinSizeRel",
        };

        let output = Command::new("cmake")
            .arg(&project_root)
            .arg(format!("-DCMAKE_BUILD_TYPE={}", cmake_build_type))
            .current_dir(&cmake_build_dir)
            .output()
            .expect("Failed to run cmake configure");

        if !output.status.success() {
            panic!(
                "CMake configure failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    let output = Command::new("cmake")
        .args(["--build", ".", "--target", "ggl-sdk"])
        .current_dir(&cmake_build_dir)
        .output()
        .expect("Failed to run cmake build");

    if !output.status.success() {
        panic!(
            "CMake build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Link to the static library
    println!(
        "cargo:rustc-link-search=native={}",
        cmake_build_dir.display()
    );
    println!("cargo:rustc-link-lib=static=ggl-sdk");

    // Rerun if CMakeLists.txt or source files change
    println!("cargo:rerun-if-changed=../../CMakeLists.txt");
    println!("cargo:rerun-if-changed=../../src");
    println!("cargo:rerun-if-changed=../../include");
    println!("cargo:rerun-if-changed=../../priv_include");
}
