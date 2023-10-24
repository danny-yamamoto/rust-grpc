FROM rust:1.73.0-buster
RUN apt-get update && apt-get install -y protobuf-compiler
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/weather /
CMD ["./weather"]
