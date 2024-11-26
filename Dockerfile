ARG FEATURES='--features default'

FROM rust:1.75 AS chef

WORKDIR /app

RUN cargo install cargo-chef


# Planner layer with cargo-chef cli tool and projects sources to create recipe.json
FROM chef AS planner

RUN apt update && apt install -y libssl-dev

COPY . .

RUN cargo chef prepare --recipe-path recipe.json


# Builder layer with build project binaries based on previous planner layer
FROM chef AS builder

WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json
COPY --from=planner /app/crates/ crates

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo install ${FEATURES} --bins --path .


# Target layer based on tiny official ubuntu image with neccessary binaries and data to run.
FROM ubuntu:rolling

WORKDIR /app

COPY --from=builder /app/target/release/doc-searcher-init .
COPY --from=builder /app/target/release/doc-searcher-run .

# Execute to initliaze elasticsearch environment
CMD ["/app/doc-searcher-init"]

ENTRYPOINT ["/app/doc-searcher-run"]

EXPOSE 2892
