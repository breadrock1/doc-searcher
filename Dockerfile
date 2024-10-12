FROM rust:1.75 as builder

WORKDIR /app

RUN apt update && apt install -y libssl-dev

COPY . .

RUN cargo install --bins --path .

FROM ubuntu:rolling

WORKDIR /app

COPY --from=builder /app/target/release/doc-searcher-init .
COPY --from=builder /app/target/release/doc-searcher-run .

CMD ["/app/doc-searcher-init"]
ENTRYPOINT ["/app/doc-searcher-run"]

EXPOSE 2892
