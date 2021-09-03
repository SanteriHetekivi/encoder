#!/bin/bash

# Get directory of this file.
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
# Move to this file's directory.
cd $SCRIPT_DIR &&
# Make release directory.
mkdir -p release &&
# Remove files from release directory.
rm -rf release/* &&


# Build 64-bit macOS (10.7+, Lion+).
rustup target add x86_64-apple-darwin &&
cargo build \
    --release \
    --target=x86_64-apple-darwin &&
cp \
    target/x86_64-apple-darwin/release/encoder \
    release/encoder_x86_64-apple-darwin &&

# Build 32-bit, 64-bit GNU and 64-bit MUSL for Linux.
docker run \
    --rm \
    --user "$(id -u)":"$(id -g)" \
    -v "$PWD":/usr/src/myapp \
    -w /usr/src/myapp \
    -t \
    ghcr.io/santerihetekivi/rust-multilib:master \
    ./build-linux-gnu.sh &&
cp \
    target/i686-unknown-linux-gnu/release/encoder \
    release/encoder_i686-unknown-linux-gnu &&
cp \
    target/x86_64-unknown-linux-gnu/release/encoder \
    release/encoder_x86_64-unknown-linux-gnu &&
cp \
    target/x86_64-unknown-linux-musl/release/encoder \
    release/encoder_x86_64-unknown-linux-musl &&

# Build ARM64 Linux (kernel 4.2, glibc 2.17+).
#cargo build --release --target=aarch64-unknown-linux-gnu &&
#cp target/release/encoder release/encoder_aarch64-unknown-linux-gnu &&

# Build 32-bit MinGW (Windows 7+).
#rustup target add i686-pc-windows-gnu &&
#cargo build --release --target=i686-pc-windows-gnu &&
#cp target/release/encoder release/encoder_i686-pc-windows-gnu &&

# Build 32-bit MSVC (Windows 7+).
#rustup target add i686-pc-windows-msvc &&
#cargo build --release --target=i686-pc-windows-msvc &&
#cp target/release/encoder release/encoder_i686-pc-windows-msvc &&

# Build 64-bit MinGW (Windows 7+).
#rustup target add x86_64-pc-windows-gnu &&
#cargo build --release --target=x86_64-pc-windows-gnu &&
#cp target/release/encoder release/encoder_x86_64-pc-windows-gnu &&

# Build 64-bit MSVC (Windows 7+).
#rustup target add x86_64-pc-windows-msvc &&
#cargo build --release --target=x86_64-pc-windows-msvc &&
#cp target/release/encoder release/encoder_x86_64-pc-windows-msvc &&

echo "Done"