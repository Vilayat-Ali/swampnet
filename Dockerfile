# ==========================================
# Stage 1: Buildtime Stage
# ==========================================

FROM rust:alpine AS builder

WORKDIR /swampnet

RUN apk add --no-cache \
    clang \
    mold \
    build-base \
    musl-dev

COPY .cargo .cargo
COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY src src

RUN touch src/main.rs && cargo build --release

# ==========================================
# Stage 2: Runtime Stage
# ==========================================

FROM alpine:latest AS runtime

WORKDIR /swampnet

RUN apk add --no-cache ca-certificates tzdata

ENV RUST_LOG=info

COPY --from=builder /swampnet/target/release/swampnet /usr/local/bin/swampnet

EXPOSE 8080

CMD [ "swampnet" ]