use crate::runtime::{NaveRuntime, Value};
use std::collections::HashMap;

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("Forensics_heap_analyze", |args| async move {
        let pid = args.get(0).and_then(|v| if let Value::Number(n) = v { Some(*n as u32) } else { None }).unwrap_or(0);
        println!("[Forensics] Analyzing heap for PID {}", pid);
        
        let mut chunks = Vec::new();
        // Mock heap chunks
        for i in 0..5 {
            let mut chunk = HashMap::new();
            chunk.insert("address".to_string(), Value::Number((0x1000 + i * 0x100) as f64));
            chunk.insert("size".to_string(), Value::Number(256.0));
            chunk.insert("status".to_string(), Value::String(if i % 2 == 0 { "allocated" } else { "free" }.to_string()));
            chunks.push(Value::Object(chunk));
        }
        Value::Array(chunks)
    });

    rt.register_async("Forensics_stack_trace", |args| async move {
        let pid = args.get(0).and_then(|v| if let Value::Number(n) = v { Some(*n as u32) } else { None }).unwrap_or(0);
        println!("[Forensics] Tracing stack for PID {}", pid);
        
        let mut frames = Vec::new();
        for i in 0..3 {
            let mut frame = HashMap::new();
            frame.insert("level".to_string(), Value::Number(i as f64));
            frame.insert("function".to_string(), Value::String(format!("func_{}", i)));
            frame.insert("return_addr".to_string(), Value::Number((0x4000 + i * 0x20) as f64));
            frames.push(Value::Object(frame));
        }
        Value::Array(frames)
    });
}
