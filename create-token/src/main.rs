use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde_json::json;

fn main() {
    let private_key = include_str!("../../keys/key0/private.pem");
    let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap();
    let token_header: Header =
        serde_json::from_value(json!({"alg": Algorithm::RS512, "kid": "key0"})).unwrap();
    let exp = (Utc::now() + Duration::try_hours(1).unwrap()).timestamp();

    let token_claims = json!({
        "aud": "oidc-authorizer-benchmark",
        "iss": "oidc-authorizer-benchmark",
        "sub": "oidc-benchmark-test-user",
        "exp": exp
    });
    let token = encode(&token_header, &token_claims, &encoding_key).unwrap();
    println!("{}", token);
}
