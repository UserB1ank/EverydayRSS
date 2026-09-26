# syntax=docker/dockerfile:1

FROM rust:1.88-bookworm AS builder

WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked \
    && cp target/release/everydayrss /tmp/everydayrss

FROM debian:bookworm-slim AS runtime

LABEL org.opencontainers.image.source="https://github.com/UserB1ank/EverydayRSS" \
      org.opencontainers.image.description="AI-powered RSS daily report generator" \
      org.opencontainers.image.licenses="MIT"

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install --yes --no-install-recommends \
        ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system --gid 10001 everydayrss \
    && useradd --system --uid 10001 --gid everydayrss \
        --home-dir /home/everydayrss --create-home everydayrss \
    && mkdir -p /data \
    && chown everydayrss:everydayrss /data

COPY --from=builder /tmp/everydayrss /usr/local/bin/everydayrss

ENV EVERYDAYRSS_CONFIG=/data/config.toml \
    EVERYDAYRSS_SCHEDULER=internal \
    TZ=Etc/UTC
WORKDIR /data
VOLUME ["/data"]

USER everydayrss:everydayrss
ENTRYPOINT ["everydayrss"]
CMD ["daemon"]
