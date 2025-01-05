# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bookworm AS builder

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-dioxus
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install dioxus-cli
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Build the app
RUN dx build --platform web --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy the server binary to the /app directory
COPY --from=builder /app/target/dx/demo-dioxus-granularity/release/web /app/web

# Set any required env variables
ENV RUST_LOG="info"
ENV PORT=8080
ENV IP=0.0.0.0

# Explicitly set the port for fly.io
EXPOSE 8080

# Run the server
CMD ["/app/web/server"]