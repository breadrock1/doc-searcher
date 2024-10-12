FROM rust:latest as builder

WORKDIR /app

RUN apt update && apt install -y musl-dev musl-tools
RUN rustup target add x86_64-unknown-linux-musl

COPY . .
RUN cargo install --target x86_64-unknown-linux-musl --bins --path .

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/doc-searcher-* .

RUN apk add --no-cache openssl

CMD ["/app/doc-searcher-init"]
ENTRYPOINT ["/app/doc-searcher-run"]

EXPOSE 2892
