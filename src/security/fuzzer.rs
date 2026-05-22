use crate::runtime::{NaveRuntime, Value};
use rand::Rng;

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("Fuzzer_mutate", |args| async move {
        let input = args.get(0).and_then(|v| if let Value::Buffer(b) = v { Some(b.clone()) } else { None }).unwrap_or_default();
        let mutation_type = args.get(1).and_then(|v| v.as_str()).unwrap_or("bitflip");
        
        let mut mutated = input.clone();
        if mutated.is_empty() { return Value::Buffer(mutated); }

        let mut rng = rand::thread_rng();
        match mutation_type {
            "bitflip" => {
                let idx = rng.gen_range(0..mutated.len());
                let bit = rng.gen_range(0..8);
                mutated[idx] ^= 1 << bit;
            }
            "byteswap" => {
                if mutated.len() > 1 {
                    let idx1 = rng.gen_range(0..mutated.len());
                    let idx2 = rng.gen_range(0..mutated.len());
                    mutated.swap(idx1, idx2);
                }
            }
            _ => {}
        }

        Value::Buffer(mutated)
    });
}
