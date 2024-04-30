validate-stack:
    sam validate --lint

deploy-stack: validate-stack
    sam deploy --no-confirm-changeset
    
stack-output:
    sam list stack-outputs --output json 

oidc-api-endpoint:
    sam list stack-outputs --output json | jq -r '.[] | select(.OutputKey == "OidcAuthorizerProtectedApiEndpoint") | .OutputValue'

python-oidc-api-endpoint:
    sam list stack-outputs --output json | jq -r '.[] | select(.OutputKey == "PythonOidcAuthorizerProtectedApiEndpoint") | .OutputValue'

build-generate-targets:
    cargo build -p generate-targets --release

run: build-generate-targets
    cargo run -p generate-targets --release -- --target-url $(just oidc-api-endpoint) --iterations 10000 | vegeta attack -lazy -rate=100/s -duration=100s | vegeta encode

run-python: build-generate-targets
    cargo run -p generate-targets --release -- --target-url $(just python-oidc-api-endpoint) --iterations 10000 | vegeta attack -lazy -rate=100/s -duration=100s | vegeta encode