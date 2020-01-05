FROM rust

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y python3-dev graphviz python3-pip && \
    pip3 install graphviz maturin && \
    rustup toolchain install nightly-2019-11-16-x86_64-unknown-linux-gnu && \
    rustup default nightly-2019-11-16-x86_64-unknown-linux-gnu

ENV RUST_BACKTRACE full

VOLUME /code
WORKDIR /code

ENTRYPOINT ["cargo"]