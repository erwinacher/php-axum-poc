FROM rust:1.90 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/rust-server /usr/local/bin/rust-server

CMD ["rust-server"]
