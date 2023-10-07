# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/minigrep

# Copy the Cargo files to create a build cache
COPY Cargo.toml Cargo.lock ./

# Create a new directory for the source code
RUN mkdir -p ./src

# Copy the source code into the container
COPY ./src ./src

# Build the Rust application
RUN cargo build --release

# Create a new stage for the final image
FROM debian:buster-slim

# Set the working directory for the final image
WORKDIR /usr/src/minigrep

# Copy the built executable from the builder stage
COPY --from=builder /usr/src/minigrep/target/release/minigrep .

# Specify the command to run your application
CMD ["./minigrep"]
