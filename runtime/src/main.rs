// navescript/runtime/src/main.rs (Updated)
mod jit;
mod loader;
mod nasi;
mod gc;

use loader::load_nas_module;
use jit::{JitEngine, LINEAR_MEMORY_SIZE};
use crate::gc::SimpleAllocator;
use std::ptr;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: nas-run <file.nas>");
        return;
    }

    // 1. Load the .nas module
    let mut module = load_nas_module(&args[1]).expect("Failed to load module");

    // 2. Instantiate JIT Engine and compile module
    let mut jit = JitEngine::new();
    let mut executable = jit.compile(&mut module).expect("JIT compilation failed");

    // 3. Prepare allocators and memory for NASI calls
    // Note: executable.allocator is already initialized during compile,
    // and passed to NASI calls from the JIT-compiled code.

    // 4. Execute
    println!("Executing module...");
    let result = unsafe {
        executable.run()
    };

    println!("Execution finished with result: {}", result);
}
