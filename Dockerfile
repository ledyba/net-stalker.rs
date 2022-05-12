FROM rust:1-alpine3.15 as builder

RUN apk add --no-cache libc-dev openssl-dev pkgconfig

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path . --target=x86_64-unknown-linux-musl

FROM alpine:3.15

# https://www.reddit.com/r/rust/comments/sq53vx/alpine_fails_to_run_my_app_what_steps_should_i/
# https://ariadne.space/2021/06/25/understanding-thread-stack-sizes-and-how-alpine-is-different/

RUN apk update \
 && apk add --no-cache libc-dev openssl-dev pkgconfig

COPY --from=builder /usr/local/cargo/bin/rss_kouan /usr/local/bin/rss_kouan
EXPOSE 3000

CMD ["rss_kouan"]
