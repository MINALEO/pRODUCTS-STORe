FROM rust:slim-bullseye as builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/spotbot
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim

RUN ap