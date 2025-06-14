use ed25519_dalek::{
    SigningKey,
    pkcs8::spki::der::pem::LineEnding,
    pkcs8::{EncodePrivateKey, EncodePublicKey},
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::{
    io::Write,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iat: u64,
    exp: u64,
    payload: Option<String>,
}
impl Claims {
    fn new(d: Duration) -> Claims {
        Claims {
            iat: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            exp: (SystemTime::now() + d)
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            payload: None,
        }
    }

    fn new_with_payload(d: Duration, p: String) -> Claims {
        let claim = Claims::new(d);
        Claims {
            payload: Some(p),
            ..claim
        }
    }
}

fn main() {
    let prompt = String::from("Enter a string to use as your payload: ");
    let mut payload = String::new();
    print!("{prompt}");
    std::io::stdout().flush().unwrap(); // Print prompt before waiting for input
    std::io::stdin()
        .read_line(&mut payload)
        .expect("Failed to read line");

    let claims = Claims::new_with_payload(
        Duration::from_secs(60 * 60 * 24),
        String::from(payload.trim()),
    );

    let key = new_key();
    let key_bytes = key
        .to_pkcs8_der()
        .expect("Newly-generated key should be valid")
        .to_bytes();
    let token = encode(
        &Header::new(Algorithm::EdDSA),
        &claims,
        &EncodingKey::from_ed_der(&key_bytes),
    )
    .unwrap();

    let pub_key_bytes = key.verifying_key().to_bytes();
    let token_data = decode::<Claims>(
        token.as_str(),
        &DecodingKey::from_ed_der(&pub_key_bytes),
        &Validation::new(Algorithm::EdDSA),
    )
    .unwrap();

    println!("\nJWT: {}", token);
    println!("{:?}", token_data.claims);
    println!("\nSignature can be verified with the following key:");
    let key_string = key
        .verifying_key()
        .to_public_key_pem(LineEnding::LF)
        .expect("Newly-generated key should be valid");
    println!("{key_string}")
}

fn new_key() -> SigningKey {
    let mut generator = OsRng;
    SigningKey::generate(&mut generator)
}
