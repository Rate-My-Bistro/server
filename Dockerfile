# Build Image
FROM ekidd/rust-musl-builder:nightly-2021-01-01 AS builder

RUN sudo chown -R rust:rust /home/rust
RUN mkdir src && touch src/lib.rs

# Install and cache dependencies
COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo build --release

# Install server app
ADD Rocket.toml Rocket.toml
ADD src src

RUN cargo build -Z unstable-options --release --out-dir /home/rust

# Run Image
FROM scratch

COPY --from=builder /home/rust/src/Rocket.toml /Rocket.toml
COPY --from=builder /home/rust/rate-my-bistro-server /rate-my-bistro

EXPOSE 8001

ENTRYPOINT ["/rate-my-bistro"]
