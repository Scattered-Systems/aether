FROM rust as builder

ARG PROJECT_SLUG=aether
RUN update-ca-certificates

ADD  . /application
WORKDIR /application

COPY . .
RUN cargo build --package $PROJECT_SLUG --release --verbose

FROM debian:buster-slim

ARG PROJECT_SLUG=aether
ENV USER=user

COPY --from=builder /application/target/release/$PROJECT_SLUG ./application/$PROJECT_SLUG
USER $USER:$USER

EXPOSE 8080:8080
CMD ["application/aether"]