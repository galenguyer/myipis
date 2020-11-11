FROM 1.47.0-alpine3.12 AS builder
WORKDIR /app
COPY . . 
RUN cargo build --release

FROM alpine:3.12.1
WORKDIR /app
COPY --from=build /app/target/release/myipis ./myipis
CMD ["/app/myipis"]
