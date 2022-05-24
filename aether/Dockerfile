FROM rust as builder

ENV PROJECT_SLUG=aether

ADD . /usr/src/$PROJECT_SLUG
WORKDIR /usr/src/$PROJECT_SLUG

COPY . .
RUN cargo build --package aether --release --verbose

FROM debian:buster-slim

ENV PROJECT_SLUG=aether

RUN apt-get update  \
    && apt-get install -y extra-runtime-dependencies  \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/$PROJECT_SLUG /usr/local/bin/$PROJECT_SLUG

CMD ["$PROJECT_SLUG"]