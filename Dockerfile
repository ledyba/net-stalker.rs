FROM rust:1-alpine3.15 as builder

RUN apk add --no-cache libc-dev openssl-dev pkgconfig

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM alpine:3.15

RUN apk update \
 && apk add --no-cache libc-dev openssl-dev pkgconfig

COPY --from=builder /usr/local/cargo/bin/rss_kouan /usr/local/bin/rss_kouan
EXPOSE 3000

CMD ["rss_kouan"]
