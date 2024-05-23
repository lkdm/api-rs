ARG RUST_VERSION=1.75.0

FROM rust:${RUST_VERSION}-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  cargo build --release && \
  cp ./target/release/api /


FROM debian:bookworm-slim AS final
COPY --from=builder /api /usr/local/bin
COPY --from=builder /app/config /opt/api/config
WORKDIR /opt/api
ENTRYPOINT ["api"]
EXPOSE 8080
