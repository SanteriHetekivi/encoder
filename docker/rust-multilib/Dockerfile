FROM rust:1.54.0
LABEL version="1.0"
LABEL description="Rust builder that supports 32 and 64 bits."
LABEL maintainer="santeri@hetekivi.com"

# Update packages.
RUN apt update -y

# Support for 32-bit.
RUN apt install -y gcc-multilib

# Target 32-bit Linux (kernel 2.6.32+, glibc 2.11+).
RUN rustup target add i686-unknown-linux-gnu

# Target 64-bit Linux (kernel 2.6.32+, glibc 2.11+).
RUN rustup target add x86_64-unknown-linux-gnu &&