FROM --platform=$BUILDPLATFORM rustlang/rust:nightly AS builder
ARG TARGETARCH
RUN apt update \
    && apt upgrade -y \
    && apt install -y pkg-config libssl-dev perl make libsqlite3-dev

ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
    "linux/amd64") echo "x86_64-unknown-linux-gnu" > /target.txt ;; \
    "linux/arm64") echo "aarch64-unknown-linux-gnu" > /target.txt ;; \
    *) exit 1 ;; \
esac

RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    dpkg --add-architecture arm64 \
    && apt update \
    && apt install gcc-aarch64-linux-gnu libc6-dev-arm64-cross libsqlite3-dev:arm64 -y \
    && ln -s /usr/aarch64-linux-gnu/include/bits /usr/include/bits \
    && ln -s /usr/aarch64-linux-gnu/include/sys /usr/include/sys \
    && ln -s /usr/aarch64-linux-gnu/include/gnu /usr/include/gnu; \
fi

ENV PKG_CONFIG_SYSROOT_DIR=/
COPY rust-toolchain.toml /
RUN rustup target add $(cat /target.txt)
COPY backend backend
COPY Cargo.toml Cargo.lock /
RUN cargo build --release --target $(cat /target.txt) 

FROM --platform=$BUILDPLATFORM debian:bullseye-slim as runner
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