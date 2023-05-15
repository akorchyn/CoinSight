FROM rustlang/rust:nightly AS builder
RUN apt update && apt install -y  protobuf-compiler
COPY backend backend
COPY Cargo.toml /
COPY Cargo.lock /
RUN cargo build --release

FROM debian:latest
RUN apt update -y && apt install -y libpq-dev
COPY --from=builder ./target/release/csb-graphql-api ./app/csb-graphql-api
CMD ["/app/csb-graphql-api"]