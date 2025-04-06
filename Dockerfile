# Builder stage
FROM rust:slim AS builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
RUN rm -f Cargo.lock
RUN cargo build --release

# Runtime stage
FROM rust:slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/poem_dev_take_home poem_dev_take_home
COPY .env .env

ENTRYPOINT ["./poem_dev_take_home"]
