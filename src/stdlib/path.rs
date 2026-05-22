// navescript/src/stdlib/path.rs
use crate::runtime::NaveRuntime;
use crate::runtime::Value;
use std::path::Path;

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("path_join", |args| async move {
        let joined: Vec<String> = args.iter().map(|v| v.to_string()).collect();
        let path = joined.join(std::path::MAIN_SEPARATOR_STR);
        Value::String(path)
    });
    
    rt.register_async("path_basename", |args| async move {
        if let Some(path) = args.get(0).and_then(|v| v.as_str()) {
            let b = Path::new(path).file_name().and_then(|s| s.to_str()).unwrap_or("");
            return Value::String(b.to_string());
        }
        Value::Nil
    });

    rt.register_async("path_dirname", |args| async move {
        if let Some(path) = args.get(0).and_then(|v| v.as_str()) {
            let d = Path::new(path).parent().and_then(|s| s.to_str()).unwrap_or(".");
            return Value::String(d.to_string());
        }
        Value::Nil
    });
}
