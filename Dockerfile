# -----------------
# Cargo Build Stage
# -----------------

FROM rust:latest AS cargo-build

COPY . .
RUN apt-get update && apt-get install -y \
  cmake \
  clang \
  protobuf-compiler \
  && rm -rf /var/lib/apt/lists/*
RUN cargo vendor > .cargo/config

RUN make build-release

# -----------------
# Run Momento Proxy
# -----------------

FROM debian:stable-slim

WORKDIR /app

RUN mkdir config

COPY --from=cargo-build ./target/release/momento_proxy .
COPY --from=cargo-build ./config/momento_proxy.toml ./config

RUN chmod +x ./momento_proxy
CMD ["./momento_proxy", "./config/momento_proxy.toml"]
