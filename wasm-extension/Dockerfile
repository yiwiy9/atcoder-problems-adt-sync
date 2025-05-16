FROM rust:1.86-bookworm

RUN apt-get update && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    \
    rustup component add rustfmt clippy && \
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
