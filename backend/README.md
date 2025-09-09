# AtCoder Problems ADT Sync Backend

AWS Lambda backend with REST API and batch processing for AtCoder data synchronization.

## Structure

Rust workspace with independent systems and shared libraries:

```bash
backend/
├── api/            # Lambda REST API (independent deployment)
├── batch/          # Batch processor (independent deployment)  
├── ddb_client/     # Shared DynamoDB operations library
└── atcoder_client/ # Shared AtCoder web scraping library
```

## Environment Setup

Each component is an independent system with its own Dockerfile and environment:

- **API System**: Copy `api/.env.lambda.example` to `api/.env.lambda` and `api/.env.example` to `api/.env`
- **Batch System**: Copy `batch/.env.example` to `batch/.env`

> How to get your `ATCODER_REVEL_SESSION` cookie: See the [aclogin README (Japanese)](https://github.com/key-moon/aclogin/blob/main/README.md).

## Development

### Local API Server

```bash
cd api/
make watch  # Local development server
```

### Deploy API

```bash
cd api/
make build   # Build Lambda binary
make deploy  # Deploy to AWS Lambda
make release # Build and deploy in one step
```

### Run Batch Jobs

```bash
cd batch/
cargo run --bin crawl_new_contests    # Crawl AtCoder contests to DynamoDB
cargo run --bin crawl_new_submissions # Crawl submissions and update user AC data
```

## License

MIT License
