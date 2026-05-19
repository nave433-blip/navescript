use crate::ir::{NSIr, Instruction, Ownership};
use anyhow::{Context, Result};
use std::fs;

pub fn compile(ir: &NSIr) -> Result<Vec<u8>> {
    let mut wat = String::from("(component\n");
    wat.push_str(&format!("  ;; Nλvescript Component: {}\n", ir.module_name));
    wat.push_str(&format!("  ;; Targeted World: {}\n", ir.world));
    
    // WASI Preview 2 Imports (WIT-style)
    wat.push_str("  (import \"wasi:filesystem/types@0.2.0\" (instance $fs-types))\n");
    wat.push_str("  (import \"wasi:cli/stdout@0.2.0\" (instance $stdout))\n");
    
    // Core Module Wrap
    wat.push_str("  (core module $base\n");
    wat.push_str("    (import \"wasi:cli/stdout@0.2.0\" \"get-stdout\" (func $get-stdout (result i32)))\n");
    wat.push_str("    (memory (export \"memory\") 1)\n");
    wat.push_str("    (func (export \"run\")\n");
    
    for instr in &ir.body {
        match instr {
            Instruction::Log(msg) => {
                wat.push_str(&format!("      ;; log: {}\n", msg));
            }
            Instruction::Call { func, .. } => {
                wat.push_str(&format!("      ;; call: {}\n", func));
            }
            Instruction::Try { retry_count, .. } => {
                wat.push_str(&format!("      ;; try-block (retries: {})\n", retry_count));
            }
            Instruction::PolyglotEval { lang, .. } => {
                wat.push_str(&format!("      ;; polyglot eval: {}\n", lang));
            }
            Instruction::Answer { var } => {
                wat.push_str(&format!("      ;; answer var: {}\n", var));
            }
            Instruction::SetVar { var, .. } => {
                wat.push_str(&format!("      ;; set var: {}\n", var));
            }
            Instruction::NativeOp { op, return_var, .. } => {
                wat.push_str(&format!("      ;; native op: {} -> {}\n", op, return_var));
            }
            Instruction::AssertEq { left_var, right_var, .. } => {
                wat.push_str(&format!("      ;; assert: {} == {}\n", left_var, right_var));
            }
        }
    }
    
    wat.push_str("    )\n");
    wat.push_str("  )\n");

    // Component Lift/Lowering
    wat.push_str("  (core instance $i (instantiate $base))\n");
    wat.push_str("  (export \"run\" (func (canon lift (core func $i \"run\"))))\n");
    
    wat.push_str(")");

    Ok(wat.into_bytes())
}

/// Dynamic WIT Generator with full ownership models
pub fn generate_wit(program: &NSIr, world_name: &str) -> Result<String> {
    let mut wit = format!("package nave:{};\n\n", program.module_name);

    // Interfaces
    wit.push_str("interface logging {\n    log: func(level: string, message: string);\n    error: func(message: string, code: u32);\n}\n\n");

    wit.push_str("interface error-handling {\n");
    wit.push_str("    resource error {\n");
    wit.push_str("        message: func() -> string;\n");
    wit.push_str("        code: func() -> u32;\n");
    wit.push_str("        drop: func();  // Explicit ownership drop\n");
    wit.push_str("    }\n");
    wit.push_str("    try-catch: func<T>(body: func() -> result<T, borrow<error>>) -> result<T, borrow<error>>;\n");
    wit.push_str("    new-error: func(message: string, code: u32) -> own<error>;\n");
    wit.push_str("}\n\n");

    // Hardware resources with ownership
    wit.push_str("interface resources {\n");
    for res in &program.resources {
        match res.ownership {
            Ownership::Owned => {
                wit.push_str(&format!("    resource {} {{\n", res.name));
                wit.push_str("        // Owned resource - caller must drop\n");
                for method in &res.methods {
                    wit.push_str(&format!("        {}: func({}) -> {};\n", 
                        method.name, method.params, method.return_type));
                }
                wit.push_str("        drop: func();\n");
                wit.push_str("    }\n");
            }
            Ownership::Borrowed | Ownership::Shared => {
                wit.push_str(&format!("    resource {} {{\n        // Borrowed/Shared - no transfer\n", res.name));
                for method in &res.methods {
                    wit.push_str(&format!("        {}: func({}) -> {};\n", 
                        method.name, method.params, method.return_type));
                }
                wit.push_str("    }\n");
            }
        }
    }
    wit.push_str("    query-capability: func(name: string) -> option<string>;\n");
    wit.push_str("}\n\n");

    // Main world
    wit.push_str(&format!("world {} {{\n", world_name));
    wit.push_str("    export logging;\n");
    wit.push_str("    export error-handling;\n");
    wit.push_str("    export resources;\n\n");

    // Task resource (core of Nλvescript)
    wit.push_str("    resource task {\n");
    wit.push_str("        run: func() -> result<string, own<error>>;\n");
    wit.push_str("    }\n");

    // WASI 0.2 imports
    wit.push_str("    import wasi:cli/environment@0.2.0;\n");
    wit.push_str("    import wasi:filesystem/types@0.2.0;\n");
    wit.push_str("    import wasi:filesystem/preopens@0.2.0;\n");

    wit.push_str("}\n");

    Ok(wit)
}

/// High-level entry point: Compile + Generate WIT
pub fn full_build(source: &str, output_base: &str) -> Result<()> {
    let program = crate::parser::parse(source)
        .context("Failed to parse Nλvescript")?;
    
    let ir = NSIr::from_program(&program);

    // 1. Generate WIT dynamically
    let wit = generate_wit(&ir, "navescript-world")?;
    fs::write(format!("{}.wit", output_base), &wit)
        .context("Failed to write .wit file")?;
    println!("✅ Dynamic WIT generated → {}.wit", output_base);

    // 2. Compile to Component WAT
    let wat = compile(&ir)?;
    fs::write(format!("{}.wat", output_base), &wat)?;

    println!("🚀 Full build complete with ownership-aware WIT → {}.wat", output_base);
    Ok(())
}
