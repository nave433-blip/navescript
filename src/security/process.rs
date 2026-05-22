use crate::runtime::{NaveRuntime, Value};
use sysinfo::{System, Pid};
use read_process_memory::{Pid as NativePid, copy_address, ProcessHandle};
use std::collections::HashMap;

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("Process_open", |args| async move {
        let pid_val = args.get(0).and_then(|v| if let Value::Number(n) = v { Some(*n as u32) } else { None }).unwrap_or(0);
        let mut sys = System::new_all();
        sys.refresh_all();
        
        if let Some(process) = sys.process(Pid::from_u32(pid_val)) {
            let mut res = HashMap::new();
            res.insert("name".to_string(), Value::String(process.name().to_string_lossy().to_string()));
            res.insert("memory".to_string(), Value::Number(process.memory() as f64));
            res.insert("status".to_string(), Value::String(format!("{:?}", process.status())));
            Value::Object(res)
        } else {
            Value::Nil
        }
    });

    rt.register_async("Process_read", |args| async move {
        let pid_val = args.get(0).and_then(|v| if let Value::Number(n) = v { Some(*n as u32) } else { None }).unwrap_or(0);
        let addr = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(0);
        let size = args.get(2).and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(1024);
        
        let pid = pid_val as NativePid;
        let handle = match ProcessHandle::try_from(pid) {
            Ok(h) => h,
            Err(_) => return Value::String("Failed to open process handle".to_string()),
        };

        match copy_address(addr, size, &handle) {
            Ok(data) => Value::Buffer(data),
            Err(e) => Value::String(format!("Error: {}", e)),
        }
    });
}
