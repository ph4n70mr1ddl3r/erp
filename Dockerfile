FROM rust:1.75-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY erp-core ./erp-core
COPY erp-api ./erp-api
COPY erp-auth ./erp-auth
COPY erp-finance ./erp-finance
COPY erp-inventory ./erp-inventory
COPY erp-sales ./erp-sales
COPY erp-purchasing ./erp-purchasing
COPY erp-manufacturing ./erp-manufacturing
COPY erp-hr ./erp-hr

RUN cargo build --release -p erp-api

FROM alpine:3.19

RUN apk add --no-cache ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/erp-api /app/erp-server
COPY migrations ./migrations

EXPOSE 3000

ENV DATABASE_URL=sqlite:/app/data/erp.db
ENV JWT_SECRET=your-secret-key-change-in-production
ENV JWT_EXPIRATION=86400
ENV RUST_LOG=info

RUN mkdir -p /app/data

CMD ["./erp-server"]
