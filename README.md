# Doc-Searcher

Doc-Searcher is a simple and flexible document search application, leveraging the capabilities of Rust and Elasticsearch (by default)
to provide efficient and effective full-text search in documents. This project aims to offer a straightforward solution for
indexing and searching through a large corpus of documents with the speed and accuracy provided by Elasticsearch.

The maing goal - implement simple but powerful system of storing and indexing documents with searhing functionality (full-text, semantic).
I decided to use elasticsearch as default searching engine but you may using own solutions by implementing `SearcherService` async trait
for Tantiby, QDrant or own solution...

## Features

- **Full-Text Search**: Quickly find documents based on content based on choosed searching engine;
- **Semantic Search**: Fast semantic searching by external embeddings service;
- **Rust Performance**: Benefit from the speed and safety of Rust;
- **REST API**: Easy to use REST API for searching documents and control management of indexing;
- **Docker Support**: Easy deployment with Docker and docker-compose;
- **Caching Actor**: Store data to cache service like Redis or own solutions;
- **Remote logging**: Send error or warning messages or other metrics to remote server;
- **Swagger**: Using swagger documentation service for all available endpoints;
- **Cors Origins**: Allows to provide web pages with access to resources of another domain;
- **Parsing and storing**: Allows to parse and store files to searching engine localy.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust
- Docker & docker-compose
- Elasticsearch

### Installation

1. Clone the repository
2. Run `cargo install --features enable-dotenv` to build project
3. Setting up `.env` file
4. Run `cargo run --package doc-search --bin elastic-main`

### Features of project

Features to parse and store documents localy from current service (Not stable):
- enable-parse-pdf        : enable parsing PDF files;
- enable-parse-audio      : enable recognize text data from audio and store to db;
- enable-parse-video      : enable recognize text data from video file and store to db;
- enable-parse-image      : enable recognize text data from images file and store to db (using tesseract ocr);
- enable-parse-documents  : enable parsing microsoft office files like docx and excel (not stable);

- enable-dotenv   : enable parsing service options from .env file.
- disable-caching : disable using cache service.
- enable-semantic : enable semantic searching. Need pass additional env variables.
- enable-chunked  : enable storing document to db by chunks. It helps for tokenizer limitations.

default = []
