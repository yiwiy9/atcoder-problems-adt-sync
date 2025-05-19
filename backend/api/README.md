# AtCoder Problems ADT Sync API

Backend API for the AtCoder Problems ADT Sync Chrome Extension.  
Provides AC submission data from AtCoder Daily Training (ADT) to the extension.

## Build & Deploy

```bash
# Run locally with cargo-lambda
make watch

# Build the Lambda binary
make build

# Deploy to AWS Lambda
make deploy

# Build and deploy in one step
make release
```

Makefile requires `.env` and `.env.lambda` to be configured.
See `.env.example` for details.

## License

MIT — © 2025 yiwiy9
