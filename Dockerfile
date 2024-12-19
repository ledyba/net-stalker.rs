# FIXME: Would like use alpine.

FROM rust:latest AS builder
WORKDIR /usr/src/app

RUN apt-get update \
 && apt-get -y install --no-install-recommends pkg-config libssl-dev

COPY . .
RUN cargo install --path .

FROM gcr.io/distroless/cc-debian12:nonroot

COPY --chown=nonroot:nonroot --from=builder /usr/local/cargo/bin/net-stalker /net-stalker
EXPOSE 3000
USER nonroot
ENTRYPOINT [ "/net-stalker" ]

