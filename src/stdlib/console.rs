// navescript/src/stdlib/console.rs
use crate::runtime::NaveRuntime;
use crate::runtime::Value;
use std::io::{self, Write};

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("console_log", |args| async move {
        println!("[Nλve] {}", args.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" "));
        Value::Nil
    });
    
    rt.register_async("console_error", |args| async move {
        eprintln!("[Nλve Error] {}", args.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" "));
        Value::Nil
    });
    
    rt.register_async("console_clear", |_args| async move {
        print!("{}[2J", 27 as char);
        io::stdout().flush().unwrap();
        Value::Nil
    });
    
    // Add other console methods: time, timeEnd, trace, assert...
}
