use crate::runtime::{NaveRuntime, Value};
use sha2::{Sha224, Sha256, Sha384, Sha512, Digest};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
use rand::RngCore;
use hex;

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("crypto_sha256", |args| async move {
        let data = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let mut hasher = Sha256::new();
        hasher.update(data);
        Value::String(format!("{:x}", hasher.finalize()))
    });

    rt.register_async("crypto_sha512", |args| async move {
        let data = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let mut hasher = Sha512::new();
        hasher.update(data);
        Value::String(format!("{:x}", hasher.finalize()))
    });

    rt.register_async("crypto_sha3_256", |args| async move {
        let data = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        Value::String(format!("{:x}", hasher.finalize()))
    });

    rt.register_async("crypto_argon2id", |args| async move {
        let password = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let salt_str = args.get(1).and_then(|v| v.as_str()).unwrap_or("defaultsalt1234");
        let salt = SaltString::encode_b64(salt_str.as_bytes()).unwrap();
        let argon2 = Argon2::default();
        match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(h) => Value::String(h.to_string()),
            Err(e) => Value::String(format!("Error: {}", e)),
        }
    });

    rt.register_async("crypto_ed25519_sign", |args| async move {
        let priv_key_hex = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let message = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        
        let priv_key_bytes = match hex::decode(priv_key_hex) {
            Ok(b) => b,
            Err(_) => return Value::Nil,
        };
        
        let signing_key = SigningKey::from_bytes(priv_key_bytes.as_slice().try_into().unwrap());
        let signature = signing_key.sign(message.as_bytes());
        Value::String(hex::encode(signature.to_bytes()))
    });

    rt.register_async("crypto_random_bytes", |args| async move {
        let len = args.get(0).and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(32);
        let mut data = vec![0u8; len];
        rand::thread_rng().fill_bytes(&mut data);
        Value::Buffer(data)
    });

    rt.register_async("crypto_rsa_encrypt", |args| async move {
        // Simplified: generates a key on the fly for demo
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);
        let data = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, data.as_bytes()).expect("failed to encrypt");
        Value::Buffer(enc_data)
    });
}
