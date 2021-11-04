#Download base image ubuntu 20.04
FROM ubuntu:20.04 as builder

# LABEL about the custom image
LABEL maintainer="hello@ajuna.io"
LABEL version="0.1"
LABEL description="This is custom Docker Image for the ajuna network."

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
    cp target/release /node

# ===== SECOND STAGE ======

FROM ubuntu:20.04

COPY --from=builder /node /usr/local/bin

# install curl in the event we want to interact with the local rpc
RUN apt-get update && apt-get install -y curl
RUN useradd --create-home runner

USER runner
EXPOSE 30333 9933 9944

ENTRYPOINT ["node"]