# ─── Build stage ────────────────────────────────────────
FROM rust:1.85-alpine AS builder

RUN apk add --no-cache musl-dev sqlite-dev pkgconfig

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src/ src/

RUN cargo build --release --locked

# ─── Runtime stage ──────────────────────────────────────
FROM alpine:3.21

RUN apk add --no-cache ca-certificates sqlite-libs

COPY --from=builder /app/target/release/pag-cms /usr/local/bin/pag-cms

RUN mkdir -p /opt/pagan-cms /var/pagan-os/build-space /etc/pag

ENV PAG_CMS_CONFIG=/etc/pag/cms.toml

EXPOSE 3005

ENTRYPOINT ["pag-cms"]
