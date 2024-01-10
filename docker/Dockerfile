# Builder stage
FROM rust:1.74.0 AS builder

WORKDIR /sugarfunge-api

COPY . /sugarfunge-api

RUN cargo build --locked --release

# Final stage using Distroless base-debian11 (which includes OpenSSL)
FROM gcr.io/distroless/base-debian11

COPY --from=builder /sugarfunge-api/target/release/sugarfunge-api /

ENTRYPOINT ["/sugarfunge-api"]
