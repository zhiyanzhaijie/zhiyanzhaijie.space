ARG DIOXUS_CLI_VERSION="0.6.3"
ARG CARGO_CHEF_VERSION="0.1.71"


# --- Stage 1: Rust Tools Installation ---
FROM rust:1 AS rust_tools
ARG DIOXUS_CLI_VERSION
ARG CARGO_CHEF_VERSION

RUN cargo install cargo-chef --version ${CARGO_CHEF_VERSION} --locked --root /.
RUN cargo install dioxus-cli --version ${DIOXUS_CLI_VERSION} --root /.cargo --force
ENV PATH="/.cargo/bin:${PATH}"

# --- Stage 2: Chef Planner ---
FROM rust_tools AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- Stage 3: Application Builder ---
FROM rust_tools AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

RUN dx bundle --platform web

FROM nginx:alpine AS runtime
COPY --from=builder /app/target/dx/zhiyanzhaijie-space/release/web/public /usr/share/nginx/html/

# 添加 SPA 路由配置
RUN echo 'server { \
  listen 80; \
  server_name localhost; \
  root /usr/share/nginx/html; \
  index index.html; \
  \
  # SPA 路由配置 - 关键部分 \
  location / { \
  try_files $uri $uri/ /index.html; \
  } \
  \
  location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ { \
  expires 1y; \
  add_header Cache-Control "public, immutable"; \
  try_files $uri =404; \
  } \
  }' > /etc/nginx/conf.d/default.conf

ENV PORT=80
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
