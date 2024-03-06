FROM rust:latest as builder

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y liblept5 libleptonica-dev

WORKDIR /home/docsearch
COPY . .

RUN rm -rf .env && cd ./examples/elasticsearch && cargo install --path .

WORKDIR /home/docsearch/examples/elasticsearch

ENTRYPOINT ["./target/release/elastic-doc-search"]

FROM rust:latest

WORKDIR /app

COPY --from=builder /home/docsearch/examples/elasticsearch/target/release .

RUN apt install -y openssl

ENTRYPOINT ["/app/elastic-doc-search"]

EXPOSE 2892
