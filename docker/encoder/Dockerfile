FROM alpine:3.14.2
LABEL version="1.0"
LABEL description="Image with HandBrake and rust encoder."
LABEL maintainer="santeri@hetekivi.com"

# Update packages.
RUN apk update

# Install needed packages.
RUN apk add --virtual build-deps \
    # Git for cloning HandBrake repo.
    git \
    # Packages needed to build HandBrake.
    curl \
    autoconf \
    automake \
    g++ \
    cmake \
    git \
    libass-dev \
    bzip2-dev \
    fontconfig-dev \
    freetype-dev \
    fribidi-dev \
    harfbuzz-dev \
    jansson-dev \
    xz-libs \
    lame-dev \
    numactl-dev \
    libogg-dev \
    opus-dev \
    libsamplerate-dev \
    speex-dev \
    libtheora-dev \
    libtool \
    libtool \
    libjpeg-turbo-dev \
    libvorbis-dev \
    x264-dev \
    libxml2-dev \
    libvpx-dev \
    m4 \
    make \
    meson \
    nasm \
    ninja \
    patch \
    pkgconf \
    python2 \
    tar \
    zlib-dev \
    # Intel Quick Sync Video support.
    libva-dev \
    libdrm-dev \
    # Wget for downloading encoder.
    wget

# Build HandBrake.
RUN git clone https://github.com/HandBrake/HandBrake.git && \
    cd HandBrake && \
    ./configure --launch-jobs=$(nproc) --launch --disable-gtk && \
    make --directory=build install && \
    rm -rf build

# Make input directory.
RUN mkdir -p /app/input
# Make output directory.
RUN mkdir -p /app/output

# Set workdir to /app.
WORKDIR /app

# Get encoder.
RUN wget \
    -O /app/encoder \
    https://github.com/SanteriHetekivi/encoder/releases/download/1.0.1/encoder_x86_64-unknown-linux-musl
# Add execution permission to program.
RUN chmod +x /app/encoder
# Add program to bin.
RUN ln -s /app/encoder /usr/local/bin

# Remove build dependences.
RUN apk del build-deps

# Needed shared liraries for HandBrake.
RUN apk add \
    libass \
    lame \
    libvpx \
    libtheora \
    libvorbis \
    x264-dev \
    libxml2 \
    bzip2 \
    jansson \
    opus \
    speex \
    xz-libs \
    libjpeg-turbo \
    numactl \
    libstdc++

# Add entrypoint.
COPY entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

# Run encoder.
ENTRYPOINT ["/app/entrypoint.sh"]