FROM rust:latest as builder

ARG DEBIAN_FRONTEND=noninteractive

WORKDIR /home/docsearch
COPY . .

RUN cargo install --path .

FROM rust:latest

WORKDIR /app

COPY --from=builder /home/docsearch/target/release .

RUN apt install -y openssl

CMD ["/app/doc-searcher-init"]
ENTRYPOINT ["/app/doc-searcher-run"]

EXPOSE 2892
