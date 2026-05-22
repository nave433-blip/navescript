use crate::runtime::{NaveRuntime, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct DebuggerHost {
    breakpoints: Arc<Mutex<HashSet<u64>>>,
    watchpoints: Arc<Mutex<HashMap<String, Value>>>,
}

impl DebuggerHost {
    pub fn new() -> Self {
        Self {
            breakpoints: Arc::new(Mutex::new(HashSet::new())),
            watchpoints: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("Debugger_breakpoint", |args| async move {
        let addr = args.get(0).and_then(|v| if let Value::Number(n) = v { Some(*n as u64) } else { None }).unwrap_or(0);
        println!("[Debugger] Breakpoint set at 0x{:x}", addr);
        Value::Bool(true)
    });

    rt.register_async("Debugger_watch", |args| async move {
        let name = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        println!("[Debugger] Watching register/variable: {}", name);
        Value::Bool(true)
    });

    rt.register_async("Debugger_step", |_| async move {
        println!("[Debugger] Single step");
        Value::Bool(true)
    });
}

use std::collections::HashSet;
