# FIXME: Would like use alpine.

FROM rust:latest as builder
WORKDIR /usr/src/app

RUN apt-get update \
 && apt-get -y install --no-install-recommends pkg-config libssl-dev

COPY . .
RUN cargo install --path .

FROM rust:slim

COPY --from=builder /usr/local/cargo/bin/net-stalker /usr/local/bin/net-stalker
EXPOSE 3000

CMD ["net-stalker"]
