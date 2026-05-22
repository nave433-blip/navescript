use sha2::{Sha256, Sha512, Digest};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};

pub struct CryptoHost;

impl CryptoHost {
    pub fn new() -> Self {
        CryptoHost {}
    }

    pub fn sha256(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    pub fn sha512(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    pub fn argon2id(&self, password: &str, salt: &[u8]) -> Result<Vec<u8>, String> {
        let salt = SaltString::encode_b64(salt).map_err(|e| e.to_string())?;
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| e.to_string())?
            .to_string();
        Ok(hash.into_bytes())
    }

    pub fn ed25519_sign(&self, private_key: &[u8], message: &[u8]) -> Vec<u8> {
        let signing_key = SigningKey::from_bytes(private_key.try_into().unwrap());
        signing_key.sign(message).to_vec()
    }

    pub fn ed25519_verify(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        let verifying_key = VerifyingKey::from_bytes(public_key.try_into().unwrap()).unwrap();
        let sig = Signature::from_bytes(signature.try_into().unwrap());
        verifying_key.verify(message, &sig).is_ok()
    }

    pub fn ed25519_generate_keypair(&self) -> (Vec<u8>, Vec<u8>) {
        let mut csprng = rand::rngs::OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        (signing_key.to_bytes().to_vec(), signing_key.verifying_key().to_bytes().to_vec())
    }
}
