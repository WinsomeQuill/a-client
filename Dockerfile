FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

FROM chef AS planner
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY .env .env
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY .env .env
RUN cargo build --release
RUN mv ./target/release/a-client ./a-client

FROM debian:stable-slim AS runtime

WORKDIR /app

RUN apt-get update && apt-get upgrade -y && apt-get install -y curl
RUN apt-get install build-essential perl pkg-config libssl-dev -y

COPY --from=builder /app/a-client /app
COPY --from=builder /app/.env /app

ENTRYPOINT ["/app/a-client"]
