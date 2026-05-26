# Stage 1: Chef - Prepare dependencies
FROM rust:1.75-bookworm as chef
RUN cargo install cargo-chef
WORKDIR /app

# Stage 2: Planner - Create recipe
FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Builder - Build dependencies and application
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies (cached layer)
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# Stage 4: Runtime - Minimal image
FROM debian:bookworm-slim as runtime
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/zyblog-backend /app/zyblog-backend
EXPOSE 8080
CMD ["./zyblog-backend"]
