FROM rust:1.78.0 AS builder
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder ./target/release/api ./target/release/api
CMD ["/target/release/api"]
EXPOSE 8080
