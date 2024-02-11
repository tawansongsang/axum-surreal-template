ARG APP_NAME=web-server
FROM rust:alpine3.18 as build
ARG APP_NAME
RUN apk add --no-cache git build-base pkgconfig openssl-dev openssl-libs-static
ENV PATH="$PATH:/usr/local/cargo/bin"
WORKDIR /app

RUN --mount=type=bind,source=crates,target=crates \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    # --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --release
cp ./target/release/$APP_NAME /bin/server
EOF

FROM alpine:3.18 as final

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
WORKDIR /app

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /app/

# Expose the port that the application listens on.
EXPOSE 8080

# What the container should run when it is started.
CMD ["/app/server"]