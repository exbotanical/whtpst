# Use chef so we can build deps separately and optimize Docker's caching layers
FROM lukemathwalker/cargo-chef:latest-rust-1.71.1 AS chef

WORKDIR /app
RUN apt update && apt install clang -y

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin whtpst

FROM debian:bullseye-slim AS runtime

WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/whtpst whtpst
COPY config config
ENV APP_ENVIRONMENT production

ENTRYPOINT ["./whtpst"]
