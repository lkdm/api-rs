FROM rust:1.78.0 AS builder
COPY . .
RUN cargo build --release
RUN cargo install sqlx-cli
RUN sqlx migrate run

FROM debian:bookworm-slim
COPY --from=builder ./target/release/api ./target/release/api
ENV DATABASE_URL=sqlite://sqlite.db
CMD ["/target/release/api"]
