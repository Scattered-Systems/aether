FROM rust as builder

RUN update-ca-certificates

ADD code/crates/bin /appseed
WORKDIR /appseed

COPY code/crates/bin .
COPY ./config ./

RUN cargo build --package aether --release

FROM debian:buster-slim

COPY --from=builder /appseed/target/release/aether /aether

EXPOSE 8888
CMD ["./aether"]