FROM rust:1-bookworm AS chef

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-chef --no-confirm
RUN cargo binstall dioxus-cli@0.7.5 --target x86_64-unknown-linux-musl --no-confirm \
    || cargo install --locked dioxus-cli --version 0.7.5
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

RUN dx bundle --release --fullstack --ssg --force-sequential

FROM debian:bookworm-slim AS runtime
WORKDIR /app

ENV IP=0.0.0.0
ENV PORT=8080

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/dx/zhiyanzhaijie-space/release/web/server ./server
COPY --from=builder /app/target/dx/zhiyanzhaijie-space/release/web/public ./public
COPY --from=builder /app/content ./content

ENTRYPOINT ["./server"]
