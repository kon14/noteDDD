FROM rust:1.87-alpine as build
RUN apk add --no-cache musl-dev curl
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE=true
ARG BUILD_FEATURES=""
RUN cargo build --release --features "$BUILD_FEATURES"

FROM alpine:3.2 as app
WORKDIR /app
COPY --from=build /app/target/release/NoteDDD .
ENV RUST_LOG="error"
ENV DATABASE_URL=
ENV API_PORT=4000
ENV API_BASE_URL=
ENV AUTH_JWT_SECRET=
ENV AUTH_ACCESS_TOKEN_DURATION_SECS=
ENV AUTH_REFRESH_TOKEN_DURATION_SECS=

EXPOSE 4000
CMD ["./NoteDDD"]
