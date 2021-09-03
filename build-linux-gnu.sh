#!/bin/bash

# Get directory of this file.
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
# Move to this file's directory.
cd $SCRIPT_DIR &&

# Build 32-bit Linux (kernel 2.6.32+, glibc 2.11+).
cargo build \
    --release \
    --target=i686-unknown-linux-gnu &&

# Build 64-bit Linux (kernel 2.6.32+, glibc 2.11+).
cargo build \
    --release \
    --target=x86_64-unknown-linux-gnu

# 64-bit Linux with MUSL
cargo build \
    --release \
    --target=x86_64-unknown-linux-musl