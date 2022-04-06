FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin api

FROM debian:bullseye-slim AS runtime
COPY --from=builder /app/target/release/api /usr/local/bin
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["/usr/local/bin/api"]