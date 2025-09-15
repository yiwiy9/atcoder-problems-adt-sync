FROM rust:1.87-bookworm

RUN apt-get update && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    \
    rustup component add rustfmt clippy
