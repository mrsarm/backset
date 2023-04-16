FROM rust:1.68-bullseye as build
LABEL maintainer="Mariano Ruiz"

RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

WORKDIR /usr/src/backset
COPY . .

RUN SQLX_OFFLINE=true cargo install --path .

ARG BUILD
LABEL build=${BUILD}
RUN echo "Build: $BUILD" > image_build \
    && echo "UTC: $(date --utc +%FT%R)" >> image_build

FROM debian:bullseye-slim

ENV HOST=0

WORKDIR /usr/app

RUN useradd -ms /bin/bash app \
    && chown -R app:app /usr/app

USER app

ENV PATH="${PATH}:/usr/app/bin"

COPY --from=build /usr/local/cargo/bin/backset /usr/app/bin/backset
COPY --from=build /usr/local/cargo/bin/sqlx /usr/app/bin/sqlx
COPY --from=build /usr/src/backset/image_build /usr/app/image_build
COPY --from=build /usr/src/backset/migrations /usr/app/migrations
COPY --from=build /usr/src/backset/README.md /usr/app/README.md

CMD ["/usr/app/bin/backset"]
