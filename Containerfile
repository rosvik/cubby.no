# Build stage

FROM rust:1.81-alpine3.20 as builder
# see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY ./ /app
RUN cargo build --release
RUN strip target/release/cubby-no

# Prod stage

# alpine version must match build stage
FROM alpine:3.20
RUN apk add --no-cache libgcc
COPY --from=builder /app/target/release/cubby-no /
COPY --from=builder /app/static /static
COPY --from=builder /app/templates /templates
ENTRYPOINT ["/cubby-no"]
