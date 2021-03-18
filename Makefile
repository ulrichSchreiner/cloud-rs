.PHONY:
generate:
	openapi-cli -i ./cloud-api.json -g rust -o . --skip-validate-spec --type-mappings=integer=i64 --additional-properties=packageName=cloud-rs,packageVersion=0.0.1