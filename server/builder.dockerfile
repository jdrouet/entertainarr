ARG DEB_REVISION=1

FROM --platform=$BUILDPLATFORM rust:1-bookworm AS vendor

ENV USER=root

WORKDIR /code

RUN cargo init --bin --name entertainarr /code/server
RUN cargo init --bin --name entertainarr-client-cli /code/client/cli
RUN cargo init --lib --name entertainarr-client-core /code/client/core

COPY Cargo.lock /code/Cargo.lock
COPY Cargo.toml /code/Cargo.toml
COPY client/cli/Cargo.toml /code/client/cli/Cargo.toml
COPY client/core/Cargo.toml /code/client/core/Cargo.toml
COPY server/Cargo.toml /code/server/Cargo.toml

# https://docs.docker.com/engine/reference/builder/#run---mounttypecache
RUN --mount=type=cache,target=$CARGO_HOME/git,sharing=locked \
    --mount=type=cache,target=$CARGO_HOME/registry,sharing=locked \
    mkdir -p /code/.cargo \
    && cargo vendor >> /code/.cargo/config.toml

FROM rust:1-bookworm AS base

# https://docs.docker.com/engine/reference/builder/#run---mounttypecache
RUN --mount=type=cache,target=$CARGO_HOME/git,sharing=locked \
    --mount=type=cache,target=$CARGO_HOME/registry,sharing=locked \
    cargo install cargo-deb

ENV USER=root

WORKDIR /code

RUN cargo init --bin --name entertainarr-client-cli /code/client/cli
RUN cargo init --lib --name entertainarr-client-core /code/client/core

COPY --from=vendor /code/client /code/client
COPY --from=vendor /code/.cargo /code/.cargo
COPY --from=vendor /code/vendor /code/vendor
COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock
COPY server/Cargo.toml /code/server/Cargo.toml
COPY server/migrations /code/server/migrations
COPY server/package /code/server/package
COPY server/src /code/server/src

FROM base AS builder

WORKDIR /code/server

ARG DEB_REVISION
# https://docs.docker.com/engine/reference/builder/#run---mounttypecache
RUN --mount=type=cache,target=$CARGO_HOME/git,sharing=locked \
    --mount=type=cache,target=$CARGO_HOME/registry,sharing=locked \
    --mount=type=cache,target=/core/target/release/.fingerprint,sharing=locked \
    --mount=type=cache,target=/core/target/release/build,sharing=locked \
    --mount=type=cache,target=/core/target/release/deps,sharing=locked \
    --mount=type=cache,target=/core/target/release/examples,sharing=locked \
    --mount=type=cache,target=/core/target/release/incremental,sharing=locked \
    cargo deb --deb-revision ${DEB_REVISION}

FROM scratch

COPY --from=builder /code/target/debian /
