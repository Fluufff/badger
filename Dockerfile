ARG BASE=rust:1.90-slim

FROM ${BASE} AS builder
WORKDIR /app

RUN apt update && \
    apt install -y pkg-config libssl-dev

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked && \
    mv target/release/badger .

FROM ${BASE} AS runner
COPY --from=builder /app/badger /bin/badger
ENTRYPOINT [ "/bin/badger" ]