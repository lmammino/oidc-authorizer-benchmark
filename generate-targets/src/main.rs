use chrono::{Duration, Utc};
use clap::Parser;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rand::prelude::*;
use serde_json::json;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "10000")]
    iterations: usize,
    #[arg(short, long)]
    target_url: String,
}

fn main() {
    let args = Args::parse();
    let mut rng = rand::thread_rng();

    let keys = [
        EncodingKey::from_rsa_pem(include_bytes!("../../keys/key0/private.pem")).unwrap(),
        EncodingKey::from_rsa_pem(include_bytes!("../../keys/key1/private.pem")).unwrap(),
        EncodingKey::from_rsa_pem(include_bytes!("../../keys/key2/private.pem")).unwrap(),
        EncodingKey::from_rsa_pem(include_bytes!("../../keys/key3/private.pem")).unwrap(),
    ];
    let keys = keys.iter().cycle();
    let seq = (0..args.iterations).zip(keys);

    seq.for_each(|(i, key)| {
        let kid = format!("key{}", i % 4);
        let token_header: Header =
            serde_json::from_value(json!({"alg": Algorithm::RS512, "kid": kid})).unwrap();
        let exp = (Utc::now() + Duration::try_hours(1).unwrap()).timestamp();

        let token_claims = json!({
            "jti": rng.gen::<u64>(), // adds a random token id to make it unlikely to generate the same token and avoid caching
            "aud": "oidc-authorizer-benchmark",
            "iss": "oidc-authorizer-benchmark",
            "sub": "oidc-benchmark-test-user",
            "exp": exp
        });
        let token = encode(&token_header, &token_claims, key).unwrap();

        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));

        println!("GET {}\nAuthorization: Bearer {}\n", args.target_url, token);
    });
}
