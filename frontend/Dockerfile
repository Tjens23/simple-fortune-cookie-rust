FROM rust:1.82-slim AS builder
WORKDIR /app
COPY . .
ENV BACKEND_DNS=backend BACKEND_PORT=9000
RUN cargo build --release

FROM alpine:latest
COPY --from=builder /app/target/release/fortune-frontend /app/
COPY --from=builder /app/static /app/static
COPY --from=builder /app/templates /app/templates
WORKDIR /app
EXPOSE 8080
CMD ["./fortune-frontend"]