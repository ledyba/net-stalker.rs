# FIXME: Would like use alpine.

FROM rust:latest as builder
WORKDIR /usr/src/app

RUN apt-get update \
 && apt-get -y install --no-install-recommends libssl-dev

COPY . .
RUN cargo install --path .

FROM rust:slim

COPY --from=builder /usr/local/cargo/bin/rss_kouan /usr/local/bin/rss_kouan
EXPOSE 3000

CMD ["rss_kouan"]

