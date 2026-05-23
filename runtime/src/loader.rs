// navescript/runtime/src/loader.rs (Expanded)

use std::fs;

// Represents a single NAS instruction.
#[derive(Debug)]
pub enum Instruction {
    LI(u8, i64),      // LI <reg>, <imm>
    ADD(u8, u8, u8),  // ADD <dest>, <src1>, <src2>
    SUB(u8, u8, u8),  // SUB <dest>, <src1>, <src2>
    MUL(u8, u8, u8),  // MUL <dest>, <src1>, <src2>
    DIV(u8, u8, u8),  // DIV <dest>, <src1>, <src2>
    LOAD(u8, String), // LOAD <reg>, <addr_label_or_offset>
    STORE(u8, String),// STORE <reg>, <addr_label_or_offset>
    JMP(String),      // JMP <label>
    JZ(u8, String),   // JZ <reg>, <label>
    CALL(String),     // CALL <label>
    RET,              // RET
    PUSH(u8),         // PUSH <reg>
    POP(u8),          // POP <reg>
    SYSCALL(u32, u64, u64, u64), // SYSCALL <id>, <arg1>, <arg2>, <arg3> - conceptual arguments
    LABEL(String),    // <label>:
    // ... other instructions
}

// Represents a loaded NAS module.
pub struct NasModule {
    pub instructions: Vec<Instruction>,
    // Store labels with their instruction indices for JIT backpatching
    pub labels: std::collections::HashMap<String, usize>,
}

// Parses a .nas file into a NasModule.
pub fn load_nas_module(path: &str) -> Result<NasModule, String> {
    let source = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut instructions = Vec::new();
    let mut labels = std::collections::HashMap::new();

    for (idx, line) in source.lines().enumerate() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with(';') {
            continue;
        }

        if let Some(label_name) = trimmed_line.strip_suffix(':') {
            labels.insert(label_name.to_string(), instructions.len());
            instructions.push(Instruction::LABEL(label_name.to_string()));
            continue;
        }

        let parts: Vec<&str> = trimmed_line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "LI" => {
                let reg = parts[1].trim_start_matches('R').parse().unwrap();
                let imm = parts[2].parse().unwrap();
                instructions.push(Instruction::LI(reg, imm));
            }
            "ADD" => {
                let dest = parts[1].trim_start_matches('R').parse().unwrap();
                let src1 = parts[2].trim_start_matches('R').parse().unwrap();
                let src2 = parts[3].trim_start_matches('R').parse().unwrap();
                instructions.push(Instruction::ADD(dest, src1, src2));
            }
            "SUB" => {
                let dest = parts[1].trim_start_matches('R').parse().unwrap();
                let src1 = parts[2].trim_start_matches('R').parse().unwrap();
                let src2 = parts[3].trim_start_matches('R').parse().unwrap();
                instructions.push(Instruction::SUB(dest, src1, src2));
            }
            "MUL" => {
                let dest = parts[1].trim_start_matches('R').parse().unwrap();
                let src1 = parts[2].trim_start_matches('R').parse().unwrap();
                let src2 = parts[3].trim_start_matches('R').parse().unwrap();
                instructions.push(Instruction::MUL(dest, src1, src2));
            }
            "DIV" => {
                let dest = parts[1].trim_start_matches('R').parse().unwrap();
                let src1 = parts[2].trim_start_matches('R').parse().unwrap();
                let src2 = parts[3].trim_start_matches('R').parse().unwrap();
                instructions.push(Instruction::DIV(dest, src1, src2));
            }
            "LOAD" => {
                let reg = parts[1].trim_start_matches('R').parse().unwrap();
                let addr_str = parts[2].to_string(); // "R15 + 8" or "label"
                instructions.push(Instruction::LOAD(reg, addr_str));
            }
            "STORE" => {
                let reg = parts[1].trim_start_matches('R').parse().unwrap();
                let addr_str = parts[2].to_string();
                instructions.push(Instruction::STORE(reg, addr_str));
            }
            "JMP" => {
                let label = parts[1].to_string();
                instructions.push(Instruction::JMP(label));
            }
            "JZ" => {
                let reg = parts[1].trim_start_matches('R').parse().unwrap();
                let label = parts[2].to_string();
                instructions.push(Instruction::JZ(reg, label));
            }
            "CALL" => {
                let label = parts[1].to_string();
                instructions.push(Instruction::CALL(label));
            }
            "RET" => {
                instructions.push(Instruction::RET);
            }
            "PUSH" => {
                let reg = parts[1].trim_start_matches('R').parse().unwrap();
                instructions.push(Instruction::PUSH(reg));
            }
            "POP" => {
                let reg = parts[1].trim_start_matches('R').parse().unwrap();
                instructions.push(Instruction::POP(reg));
            }
            "SYSCALL" => {
                let id = parts[1].parse().unwrap();
                let arg1 = parts.get(2).map_or(0, |s| s.parse().unwrap_or(0));
                let arg2 = parts.get(3).map_or(0, |s| s.parse().unwrap_or(0));
                let arg3 = parts.get(4).map_or(0, |s| s.parse().unwrap_or(0));
                instructions.push(Instruction::SYSCALL(id, arg1, arg2, arg3));
            }
            _ => {
                eprintln!("Unknown NAS instruction: {}", parts[0]);
            }
        }
    }

    Ok(NasModule { instructions, labels })
}
