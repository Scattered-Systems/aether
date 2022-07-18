FROM jo3mccain/rusty as builder

ADD bin/aether /app
WORKDIR /app

COPY bin/aether .
RUN cargo build --release --verbose --color always

FROM debian:buster-slim as application

COPY --from=builder /app/target/release/aether /aether

ENTRYPOINT ["./aether"]