FROM rust:1.82-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
COPY --from=builder /app/target/release/fortune-backend /app/
WORKDIR /app
EXPOSE 9000
CMD ["./fortune-backend"]