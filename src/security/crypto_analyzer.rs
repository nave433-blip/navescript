use crate::runtime::{NaveRuntime, Value};
use std::collections::HashMap;

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("Crypto_test_entropy", |args| async move {
        let data = args.get(0).and_then(|v| if let Value::Buffer(b) = v { Some(b.clone()) } else { None }).unwrap_or_default();
        if data.is_empty() { return Value::Number(0.0); }

        let mut counts = HashMap::new();
        for &byte in &data {
            *counts.entry(byte).or_insert(0) += 1;
        }

        let mut entropy = 0.0;
        let len = data.len() as f64;
        for &count in counts.values() {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }

        Value::Number(entropy)
    });

    rt.register_async("Crypto_is_weak_key", |args| async move {
        let key_hex = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        // Placeholder logic: check for short keys
        Value::Bool(key_hex.len() < 32)
    });
}
