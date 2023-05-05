# Build Stage
FROM --platform=linux/amd64 rustlang/rust:nightly as builder

ENV DEBIAN_FRONTEND=noninteractive
## Install build dependencies.
RUN apt-get update 
RUN apt-get install -y cmake clang pkg-config libfreetype6-dev libfontconfig1-dev
RUN cargo install cargo-fuzz

## Add source code to the build stage.
ADD . /plotters/

WORKDIR /plotters/fuzz/

RUN cargo +nightly fuzz build

FROM --platform=linux/amd64 rustlang/rust:nightly

## TODO: Change <Path in Builder Stage>

COPY --from=builder /plotters/fuzz/target/x86_64-unknown-linux-gnu/release/example_chart /