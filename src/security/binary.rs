use crate::runtime::{NaveRuntime, Value};
use goblin::{elf::Elf, pe::PE, mach::Mach};
use capstone::prelude::*;
use std::fs;
use std::collections::HashMap;

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("Binary_parse", |args| async move {
        let path = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let format = args.get(1).and_then(|v| v.as_str()).unwrap_or("auto");
        
        let buffer = match fs::read(path) {
            Ok(b) => b,
            Err(e) => return Value::String(format!("Error: {}", e)),
        };

        let mut res = HashMap::new();
        
        match format {
            "elf" => {
                if let Ok(elf) = Elf::parse(&buffer) {
                    res.insert("entry_point".to_string(), Value::Number(elf.entry as f64));
                    res.insert("arch".to_string(), Value::String(format!("{:?}", elf.header.e_machine)));
                    res.insert("is_64".to_string(), Value::Bool(elf.is_64));
                }
            }
            "pe" => {
                if let Ok(pe) = PE::parse(&buffer) {
                    res.insert("entry_point".to_string(), Value::Number(pe.entry as f64));
                    res.insert("is_64".to_string(), Value::Bool(pe.is_64));
                }
            }
            "macho" => {
                if let Ok(macho) = Mach::parse(&buffer) {
                    let arch_count = match macho {
                        Mach::Binary(_) => 1,
                        Mach::Fat(f) => f.arches().unwrap_or_default().len(),
                    };
                    res.insert("is_64".to_string(), Value::Bool(true));
                    res.insert("status".to_string(), Value::String(format!("Mach-O with {} architectures", arch_count)));
                }
            }
            _ => {
                res.insert("status".to_string(), Value::String("Unknown or multi-arch".to_string()));
            }
        }

        Value::Object(res)
    });

    rt.register_async("Binary_disassemble", |args| async move {
        let path = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let arch_str = args.get(1).and_then(|v| v.as_str()).unwrap_or("x86");
        
        let buffer = fs::read(path).unwrap_or_default();
        
        let cs = match arch_str {
            "arm64" => Capstone::new().arm64().mode(arch::arm64::ArchMode::Arm).build(),
            _ => Capstone::new().x86().mode(arch::x86::ArchMode::Mode64).build(),
        }.unwrap();

        let insns = cs.disasm_all(&buffer, 0x1000).unwrap();
        let mut instructions = Vec::new();
        for i in insns.iter().take(100) { // Limit for demo
            let mut instr_obj = HashMap::new();
            instr_obj.insert("address".to_string(), Value::Number(i.address() as f64));
            instr_obj.insert("mnemonic".to_string(), Value::String(i.mnemonic().unwrap_or("").to_string()));
            instr_obj.insert("operands".to_string(), Value::String(i.op_str().unwrap_or("").to_string()));
            instructions.push(Value::Object(instr_obj));
        }

        Value::Array(instructions)
    });
}
