use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rand::prelude::*;
use serde_json::json;
use std::sync::atomic;

pub struct TokenGen {
    keys: Vec<EncodingKey>,
    generated_keys: atomic::AtomicUsize,
}

impl Default for TokenGen {
    fn default() -> Self {
        let keys = [
            EncodingKey::from_rsa_pem(include_bytes!("../../keys/key0/private.pem")).unwrap(),
            EncodingKey::from_rsa_pem(include_bytes!("../../keys/key1/private.pem")).unwrap(),
            EncodingKey::from_rsa_pem(include_bytes!("../../keys/key2/private.pem")).unwrap(),
            EncodingKey::from_rsa_pem(include_bytes!("../../keys/key3/private.pem")).unwrap(),
        ];
        TokenGen {
            keys: keys.to_vec(),
            generated_keys: atomic::AtomicUsize::new(0),
        }
    }
}

impl TokenGen {
    pub fn next_token(&self) -> String {
        let i: usize = self.generated_keys.fetch_add(1, atomic::Ordering::SeqCst);
        let kid = format!("key{}", i % 4);
        let key = &self.keys[i % self.keys.len()];

        let token_header: Header =
            serde_json::from_value(json!({"alg": Algorithm::RS512, "kid": kid})).unwrap();
        let exp = (Utc::now() + Duration::try_hours(1).unwrap()).timestamp();

        let jti: u64 = random();
        let token_claims = json!({
            "jti": jti, // adds a random token id to make it unlikely to generate the same token and avoid caching
            "aud": "oidc-authorizer-benchmark",
            "iss": "oidc-authorizer-benchmark",
            "sub": "oidc-benchmark-test-user",
            "exp": exp
        });

        encode(&token_header, &token_claims, key).unwrap()
    }
}

impl Iterator for TokenGen {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_token())
    }
}
