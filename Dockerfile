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
	clang

COPY --from=builder /usr/local/cargo/bin/toolhub /usr/local/bin/toolhub

ADD public public
ADD handlebar handlebar
ADD Rocket.toml .

CMD ["toolhub"]
