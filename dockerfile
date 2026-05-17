FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY ./src ./src

RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/nyx /usr/local/bin/nyx

ENTRYPOINT [ "nyx" ]