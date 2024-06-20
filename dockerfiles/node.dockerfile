FROM rust:latest AS chef
WORKDIR app
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG CRATE
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release -p $CRATE

FROM debian:bookworm-slim AS runtime
ARG CRATE
WORKDIR app

RUN apt-get update && apt install -y openssl
COPY --from=builder /app/target/release/$CRATE /usr/local/bin
