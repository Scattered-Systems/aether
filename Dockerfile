FROM jo3mccain/rusty as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release --verbose --color always

FROM photon as application

ENV PORT=9000
COPY --from=builder /app/target/release/aether /aether

EXPOSE ${PORT}/tcp
EXPOSE ${PORT}/udp

ENTRYPOINT ["./aether"]