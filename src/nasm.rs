// navescript/src/nasm.rs
use crate::ir::Instruction;
use anyhow::{Result, anyhow};

pub struct Assembler;

impl Assembler {
    pub fn parse(code: &str) -> Result<Vec<Instruction>> {
        let mut instructions = Vec::new();
        for line in code.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with("/*") || line.contains("module") || line.contains("func") || line.contains("export") || line == "}" {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().map(|s| s.trim_matches(',')).collect();
            if parts.is_empty() { continue; }

            match parts[0] {
                "LOAD" => {
                    let reg = parts[1].strip_prefix('R').and_then(|s| s.parse().ok()).ok_or_else(|| anyhow!("Invalid register"))?;
                    let addr = parts[2].parse().ok().ok_or_else(|| anyhow!("Invalid address"))?;
                    instructions.push(Instruction::Load { reg, addr });
                }
                "LI" => {
                    let reg = parts[1].strip_prefix('R').and_then(|s| s.parse().ok()).ok_or_else(|| anyhow!("Invalid register"))?;
                    let value = parts[2].parse().ok().ok_or_else(|| anyhow!("Invalid value"))?;
                    instructions.push(Instruction::LoadImm { reg, value });
                }
                "STORE" => {
                    let reg = parts[1].strip_prefix('R').and_then(|s| s.parse().ok()).ok_or_else(|| anyhow!("Invalid register"))?;
                    let addr = parts[2].parse().ok().ok_or_else(|| anyhow!("Invalid address"))?;
                    instructions.push(Instruction::Store { reg, addr });
                }
                "ADD" => {
                    let dest = parts[1].strip_prefix('R').and_then(|s| s.parse().ok()).ok_or_else(|| anyhow!("Invalid register"))?;
                    let (src1, src2) = if parts.len() >= 4 {
                        let s1 = parts[2].strip_prefix('R').and_then(|s| s.parse().ok()).ok_or_else(|| anyhow!("Invalid src1 register"))?;
                        let s2 = parts[3].strip_prefix('R').and_then(|s| s.parse().ok()).ok_or_else(|| anyhow!("Invalid src2 register"))?;
                        (s1, s2)
                    } else {
                        let s2 = parts[2].strip_prefix('R').and_then(|s| s.parse().ok()).ok_or_else(|| anyhow!("Invalid src2 register"))?;
                        (dest, s2)
                    };
                    instructions.push(Instruction::Add { dest, src1, src2 });
                }
                "RET" => {
                    let reg = parts[1].strip_prefix('R').and_then(|s| s.parse().ok()).ok_or_else(|| anyhow!("Invalid register"))?;
                    instructions.push(Instruction::Ret { reg });
                }
                "SYSCALL" => {
                    let id = parts[1].parse().ok().ok_or_else(|| anyhow!("Invalid syscall ID"))?;
                    let mut args = Vec::new();
                    if parts.len() > 2 {
                        // Very simplified arg parsing: everything else as a string-to-bytes
                        let msg = parts[2..].join(" ");
                        args = msg.into_bytes();
                    }
                    instructions.push(Instruction::Syscall { id, args });
                }
                _ => {}
            }
        }
        Ok(instructions)
    }

    // Translates NASM-style instructions into WASM bytecode
    // For this POC, we generate WebAssembly Text (WAT) instructions
    pub fn assemble(instructions: &[Instruction]) -> Result<String> {
        let mut wat = String::new();
        for instr in instructions {
            match instr {
                Instruction::Load { reg, addr } => {
                    wat.push_str(&format!("    local.get ${}\n    i32.const {}\n    i32.load\n    local.set ${}\n", addr, addr, reg));
                }
                Instruction::Store { reg, addr } => {
                    wat.push_str(&format!("    local.get ${}\n    i32.const {}\n    i32.store\n", reg, addr));
                }
                Instruction::Add { dest, src1, src2 } => {
                    wat.push_str(&format!("    local.get ${}\n    local.get ${}\n    i32.add\n    local.set ${}\n", src1, src2, dest));
                }
                Instruction::Ret { reg } => {
                    wat.push_str(&format!("    local.get ${}\n    return\n", reg));
                }
                _ => {}
            }
        }
        Ok(wat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble_basic() {
        let instructions = vec![
            Instruction::Load { reg: 0, addr: 100 },
            Instruction::Load { reg: 1, addr: 104 },
            Instruction::Add { dest: 2, src1: 0, src2: 1 },
            Instruction::Ret { reg: 2 },
        ];

        let wat = Assembler::assemble(&instructions).unwrap();
        assert!(wat.contains("local.get $100"));
        assert!(wat.contains("i32.load"));
        assert!(wat.contains("local.set $0"));
        assert!(wat.contains("i32.add"));
        assert!(wat.contains("return"));
    }
}
