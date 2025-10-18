ARG DEB_REVISION=1

FROM --platform=$BUILDPLATFORM rust:1-bookworm AS vendor

ENV USER=root

WORKDIR /code

RUN cargo init --bin --name entertainarr /code/server
RUN cargo init --lib --name entertainarr-adapter-http /code/adapter/http
RUN cargo init --lib --name entertainarr-adapter-jsonwebtoken /code/adapter/jsonwebtoken
RUN cargo init --lib --name entertainarr-adapter-rss /code/adapter/rss
RUN cargo init --lib --name entertainarr-adapter-sqlite /code/adapter/sqlite
RUN cargo init --bin --name entertainarr-client-cli /code/client/cli
RUN cargo init --lib --name entertainarr-client-core /code/client/core
RUN cargo init --lib --name entertainarr-domain /code/domain

COPY Cargo.lock /code/Cargo.lock
COPY Cargo.toml /code/Cargo.toml
COPY adapter/http/Cargo.toml /code/adapter/http/Cargo.toml
COPY adapter/jsonwebtoken/Cargo.toml /code/adapter/jsonwebtoken/Cargo.toml
COPY adapter/rss/Cargo.toml /code/adapter/rss/Cargo.toml
COPY adapter/sqlite/Cargo.toml /code/adapter/sqlite/Cargo.toml
COPY client/cli/Cargo.toml /code/client/cli/Cargo.toml
COPY client/core/Cargo.toml /code/client/core/Cargo.toml
COPY domain/Cargo.toml /code/domain/Cargo.toml
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

COPY adapter/http/Cargo.toml /code/adapter/http/Cargo.toml
COPY adapter/http/src /code/adapter/http/src

COPY adapter/jsonwebtoken/Cargo.toml /code/adapter/jsonwebtoken/Cargo.toml
COPY adapter/jsonwebtoken/src /code/adapter/jsonwebtoken/src

COPY adapter/rss/Cargo.toml /code/adapter/rss/Cargo.toml
COPY adapter/rss/src /code/adapter/rss/src

COPY adapter/sqlite/Cargo.toml /code/adapter/sqlite/Cargo.toml
COPY adapter/sqlite/migrations /code/adapter/sqlite/migrations
COPY adapter/sqlite/src /code/adapter/sqlite/src

COPY domain/Cargo.toml /code/domain/Cargo.toml
COPY domain/src /code/domain/src

COPY server/Cargo.toml /code/server/Cargo.toml
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
