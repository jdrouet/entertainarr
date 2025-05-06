# trunk base image
FROM --platform=$BUILDPLATFORM rust:1-alpine AS trunk

RUN apk add --no-cache musl-dev

RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked trunk --features rustls --no-default-features

# fetch the vendor with the builder platform to avoid qemu issues
FROM --platform=$BUILDPLATFORM rust:1-alpine AS vendor

ENV USER=root

WORKDIR /code
RUN cargo init
RUN cargo init packages/api
RUN cargo init packages/database
RUN cargo init packages/server
RUN cargo init packages/storage
RUN cargo init packages/web

COPY Cargo.toml Cargo.lock /code/
COPY packages/api/Cargo.toml /code/packages/api/Cargo.toml
COPY packages/database/Cargo.toml /code/packages/database/Cargo.toml
COPY packages/server/Cargo.toml /code/packages/server/Cargo.toml
COPY packages/storage/Cargo.toml /code/packages/storage/Cargo.toml
COPY packages/web/Cargo.toml /code/packages/web/Cargo.toml

# https://docs.docker.com/engine/reference/builder/#run---mounttypecache
RUN --mount=type=cache,target=$CARGO_HOME/git,sharing=locked \
  --mount=type=cache,target=$CARGO_HOME/registry,sharing=locked \
  mkdir -p /code/.cargo \
  && cargo vendor >> /code/.cargo/config.toml

# build web
FROM --platform=$BUILDPLATFORM trunk AS client

COPY Cargo.toml Cargo.lock /code/
COPY assets /code/assets
COPY packages /code/packages
COPY --from=vendor /code/.cargo /code/.cargo
COPY --from=vendor /code/vendor /code/vendor

WORKDIR /code/packages/web

RUN trunk build --release

# build server
FROM rust:1-alpine 

RUN apk add --no-cache musl-dev

ENV USER=root

WORKDIR /code

COPY Cargo.toml Cargo.lock /code/
COPY assets /code/assets
COPY packages /code/packages
COPY --from=vendor /code/.cargo /code/.cargo
COPY --from=vendor /code/vendor /code/vendor
COPY --from=client /code/packages/web/dist/* packages/server/public/

RUN cargo build --release --package entertainarr-server

