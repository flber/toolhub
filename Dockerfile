# syntax=docker/dockerfile:1.3-labs
FROM rust:1.67 as builder

WORKDIR /opt/toolhub

RUN apt-get update && apt-get install -y \
	libsqlite3-dev \
	graphicsmagick \
	libgraphicsmagick1-dev \
	llvm-dev \
	libclang-dev \
	clang

COPY ["Cargo.toml", "Cargo.lock",  "./"]
# Make empty fake project, just pulls dependencies
RUN mkdir src && mkdir handlebar && echo "fn main() {}" > src/main.rs
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

# Clobber fake project with real project source
COPY src src
COPY handlebar handlebar
RUN --mount=type=cache,target=/usr/local/cargo/registry \
  set -e && \
  touch src/main.rs && \
  cargo install --path .


FROM debian:bullseye-slim

WORKDIR /opt/toolhub

ENV TZ="America/Los_Angeles"

RUN apt-get update && apt-get install -y \
	libsqlite3-dev \
	graphicsmagick \
	libgraphicsmagick1-dev \
	llvm-dev \
	libclang-dev \
	curl \
	build-essential \
	clang \
	pkg-config \
	libjpeg-turbo-progs \
	libpng-dev
RUN rm -rfv /var/lib/apt/lists/*

ENV MAGICK_VERSION 7.1

RUN curl https://imagemagick.org/archive/ImageMagick.tar.gz | tar xz \
	&& cd ImageMagick-${MAGICK_VERSION}* \
	&& ./configure --with-magick-plus-plus=no --with-perl=no \
	&& make \
	&& make install \
	&& cd .. \
	&& rm -r ImageMagick-${MAGICK_VERSION}*

WORKDIR /magick
COPY build.rs .
COPY Cargo.toml .
COPY src src
COPY tests tests

RUN adduser --disabled-password --gecos '' magick-rust
RUN chown -R magick-rust .

USER magick-rust

ENV USER=magick-rust
ENV LD_LIBRARY_PATH=/usr/local/lib


WORKDIR /opt/toolhub

COPY --from=builder /usr/local/cargo/bin/toolhub /usr/local/bin/toolhub

ADD public public
ADD handlebar handlebar
ADD static static
ADD Rocket.toml .

CMD ["toolhub"]
