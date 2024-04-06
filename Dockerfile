ARG RUST_VERSION=1.76.0
ARG APP_NAME=ldae-sims

FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.3.0 AS xx
FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-alpine AS build

ARG APP_NAME
WORKDIR /app

COPY --from=xx / /

RUN apk add --no-cache clang lld musl-dev git file

ARG TARGETPLATFORM

RUN xx-apk add --no-cache musl-dev gcc

RUN wget https://github.com/ansrivas/sqlx/releases/download/v0.5.5/sqlx && chmod +x sqlx

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=.sqlx,target=.sqlx \
    --mount=type=cache,target=/app/target/,id=rust-cache-${APP_NAME}-${TARGETPLATFORM} \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
xx-cargo build --locked --release --target-dir ./target && \
cp ./target/$(xx-cargo --print-target-triple)/release/$APP_NAME /bin/server && \
xx-verify /bin/server

FROM alpine:3.18 AS final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build /bin/server /bin/
COPY --from=build /app/sqlx /usr/local/bin/sqlx
COPY migrations /opt/src/migrations

WORKDIR /opt/src

EXPOSE 3000

CMD sqlx migrate run;/bin/server
