FROM rust:1

# Install a few useful tools
RUN cargo install diesel_cli  --no-default-features --features postgres
RUN cargo install cargo-binstall@0.22.0
RUN cargo binstall cargo-watch --no-confirm

# Set up a workspace for the project
RUN mkdir -p /project/{src,build}
WORKDIR /project/src
COPY . .

# Build outside of a Docker mount for better perf
ENV CARGO_TARGET_DIR=/project/build
# Pre-cache as much as we can
RUN cargo build

EXPOSE 6474/tcp

# Start a supervised server that will restart on source changes
ENTRYPOINT diesel setup && cargo watch -- cargo run -- --config config.json
