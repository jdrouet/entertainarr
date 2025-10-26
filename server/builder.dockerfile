ARG DEB_REVISION=1

FROM --platform=$BUILDPLATFORM rust:1-bookworm AS vendor

ENV USER=root

WORKDIR /code

RUN cargo init --bin --vcs none --name entertainarr /code/server
RUN cargo init --lib --vcs none --name entertainarr-adapter-http /code/adapter/http
RUN cargo init --lib --vcs none --name entertainarr-adapter-jsonwebtoken /code/adapter/jsonwebtoken
RUN cargo init --lib --vcs none --name entertainarr-adapter-rss /code/adapter/rss
RUN cargo init --lib --vcs none --name entertainarr-adapter-sqlite /code/adapter/sqlite
RUN cargo init --lib --vcs none --name entertainarr-client-core /code/client/core
RUN cargo init --lib --vcs none --name entertainarr-client-core-types /code/client/core-types
RUN cargo init --bin --vcs none --name entertainarr-client-web-leptos /code/client/web-leptos
RUN cargo init --lib --vcs none --name entertainarr-domain /code/domain

COPY Cargo.lock /code/Cargo.lock
COPY Cargo.toml /code/Cargo.toml
COPY adapter/http/Cargo.toml /code/adapter/http/Cargo.toml
COPY adapter/jsonwebtoken/Cargo.toml /code/adapter/jsonwebtoken/Cargo.toml
COPY adapter/rss/Cargo.toml /code/adapter/rss/Cargo.toml
COPY adapter/sqlite/Cargo.toml /code/adapter/sqlite/Cargo.toml
COPY client/core/Cargo.toml /code/client/core/Cargo.toml
COPY client/core-types/Cargo.toml /code/client/core-types/Cargo.toml
COPY client/web-leptos/Cargo.toml /code/client/web-leptos/Cargo.toml
COPY domain/Cargo.toml /code/domain/Cargo.toml
COPY server/Cargo.toml /code/server/Cargo.toml

# https://docs.docker.com/engine/reference/builder/#run---mounttypecache
RUN --mount=type=cache,target=$CARGO_HOME/git,sharing=locked \
    --mount=type=cache,target=$CARGO_HOME/registry,sharing=locked \
    mkdir -p /code/.cargo \
    && cargo vendor >> /code/.cargo/config.toml

########### CLIENT

FROM --platform=$BUILDPLATFORM rust:1-bookworm AS client-base

RUN rustup target add wasm32-unknown-unknown

# https://docs.docker.com/engine/reference/builder/#run---mounttypecache
RUN --mount=type=cache,target=$CARGO_HOME/git,sharing=locked \
    --mount=type=cache,target=$CARGO_HOME/registry,sharing=locked \
    cargo install --locked trunk \
    && cargo install --locked stylance-cli

ENV USER=root

WORKDIR /code

COPY --from=vendor /code/client /code/client
COPY --from=vendor /code/.cargo /code/.cargo
COPY --from=vendor /code/vendor /code/vendor
COPY --from=vendor /code/adapter /code/adapter
COPY --from=vendor /code/server /code/server

COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock

COPY adapter/http/Cargo.toml /code/adapter/http/Cargo.toml
COPY adapter/http/src /code/adapter/http/src

COPY client/core/Cargo.toml /code/client/core/Cargo.toml
COPY client/core/src /code/client/core/src

COPY client/core-types/Cargo.toml /code/client/core-types/Cargo.toml
COPY client/core-types/src /code/client/core-types/src

COPY client/web-leptos/Cargo.toml /code/client/web-leptos/Cargo.toml
COPY client/web-leptos/src /code/client/web-leptos/src
COPY client/web-leptos/index.html /code/client/web-leptos/index.html
COPY client/web-leptos/style.css /code/client/web-leptos/style.css
COPY client/web-leptos/Trunk.toml /code/client/web-leptos/Trunk.toml

COPY domain/Cargo.toml /code/domain/Cargo.toml
COPY domain/src /code/domain/src

FROM --platform=$BUILDPLATFORM client-base AS client-builder

WORKDIR /code/client/web-leptos

# produces in /code/client/web-leptos/dist
RUN trunk build --release

########### SERVER

FROM rust:1-bookworm AS server-base

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

FROM server-base AS builder

COPY --from=client-builder /code/client/web-leptos/dist /code/server/assets

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
