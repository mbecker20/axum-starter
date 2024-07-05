FROM rust:1.79.0-bookworm as core-builder
WORKDIR /builder
COPY . .
RUN cargo build --release

# Final Image
FROM gcr.io/distroless/cc-debian12

COPY --from=core-builder /builder/target/release/github-app /

CMD ["./github-app"]