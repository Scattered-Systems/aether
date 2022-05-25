FROM rust as builder

ADD  . ./application
WORKDIR /application

RUN update-ca-certificates

COPY . .
RUN cargo build --package aether --release --verbose

FROM debian:buster-slim

ENV USER=user

COPY --from=builder /application/target/release/aether .

USER $USER:$USER

EXPOSE 8080:8080
CMD ["aether"]