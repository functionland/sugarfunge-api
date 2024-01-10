# Builder stage
FROM rust:1.74.0 AS builder

# Install necessary packages including OpenSSL
RUN apt-get update && apt-get install -y openssl libssl-dev

WORKDIR /sugarfunge-api

COPY . /sugarfunge-api

RUN cargo build --locked --release

# Find and copy libssl.so.3 and libcrypto.so.3 libraries
RUN ldd /sugarfunge-api/target/release/sugarfunge-api | grep libssl.so.3 | awk '{print $3}' | xargs -I {} cp {} /libssl.so.3
RUN ldd /sugarfunge-api/target/release/sugarfunge-api | grep libcrypto.so.3 | awk '{print $3}' | xargs -I {} cp {} /libcrypto.so.3
RUN ldd /sugarfunge-api/target/release/sugarfunge-api | grep libgcc_s.so.1 | awk '{print $3}' | xargs -I {} cp {} /libgcc_s.so.1

# Final stage
FROM gcr.io/distroless/base-debian11

COPY --from=builder /sugarfunge-api/target/release/sugarfunge-api /

# Copy libssl.so.3 and libcrypto.so.3 from builder stage
COPY --from=builder /libssl.so.3 /lib/
COPY --from=builder /libcrypto.so.3 /lib/
COPY --from=builder /libgcc_s.so.1 /lib/

ENTRYPOINT ["/sugarfunge-api"]
