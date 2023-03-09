VERSION 0.7
PROJECT AlexMikhalev-1061/atomic-data-rust

test-pipeline:
    PIPELINE --push
    TRIGGER push develop
    TRIGGER pr develop
    BUILD +docker
    
build:
    FROM rust:latest
    RUN rustup target add x86_64-unknown-linux-musl
    RUN apt update && apt install -y musl-tools musl-dev
    RUN update-ca-certificates
    WORKDIR /app
    COPY --dir server lib cli desktop Cargo.lock Cargo.toml .
    RUN cargo build --release --bin atomic-server --config net.git-fetch-with-cli=true --target x86_64-unknown-linux-musl
    RUN strip -s /app/target/x86_64-unknown-linux-musl/release/atomic-server
    SAVE ARTIFACT /app/target/x86_64-unknown-linux-musl/release/atomic-server

docker:
    # We only need a small runtime for this step, but make sure glibc is installed
    FROM scratch
    COPY --chmod=0755 +build/atomic-server /atomic-server-bin
    # For a complete list of possible ENV vars or available flags, run with `--help`
    ENV ATOMIC_STORE_PATH="/atomic-storage/db"
    ENV ATOMIC_CONFIG_PATH="/atomic-storage/config.toml"
    ENV ATOMIC_PORT="80"
    EXPOSE 80
    VOLUME /atomic-storage
    ENTRYPOINT ["/atomic-server-bin"]
    SAVE IMAGE --push ghcr.io/applied-knowledge-systems/atomic-server:edge
