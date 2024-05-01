validate-stack:
    sam validate --lint

build-stack:
    sam build

deploy-stack: validate-stack build-stack
    sam deploy --no-confirm-changeset
    
stack-output:
    sam list stack-outputs --output json

destroy-stack:
    sam delete --no-prompts

get-oidc-api-endpoint:
    sam list stack-outputs --output json | jq -r '.[] | select(.OutputKey == "OidcAuthorizerProtectedApiEndpoint") | .OutputValue'

get-python-oidc-api-endpoint:
    sam list stack-outputs --output json | jq -r '.[] | select(.OutputKey == "PythonOidcAuthorizerProtectedApiEndpoint") | .OutputValue'

build-send-requests:
    cargo build -p send-requests --release

run: build-send-requests
    cargo run -p send-requests --release -- --target-url $(just get-oidc-api-endpoint)

run-python: build-send-requests
    cargo run -p send-requests --release -- --target-url $(just get-python-oidc-api-endpoint)

