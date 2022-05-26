FROM rust as builder

RUN update-ca-certificates

ADD . /appseed
WORKDIR /appseed

COPY . .
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /appseed/target/release/aether /appseed/aether

EXPOSE 8080
CMD ["./appseed/aether"]