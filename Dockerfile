FROM rust:slim-bullseye as builder

RUN apt-get update \
    && apt-get i