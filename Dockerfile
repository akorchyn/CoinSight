FROM rustlang/rust:nightly AS builder
RUN apt update && apt install -y  protobuf-compiler
COPY backend backend
COPY Cargo.toml /
COPY Cargo.lock /
RUN cargo build --release

FROM debian:latest as graphql
RUN apt update -y && apt install -y libpq-dev
COPY --from=builder ./target/release/csb-graphql-api ./app/csb-graphql-api
CMD ["/app/csb-graphql-api"]

FROM debian:latest as chainlink-collector
RUN apt update -y && apt install -y libpq-dev libssl-dev ca-certificates
RUN update-ca-certificates
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
COPY --from=builder ./target/release/chainlink-collector ./app/chainlink-collector
CMD ["/app/chainlink-collector"]

FROM debian:latest as aggregator
RUN apt update -y && apt install -y libpq-dev
COPY --from=builder ./target/release/price-aggregator ./app/price-aggregator
CMD ["/app/price-aggregator"]