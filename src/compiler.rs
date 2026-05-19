use crate::ir::NSIr;
use crate::cranelift_backend::CraneliftBackend;
use anyhow::{Context, Result};
use std::fs;

pub fn full_build(source: &str, output_base: &str) -> Result<()> {
    let program = crate::parser::parse(source)
        .context("Failed to parse Nλvescript")?;
    
    let ir = NSIr::from_program(&program);

    let mut backend = CraneliftBackend::new()?;
    backend.compile_ir("main", &ir)?;
    let binary = backend.finish()?;
    
    fs::write(format!("{}.o", output_base), &binary)
        .context("Failed to write native object file")?;

    let wat = compile_to_wat(&ir)?;
    fs::write(format!("{}.wat", output_base), &wat)?;

    println!("🚀 Full build complete: {}.o, {}.wat", output_base, output_base);
    Ok(())
}

pub fn compile_to_wat(ir: &NSIr) -> Result<String> {
    let mut wat = String::from("(component\n");
    wat.push_str(&format!("  ;; Nλvescript Component: {}\n", ir.module_name));
    wat.push_str(&format!("  ;; Targeted World: {}\n", ir.world));
    
    wat.push_str("  (import \"wasi:filesystem/types@0.2.0\" (instance $fs-types))\n");
    wat.push_str("  (import \"wasi:cli/stdout@0.2.0\" (instance $stdout))\n");
    
    wat.push_str("  (core module $base\n");
    wat.push_str("    (import \"wasi:cli/stdout@0.2.0\" \"get-stdout\" (func $get-stdout (result i32)))\n");
    wat.push_str("    (memory (export \"memory\") 1)\n");
    wat.push_str("    (func (export \"run\")\n");
    
    for instr in &ir.body {
        match instr {
            crate::ir::Instruction::Log(msg) => {
                wat.push_str(&format!("      ;; log: {}\n", msg));
            }
            crate::ir::Instruction::Call { func, .. } => {
                wat.push_str(&format!("      ;; call: {}\n", func));
            }
            crate::ir::Instruction::Try { retry_count, .. } => {
                wat.push_str(&format!("      ;; try-block (retries: {})\n", retry_count));
            }
            crate::ir::Instruction::PolyglotEval { lang, .. } => {
                wat.push_str(&format!("      ;; polyglot eval: {}\n", lang));
            }
            crate::ir::Instruction::Answer { var } => {
                wat.push_str(&format!("      ;; answer var: {}\n", var));
            }
            crate::ir::Instruction::SetVar { var, .. } => {
                wat.push_str(&format!("      ;; set var: {}\n", var));
            }
            crate::ir::Instruction::NativeOp { op, return_var, .. } => {
                wat.push_str(&format!("      ;; native op: {} -> {}\n", op, return_var));
            }
            crate::ir::Instruction::AssertEq { left_var, right_var, .. } => {
                wat.push_str(&format!("      ;; assert: {} == {}\n", left_var, right_var));
            }
            _ => {
                wat.push_str("      ;; nasm instruction\n");
            }
        }
    }
    
    wat.push_str("    )\n");
    wat.push_str("  )\n");

    wat.push_str("  (core instance $i (instantiate $base))\n");
    wat.push_str("  (export \"run\" (func (canon lift (core func $i \"run\"))))\n");
    
    wat.push_str(")");

    Ok(wat)
}
