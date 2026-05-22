use crate::runtime::{NaveRuntime, Value};
use std::fs::OpenOptions;
use std::io::Write;

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("Audit_log", |args| async move {
        let op = args.get(0).and_then(|v| v.as_str()).unwrap_or("unknown_op");
        let target = args.get(1).and_then(|v| v.as_str()).unwrap_or("unknown_target");
        
        let log_entry = format!("[Audit] OP: {}, TARGET: {}\n", op, target);
        
        // Log to a file (safeguard mandate)
        if let Ok(mut file) = OpenOptions::new().append(true).create(true).open("security_audit.log") {
            let _ = file.write_all(log_entry.as_bytes());
        }
        
        println!("{}", log_entry.trim());
        Value::Bool(true)
    });
}
