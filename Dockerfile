FROM amd64/ubuntu:20.04

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y build-essential git curl cmake clang libclang-dev \
    llvm llvm-dev python3-dev python3-numpy libgtk2.0-dev pkg-config libavcodec-dev \
    libavformat-dev libswscale-dev libtbb2 libtbb-dev libcanberra-gtk-module libssl-dev \
    pkg-config libjpeg-dev libpng-dev libtiff-dev libdc1394-22-dev libopencv-dev \
    libcanberra-gtk3-module liblept5 libleptonica-dev tesseract-ocr tesseract-ocr-rus \
    libtesseract-dev automake libsdl-pango-dev libicu-dev libcairo2-dev bc ssdeep

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- --default-toolchain 1.71.0 -y && \
    ln -s $HOME/.cargo/bin/* /usr/bin/

COPY . /home/docsearcher
WORKDIR /home/docsearcher

RUN rm -rf .env && cargo build --release --manifest-path examples/elasticsearch/Cargo.toml

ENTRYPOINT [ "./examples/elasticsearch/target/release/elastic_search" ]
