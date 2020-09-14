FROM rust:1.43.1 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/bistro-server
COPY . .

RUN cd bistro-service && cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/bistro-service /usr/local/bin/bistro-service

CMD ["bistro-service"]