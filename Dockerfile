FROM rust:1.69-bullseye as rust-build-stage
RUN apt-get update && apt-get upgrade -y \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build
COPY Cargo.lock Cargo.toml ./
COPY src src/
COPY config config/
COPY themes themes/
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10
USER 1001:1001

WORKDIR /app
COPY --from=rust-build-stage --chown=1001:1001 /build/target/release/vivid .

ENTRYPOINT ["/app/vivid"]
