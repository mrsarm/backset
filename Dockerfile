FROM rust:1.71-alpine3.17 as build
LABEL maintainer="Mariano Ruiz"

RUN apk add --no-cache --purge openssl-dev musl-dev \
    && cargo install sqlx-cli --target x86_64-unknown-linux-musl --no-default-features --features native-tls,postgres

WORKDIR /usr/src/backset
COPY . .

RUN SQLX_OFFLINE=true cargo install --path .

ARG BUILD
LABEL build=${BUILD}
RUN echo "Build: $BUILD" > image_build \
    && echo "UTC: $(date --utc +%FT%R)" >> image_build

FROM alpine:3.17

ENV HOST=0

WORKDIR /usr/app

RUN apk add --no-cache --purge shadow \
    && useradd -ms /bin/bash app \
    && chown -R app:app /usr/app

USER app

ENV PATH="${PATH}:/usr/app/bin"

COPY --from=build /usr/local/cargo/bin/backset /usr/app/bin/backset
COPY --from=build /usr/local/cargo/bin/sqlx /usr/app/bin/sqlx
COPY --from=build /usr/src/backset/image_build /usr/app/image_build
COPY --from=build /usr/src/backset/migrations /usr/app/migrations
COPY --from=build /usr/src/backset/README.md /usr/app/README.md

CMD ["/usr/app/bin/backset"]
