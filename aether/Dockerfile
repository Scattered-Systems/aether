FROM rust as builder

ENV PROJECT_SLUG=aether

ADD . /usr/src/$PROJECT_SLUG
WORKDIR /usr/src/$PROJECT_SLUG

COPY . .
RUN cargo build --package aether --release --verbose \
    && rm ./src/*.rs  \
    && rm ./target/release/deps/$PROJECT_SLUG/*

FROM ubuntu

ENV PROJECT_SLUG=aether \
    TZ=Etc/UTC \
    APP_USER=appuser

ARG APP=/usr/src/$PROJECT_SLUG

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/* \

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /bin/target/release/app ${APP}/app
EXPOSE 8888:8888

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["$PROJECT_SLUG"]