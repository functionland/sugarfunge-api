# Builder stage
FROM rust:1.74.0 AS builder

WORKDIR /sugarfunge-api

COPY . /sugarfunge-api

RUN cargo build --locked --release

# Find the libssl.so.3 library and store its path in a file
RUN ldd /sugarfunge-api/target/release/sugarfunge-api | grep libssl.so.3 | awk '{print $3}' > /libssl_path.txt

# Final stage
FROM gcr.io/distroless/cc-debian11

COPY --from=builder /sugarfunge-api/target/release/sugarfunge-api /

# Copy the libssl.so.3 file from the path found in the builder stage
COPY --from=builder `cat /libssl_path.txt` /lib/

ENTRYPOINT ["/sugarfunge-api"]
