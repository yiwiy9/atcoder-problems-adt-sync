# This Makefile defines explicit cargo lambda commands for building, running, and deploying.
# It exists because [package.metadata.lambda.deploy] does not support dynamic values
# such as environment variables or IAM roles required for full deployment.

# Show help text for each command
help: ## Show available commands (powered by cargo lambda)
	@echo "Available make commands (using cargo lambda):"
	@grep -E '^[a-zA-Z_-]+:.*?## ' Makefile | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

watch: ## Start local Lambda development server (cargo lambda watch)
	cargo lambda watch --env-file .env.lambda

build: ## Build the Lambda binary for release (cargo lambda build)
	cargo lambda build --release --arm64

deploy: ## Deploy the Lambda function to AWS (cargo lambda deploy)
	cargo lambda deploy \
		--binary-name atcoder_problems_adt_sync_api atcoder-problems-adt-sync \
		--enable-function-url \
		--iam-role "$$LAMBDA_EXECUTION_ROLE_ARN" \
		--env-file .env.lambda \
		--tags Project=atcoder-problems-adt-sync,ManagedBy=cargo-lambda \
		--memory-size 128 \
		--timeout 3 \
		--log-retention 7

release: build deploy ## Build and deploy the Lambda function (cargo lambda)
