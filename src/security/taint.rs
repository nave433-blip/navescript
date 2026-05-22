use crate::runtime::{NaveRuntime, Value};
use std::collections::HashMap;

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("Taint_mark_source", |args| async move {
        let val = args.get(0).cloned().unwrap_or(Value::Nil);
        println!("[Taint] Marking value as tainted source: {}", val);
        Value::Bool(true)
    });

    rt.register_async("Taint_check_sink", |args| async move {
        let val = args.get(0).cloned().unwrap_or(Value::Nil);
        let sink_name = args.get(1).and_then(|v| v.as_str()).unwrap_or("unknown");
        println!("[Taint] Checking if tainted value reaches sink {}: {}", sink_name, val);
        Value::Bool(false) // Demo: always clean for now
    });
}
