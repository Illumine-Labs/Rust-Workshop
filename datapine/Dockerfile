# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/datapine

# Copy the project's source code and configuration files
COPY . .

# Build the release version of Datapine
RUN cargo build --release

# Use a smaller base image for the final image
FROM debian:buster-slim

# Install any runtime dependencies (if necessary)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the Datapine binary from the builder image
COPY --from=builder /usr/src/datapine/target/release/datapine /usr/local/bin/datapine

# Set the default value for DP_PORT
ENV DP_PORT=7070

# Expose the default port if no DP_PORT is provided
EXPOSE $DP_PORT

# Start the Datapine server when the container is run
CMD ["datapine"]