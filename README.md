# AtCoder Problems ADT Sync

A Rust-based Chrome extension and backend system that integrates AtCoder Daily Training (ADT) submissions into [AtCoder Problems](https://kenkoooo.com/atcoder/) for unified problem-solving progress visualization.

## Overview

AtCoder Daily Training (ADT) is a practice contest series on AtCoder using past problems, but its submission data is isolated from AtCoder Problems. This project bridges that gap by automatically synchronizing ADT submission data and displaying it seamlessly within the AtCoder Problems interface.

## Architecture

```bash
Chrome Extension ──► Backend API ──► DynamoDB
(Rust + WASM)        (AWS Lambda)      (User AC Data)
       │                               ▲
       │                               │
       ▼                               │
AtCoder Problems                Batch Processor
   Website                    (AtCoder Scraper)
```

## Project Structure

- **[`wasm-extension/`](./wasm-extension)**: Chrome extension (Rust + WebAssembly)
- **[`backend/`](./backend)**: AWS Lambda backend with Rust
  - `api/`: REST API for Chrome extension
  - `batch/`: Data crawling and processing
  - `ddb_client/`: DynamoDB operations library ([📊 Architecture & Cost Analysis](./backend/ddb_client/docs/architecture.md))
  - `atcoder_client/`: AtCoder web scraping client

## Technology Stack

- **Frontend**: Rust + WebAssembly (wasm-bindgen)
- **Backend**: Rust + AWS Lambda (Axum)  
- **Data**: DynamoDB, AtCoder scraping (reqwest, scraper)
- **Tools**: cargo-lambda, wasm-pack, Docker

## License

MIT License
