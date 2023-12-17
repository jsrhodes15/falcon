FROM lukemathwalker/cargo-chef:latest-rust-1.74 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
# compute a lock-like file for project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# build project depenedncies, not application
RUN cargo chef cook --release --recipe-path recipe.json
# up to this point, if our dependency tree stays the same, all layers should be cached
COPY . .
ENV SQLX_OFFLINE true
# build app
RUN cargo build --release --bin falcon_rust

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/falcon_rust falcon_rust
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./target/release/falcon_rust"]