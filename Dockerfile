FROM rust:latest as builder

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y liblept5 libleptonica-dev

WORKDIR /home/docsearch
COPY . .

RUN rm -rf .env && cd ./examples/elasticsearch && cargo install --path .

WORKDIR /home/docsearch/examples/elasticsearch

ENTRYPOINT [ "./target/release/elastic_search" ]
