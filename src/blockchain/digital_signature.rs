use ed25519_dalek::{Signature, SignatureError, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng; 

pub fn gen_key() -> (SigningKey, VerifyingKey) {
    let priv_key = SigningKey::generate(&mut OsRng);
    let pub_key = priv_key.verifying_key();

    (priv_key, pub_key)
}

pub fn sign_str(data : &String, key : &SigningKey) -> Signature {
    let signature: Signature = key.sign(data.as_bytes());

    signature
}

pub fn verify_str(data : &String, signature : &Signature, key : &VerifyingKey) -> Result<(), SignatureError> {
    key.verify(data.as_bytes(), signature)
}