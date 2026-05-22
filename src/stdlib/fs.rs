use std::fs;
use crate::runtime::{NaveRuntime, Value};

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("fs_read", |args| async move {
        let path = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        match fs::read_to_string(path) {
            Ok(s) => Value::String(s),
            Err(e) => Value::String(format!("Error: {}", e)),
        }
    });

    rt.register_async("fs_write", |args| async move {
        let path = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let content = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        match fs::write(path, content) {
            Ok(_) => Value::Bool(true),
            Err(_) => Value::Bool(false),
        }
    });

    rt.register_async("fs_exists", |args| async move {
        let path = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        Value::Bool(fs::metadata(path).is_ok())
    });

    rt.register_async("fs_remove", |args| async move {
        let path = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        match fs::remove_file(path) {
            Ok(_) => Value::Bool(true),
            Err(_) => Value::Bool(false),
        }
    });
}
