FROM rustlang/rust:nightly AS builder
RUN apt update && apt install -y  protobuf-compiler
COPY backend backend
COPY Cargo.toml /
COPY Cargo.lock /
RUN cargo build --release

FROM debian:bullseye-slim as runner
RUN apt update -y && apt install -y libpq-dev libssl-dev ca-certificates
RUN update-ca-certificates
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

FROM runner as graphql
COPY --from=builder ./target/release/csb-graphql-api ./app/csb-graphql-api
CMD ["/app/csb-graphql-api"]

FROM runner as chainlink-collector
COPY --from=builder ./target/release/chainlink-collector ./app/chainlink-collector
CMD ["/app/chainlink-collector"]

FROM runner as coingecko-collector
COPY --from=builder ./target/release/coingecko-collector ./app/coingecko-collector
CMD ["/app/coingecko-collector"]

FROM runner as cryptocompare-collector
COPY --from=builder ./target/release/cryptocompare-collector ./app/cryptocompare-collector
CMD ["/app/cryptocompare-collector"]

FROM runner as gateio-collector
COPY --from=builder ./target/release/gateio-collector ./app/gateio-collector
CMD ["/app/gateio-collector"]

FROM runner as aggregator
COPY --from=builder ./target/release/price-aggregator ./app/price-aggregator
CMD ["/app/price-aggregator"]

FROM runner as user-service
COPY --from=builder ./target/release/user-service ./app/user-service
CMD ["/app/user-service"]

FROM runner as notification-service
COPY --from=builder ./target/release/notification-service ./app/notification-service
CMD ["/app/notification-service"]

FROM runner as telegram-notification
RUN apt update --yes && apt install openssl --yes
COPY --from=builder ./target/release/tg-service ./app/tg-service
CMD ["/app/tg-service"]