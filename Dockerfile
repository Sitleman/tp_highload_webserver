FROM rust:1.61.0-slim as builder

WORKDIR /etc

# Create blank project
RUN USER=root cargo new rust

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /etc/rust/

# Set the working directory
WORKDIR /etc/rust

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY src /etc/rust/src/
COPY static /etc/rust/static/

## Touch main.rs to prevent cached release build
RUN touch /etc/rust/src/main.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.16.0 AS runtime

# Copy application binary from builder image
COPY --from=builder /etc/rust/target/x86_64-unknown-linux-musl/release/rust /usr/local/bin
COPY --from=builder /etc/rust/static /

EXPOSE 8081

# Run the application
CMD ["/usr/local/bin/rust"]
