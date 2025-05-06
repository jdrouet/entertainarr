FROM docker.io/rust:alpine

RUN apk add --no-cache musl-dev

RUN cargo install --locked trunk --features rustls --no-default-features

