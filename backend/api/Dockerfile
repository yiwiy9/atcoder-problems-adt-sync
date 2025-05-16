FROM ghcr.io/cargo-lambda/cargo-lambda:1.8.4

RUN apt-get update && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    \
    rustup component add rustfmt clippy
