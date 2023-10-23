FROM rust

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

ENV PORT 8080

ENTRYPOINT ["target/release/weather"]
