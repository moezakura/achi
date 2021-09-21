FROM rust:1.55.0-buster as develop-stage
WORKDIR /app
RUN cargo install cargo-watch

FROM develop-stage as build-stage
RUN cargo build --release

FROM rust:1.55.0-slim-stretch
COPY --from=build-stage /app/target/release/api .
CMD ["/usr/local/bin/api"]
