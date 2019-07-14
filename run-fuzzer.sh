#!/bin/bash

set -e
set -x

rustup override set nightly

# Specify RUSTFLAGS:
export RUSTFLAGS=""

# - so the target crate is compiled with sanitization
export RUSTFLAGS="$RUSTFLAGS -C passes=sancov -C llvm-args=-sanitizer-coverage-level=3"

# - optimizations
export RUSTFLAGS="$RUSTFLAGS -C opt-level=0"

# - and debug assertions
export RUSTFLAGS="$RUSTFLAGS -C debug-assertions=on"

# Create seed directory if it does not exist. Add example files here.
mkdir -p seeds

# Create corpus directory which the fuzzer will fill with interesting inputs.
mkdir -p corpus

# Create artifact output directory.
mkdir -p artifacts

# Detect the target.
if [ "$(uname -s)" == "Darwin" ]; then
    export TARGET="x86_64-apple-darwin"
elif [ "$(uname -s)" == "Linux" ]; then
    export TARGET="x86_64-unknown-linux-gnu"
else
    echo "Sorry, currently only Mac OS and Linux are supported"
    exit 1
fi

TOOLCHAIN_ROOT=${RUSTUP_BASE:-$HOME/.rustup}/toolchains/nightly-$TARGET

# Set some evironment variables that are needed when building the rustc source code.
export CFG_CODEGEN_BACKENDS_DIR=$TOOLCHAIN_ROOT/lib/rustlib/$TARGET/codegen-backends
export CFG_COMPILER_HOST_TRIPLE=$TARGET

# Any writable location will do for this one.
export RUSTC_ERROR_METADATA_DST=/tmp/rustc_error_metadata

# Custom environment variable.
export FUZZ_RUSTC_LIBRARY_DIR=$TOOLCHAIN_ROOT/lib/rustlib/$TARGET/lib

# The --target flag is imporant because it prevents build.rs scripts from being built with
# the above-specified RUSTFLAGS.
cargo run --verbose --target $TARGET --bin "fuzz_target" -- -artifact_prefix=artifacts/ ${@:1} `pwd`/corpus `pwd`/seeds
