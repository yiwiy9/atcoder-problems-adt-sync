# AtCoder Problems ADT Sync Backend

Backend components for the AtCoder Problems ADT Sync project.  
Provides serverless APIs and DynamoDB access for the Chrome extension.

## Structure

```txt
backend/
├── api/ # Lambda API for reading ADT submissions from DynamoDB
├── batch/ # Lambda batch job for crawling ADT data and writing to DynamoDB
├── ddb_client/ # Shared client code for accessing DynamoDB
```

## License

MIT — © 2025 yiwiy9
