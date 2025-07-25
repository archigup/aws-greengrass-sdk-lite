# aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

{
  description = "Lightweight AWS IoT Greengrass SDK.";
  inputs.flakelight.url = "github:nix-community/flakelight";
  outputs = { flakelight, ... }@inputs: flakelight ./.
    ({ lib, config, ... }:
      let
        filteredSrc = lib.fileset.toSource {
          root = ./.;
          fileset = lib.fileset.unions [
            ./CMakeLists.txt
            ./include
            ./priv_include
            ./src
            ./.clang-tidy
            ./samples
          ];
        };

        llvmStdenv = pkgs: pkgs.overrideCC pkgs.llvmPackages.stdenv
          (pkgs.llvmPackages.stdenv.cc.override
            { inherit (pkgs.llvmPackages) bintools; });
      in
      {
        systems = lib.systems.flakeExposed;
        inherit inputs;

        devShell = pkgs: {
          packages = with pkgs; [
            clang-tools
            clangd-tidy
            git
            cmake
            cbmc
          ];
          env.NIX_HARDENING_ENABLE = "";
          shellHook = ''
            export MAKEFLAGS=-j
          '';
        };

        devShells.clang = { lib, pkgs, ... }: {
          imports = [ (config.devShell pkgs) ];
          stdenv = lib.mkForce (llvmStdenv pkgs);
        };

        formatters = { llvmPackages, cmake-format, nodePackages, yapf, ... }:
          let
            fmt-c = "${llvmPackages.clang-unwrapped}/bin/clang-format -i";
            fmt-cmake = "${cmake-format}/bin/cmake-format -i";
            fmt-yaml =
              "${nodePackages.prettier}/bin/prettier --write --parser yaml";
          in
          {
            "*.c" = fmt-c;
            "*.h" = fmt-c;
            "CMakeLists.txt" = fmt-cmake;
            ".clang*" = fmt-yaml;
            "*.py" = "${yapf}/bin/yapf -i";
          };

        pname = "ggl-sdk";
        package = { stdenv, pkg-config, cmake, ninja, defaultMeta }:
          stdenv.mkDerivation {
            name = "ggl-sdk";
            src = filteredSrc;
            nativeBuildInputs = [ pkg-config cmake ninja ];
            cmakeBuildType = "MinSizeRel";
            cmakeFlags = [ "-DENABLE_WERROR=1" ];
            hardeningDisable = [ "all" ];
            dontStrip = true;
            meta = defaultMeta;
          };

        packages.ggl-sdk-static = { pkgsStatic }: pkgsStatic.ggl-sdk;

        checks =
          let
            clangBuildDir = { pkgs, pkg-config, clang-tools, cmake, ... }:
              (llvmStdenv pkgs).mkDerivation {
                name = "clang-cmake-build-dir";
                nativeBuildInputs = [ pkg-config clang-tools ];
                buildPhase = ''
                  ${cmake}/bin/cmake -B $out -S ${filteredSrc} \
                    -D CMAKE_BUILD_TYPE=Debug
                  rm $out/CMakeFiles/CMakeConfigureLog.yaml
                '';
                dontUnpack = true;
                dontPatch = true;
                dontConfigure = true;
                dontInstall = true;
                dontFixup = true;
                allowSubstitutes = false;
              };
          in
          {
            build-clang = pkgs: pkgs.ggl-sdk.override
              { stdenv = llvmStdenv pkgs; };

            build-musl-pi = pkgs: pkgs.pkgsCross.muslpi.ggl-sdk;

            clang-tidy = pkgs: ''
              set -eo pipefail
              cd ${filteredSrc}
              PATH=${lib.makeBinPath
                (with pkgs; [clangd-tidy clang-tools fd])}:$PATH
              clangd-tidy -j$(nproc) -p ${clangBuildDir pkgs} --color=always \
                $(fd . src -e c -e h)
            '';

            cbmc-contracts = { stdenv, pkg-config, cmake, cbmc, python3, ... }:
              stdenv.mkDerivation {
                name = "check-cbmc-contracts";
                src = filteredSrc;
                nativeBuildInputs = [ pkg-config cbmc python3 ];
                buildPhase = ''
                  ${cmake}/bin/cmake -B build -D CMAKE_BUILD_TYPE=Debug \
                    -D GGL_LOG_LEVEL=TRACE
                  python ${./misc/check_contracts.py} src
                  touch $out
                '';
                dontPatch = true;
                dontConfigure = true;
                dontInstall = true;
                dontFixup = true;
                allowSubstitutes = false;
              };

            iwyu = pkgs: ''
              set -eo pipefail
              PATH=${lib.makeBinPath
                (with pkgs; [include-what-you-use fd])}:$PATH
              white=$(printf "\e[1;37m")
              red=$(printf "\e[1;31m")
              clear=$(printf "\e[0m")
              iwyu_tool.py -o clang -j $(nproc) -p ${clangBuildDir pkgs} \
                $(fd . ${filteredSrc}/ -e c) -- \
                -Xiwyu --error -Xiwyu --check_also="${filteredSrc}/*" \
                -Xiwyu --mapping_file=${./.}/misc/iwyu_mappings.yml |\
                { grep error: || true; } |\
                sed 's|\(.*\)error:\(.*\)|'$white'\1'$red'error:'$white'\2'$clear'|' |\
                sed 's|${filteredSrc}/||'
            '';

            cmake-lint = pkgs: ''
              ${pkgs.cmake-format}/bin/cmake-lint \
                $(${pkgs.fd}/bin/fd '.*\.cmake|CMakeLists.txt') \
                --suppress-decorations
            '';

            spelling = pkgs: ''
              ${pkgs.nodePackages.cspell}/bin/cspell "**" --quiet
              ${pkgs.coreutils}/bin/sort -cuf misc/dictionary.txt
            '';
          };

        withOverlays = final: prev: {
          clangd-tidy = final.callPackage
            ({ python3Packages }:
              python3Packages.buildPythonPackage rec {
                pname = "clangd_tidy";
                version = "1.1.0.post2";
                format = "pyproject";
                src = final.fetchPypi {
                  inherit pname version;
                  hash = "sha256-NyghLY+BeY9LAOstKEFcPLdA7l1jCdHLuyPms4bOyYE=";
                };
                buildInputs = with python3Packages; [ setuptools-scm ];
                propagatedBuildInputs = with python3Packages; [
                  attrs
                  cattrs
                  typing-extensions
                ];
              })
            { };
          cbmc = prev.cbmc.overrideAttrs {
            version = "6.7.1";
            src = final.fetchFromGitHub {
              owner = "diffblue";
              repo = "cbmc";
              # Includes commits after 6.7.1 needed for harness generation
              # Use tag when 6.7.2 is released
              rev = "062962c1da7149be418338b1f2220d51960e06f8";
              hash = "sha256-qcCiKv+AUoiIZdiCK955Bl5GBK+JHv0mDflQ4aAj4IQ=";
            };
          };
        };
      });
}
