// navescript/src/nasm.rs
use crate::ir::Instruction;
use anyhow::Result;

pub struct Assembler;

impl Assembler {
    // Translates NASM-style instructions into WASM bytecode
    // For this POC, we generate WebAssembly Text (WAT) instructions
    pub fn assemble(instructions: &[Instruction]) -> Result<String> {
        let mut wat = String::new();
        for instr in instructions {
            match instr {
                Instruction::LOAD { reg, addr } => {
                    wat.push_str(&format!("    local.get ${}\n    i32.const {}\n    i32.load\n    local.set ${}\n", addr, addr, reg));
                }
                Instruction::STORE { reg, addr } => {
                    wat.push_str(&format!("    local.get ${}\n    i32.const {}\n    i32.store\n", reg, addr));
                }
                Instruction::ADD { dest, src1, src2 } => {
                    wat.push_str(&format!("    local.get ${}\n    local.get ${}\n    i32.add\n    local.set ${}\n", src1, src2, dest));
                }
                Instruction::RET { reg } => {
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
            Instruction::LOAD { reg: 0, addr: 100 },
            Instruction::LOAD { reg: 1, addr: 104 },
            Instruction::ADD { dest: 2, src1: 0, src2: 1 },
            Instruction::RET { reg: 2 },
        ];

        let wat = Assembler::assemble(&instructions).unwrap();
        assert!(wat.contains("local.get $100"));
        assert!(wat.contains("i32.load"));
        assert!(wat.contains("local.set $0"));
        assert!(wat.contains("i32.add"));
        assert!(wat.contains("return"));
    }
}
