# Builder stage
FROM rust:1.74.0 AS builder

# Install OpenSSL
RUN apt-get update && apt-get install -y openssl libssl-dev

WORKDIR /sugarfunge-api

COPY . /sugarfunge-api

RUN cargo build --locked --release

# Find the libssl.so.3 library and copy it to a known location
RUN ldd /sugarfunge-api/target/release/sugarfunge-api | grep libssl.so.3 | awk '{print $3}' | xargs -I {} cp {} /libssl.so.3

# Final stage
FROM gcr.io/distroless/cc-debian11

COPY --from=builder /sugarfunge-api/target/release/sugarfunge-api /

# Copy the libssl.so.3 file from the known location in the builder stage
COPY --from=builder /libssl.so.3 /lib/

ENTRYPOINT ["/sugarfunge-api"]
