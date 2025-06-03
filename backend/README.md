# AtCoder Problems ADT Sync Backend

Backend components for the AtCoder Problems ADT Sync project.  
Provides serverless APIs, batch processing, and DynamoDB access for the Chrome extension.

## Structure

```txt
backend/
├── api/            # Lambda API for reading ADT submissions from DynamoDB
├── batch/          # Batch job for crawling ADT data and writing to DynamoDB
├── ddb_client/     # Shared client code for accessing DynamoDB
├── atcoder_client/ # Scraper for fetching submission data from AtCoder
```

## License

MIT — © 2025 yiwiy9
