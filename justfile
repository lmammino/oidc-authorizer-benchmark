validate-stack:
    sam validate --lint

deploy-stack: validate-stack
    sam deploy --no-confirm-changeset
    
stack-output:
    sam list stack-outputs --output json 

stack-output-api-endpoint:
    sam list stack-outputs --output json | jq -r .[1].OutputValue

build-generate-targets:
    cargo build -p generate-targets --release

run: build-generate-targets
    cargo run -p generate-targets --release -- --target-url $(just stack-output-api-endpoint) --iterations 10000 | vegeta attack -lazy -rate=100/s -duration=100s | vegeta encode