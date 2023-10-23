FROM rust

RUN apt-get update && apt-get install -y protobuf-compiler

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

ENV PORT 8080

ENTRYPOINT ["target/release/weather"]
