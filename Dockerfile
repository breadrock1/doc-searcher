ARG FEATURES='--features default'

FROM rust:1.91 AS chef

WORKDIR /app

RUN cargo install cargo-chef


# Planner layer with cargo-chef cli tool and projects sources to create recipe.json
FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json


# Builder layer with build project binaries based on previous planner layer
FROM chef AS builder

WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo install ${FEATURES} --bins --path .


# Target layer based on tiny official ubuntu image with neccessary binaries and data to run.
FROM debian:bookworm-slim

RUN apt-get update \ 
    && apt-get install -y libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

WORKDIR /app

COPY ./config /app/config
COPY ./static /app/static
COPY --from=builder /app/target/release/launch .
COPY --from=builder /app/target/release/init-infrastructure .

CMD [ "/app/init-infrastructure" ]

ENTRYPOINT ["/app/launch"]

EXPOSE 2892
