#
# Ajuna Network
# Substrate testing image
#
# Building
# docker build -t substrate:ajuna .
#
# Running
# docker run --rm -d  -p 30333:30333/tcp -p 9933:9933/tcp -p 9944:9944/tcp substrate:ajuna

FROM ubuntu:20.04 as builder

# LABEL about the custom image
LABEL maintainer="hello@ajuna.io"
LABEL version="0.1"
LABEL description="This image is for development purposes. Follow the instruction in Dockerfile to build and start this image."

# Disable Prompt During Packages Installation
ARG DEBIAN_FRONTEND=noninteractive
ARG GIT_REPO=https://github.com/ajuna-network/substrate.git
ARG GIT_BRANCH=ajuna-node

# Update Ubuntu Software repository
RUN apt update  &&  \
    apt-get install -y \
    clang \
    cmake \
    curl \
    git \
    libssl-dev \
    pkg-config

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN git clone ${GIT_REPO} substrate && \
    cd substrate && \
    git checkout ${GIT_BRANCH} && \
    git submodule update --init --recursive

RUN rustup default stable && \
    rustup update && \
    rustup update nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly

RUN cd substrate && \
    cargo build --release && \
    cp -r target/release /node

#
# Ajuna.Network Runner
#
# Description
# This is the runtime environment. Copy from build environment and kick-off the node.
#
FROM ubuntu:20.04

COPY --from=builder /node /usr/local/bin

RUN apt-get update && apt-get install -y curl
RUN useradd --create-home runner

USER runner
EXPOSE 30333 9933 9944

CMD [ "substrate", "--dev", "--unsafe-ws-external", "--unsafe-rpc-external", "--prometheus-external", "--rpc-cors=all", "--rpc-methods=Unsafe" ]
