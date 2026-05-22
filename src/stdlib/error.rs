use crate::runtime::{NaveRuntime, Value};
use std::collections::HashMap;

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("Error", |args| async move {
        let message = args.get(0).and_then(|v| v.as_str()).unwrap_or("Error");
        let mut obj = HashMap::new();
        obj.insert("message".to_string(), Value::String(message.to_string()));
        obj.insert("name".to_string(), Value::String("Error".to_string()));
        Value::Object(obj)
    });

    rt.register_async("TypeError", |args| async move {
        let message = args.get(0).and_then(|v| v.as_str()).unwrap_or("TypeError");
        let mut obj = HashMap::new();
        obj.insert("message".to_string(), Value::String(message.to_string()));
        obj.insert("name".to_string(), Value::String("TypeError".to_string()));
        Value::Object(obj)
    });
}
