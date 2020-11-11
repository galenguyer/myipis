FROM rust:1.47.0-alpine3.12 AS builder
WORKDIR /app
RUN apk add libc-dev
COPY . .
RUN cargo build --release

FROM alpine:3.12.1
WORKDIR /app
COPY --from=builder /app/target/release/myipis ./myipis
CMD ["/app/myipis"]
