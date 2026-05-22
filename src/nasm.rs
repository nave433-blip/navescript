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
                "LOAD" => instructions.push(Instruction::Load { reg: parse_reg(parts[1])?, addr: parts[2].parse()? }),
                "LI" => instructions.push(Instruction::LoadImm { reg: parse_reg(parts[1])?, value: parts[2].parse()? }),
                "STORE" => instructions.push(Instruction::Store { reg: parse_reg(parts[1])?, addr: parts[2].parse()? }),
                "ADD" => instructions.push(Instruction::Add { dest: parse_reg(parts[1])?, src1: parse_reg(parts[2])?, src2: parse_reg(parts[3])? }),
                "SUB" => instructions.push(Instruction::Sub { dest: parse_reg(parts[1])?, src1: parse_reg(parts[2])?, src2: parse_reg(parts[3])? }),
                "MUL" => instructions.push(Instruction::Mul { dest: parse_reg(parts[1])?, src1: parse_reg(parts[2])?, src2: parse_reg(parts[3])? }),
                "DIV" => instructions.push(Instruction::Div { dest: parse_reg(parts[1])?, src1: parse_reg(parts[2])?, src2: parse_reg(parts[3])? }),
                "MOD" => instructions.push(Instruction::Mod { dest: parse_reg(parts[1])?, src1: parse_reg(parts[2])?, src2: parse_reg(parts[3])? }),
                "AND" => instructions.push(Instruction::And { dest: parse_reg(parts[1])?, src1: parse_reg(parts[2])?, src2: parse_reg(parts[3])? }),
                "OR" => instructions.push(Instruction::Or { dest: parse_reg(parts[1])?, src1: parse_reg(parts[2])?, src2: parse_reg(parts[3])? }),
                "XOR" => instructions.push(Instruction::Xor { dest: parse_reg(parts[1])?, src1: parse_reg(parts[2])?, src2: parse_reg(parts[3])? }),
                "SHL" => instructions.push(Instruction::Shl { dest: parse_reg(parts[1])?, src1: parse_reg(parts[2])?, src2: parse_reg(parts[3])? }),
                "SHR" => instructions.push(Instruction::Shr { dest: parse_reg(parts[1])?, src1: parse_reg(parts[2])?, src2: parse_reg(parts[3])? }),
                "NOT" => instructions.push(Instruction::Not { dest: parse_reg(parts[1])?, src: parse_reg(parts[2])? }),
                "JMP" => instructions.push(Instruction::Jmp { label: parts[1].to_string() }),
                "JZ" => instructions.push(Instruction::Jz { reg: parse_reg(parts[1])?, label: parts[2].to_string() }),
                "JNZ" => instructions.push(Instruction::Jnz { reg: parse_reg(parts[1])?, label: parts[2].to_string() }),
                "CMP" => instructions.push(Instruction::Cmp { left: parse_reg(parts[1])?, right: parse_reg(parts[2])? }),
                "RET" => instructions.push(Instruction::Ret { reg: parse_reg(parts[1])? }),
                "PUSH" => instructions.push(Instruction::Push { value: parts[1].parse::<f64>().map(|n| n.into()).unwrap_or(parts[1].into()) }),
                "POP" => instructions.push(Instruction::Pop { var: parts[1].to_string() }),
                "CALL" => instructions.push(Instruction::CallLabel { label: parts[1].to_string() }),
                "ALLOC" => instructions.push(Instruction::Alloc { size: parts[1].parse()?, return_reg: parse_reg(parts[2])? }),
                "SYSCALL" => {
                    let id = parts[1].parse().ok().ok_or_else(|| anyhow!("Invalid syscall ID"))?;
                    let args = if parts.len() > 2 { parts[2..].join(" ").into_bytes() } else { Vec::new() };
                    instructions.push(Instruction::Syscall { id, args });
                }
                _ => {}
            }
        }
        Ok(instructions)
    }

    pub fn assemble(instructions: &[Instruction]) -> Result<String> {
        let mut wat = String::new();
        for instr in instructions {
            match instr {
                Instruction::Load { reg, addr } => wat.push_str(&format!("    i32.const {}\n    i32.load\n    local.set ${}\n", addr, reg)),
                Instruction::LoadImm { reg, value } => wat.push_str(&format!("    i32.const {}\n    local.set ${}\n", value, reg)),
                Instruction::Store { reg, addr } => wat.push_str(&format!("    i32.const {}\n    local.get ${}\n    i32.store\n", addr, reg)),
                Instruction::Add { dest, src1, src2 } => wat.push_str(&format!("    local.get ${}\n    local.get ${}\n    i32.add\n    local.set ${}\n", src1, src2, dest)),
                Instruction::Sub { dest, src1, src2 } => wat.push_str(&format!("    local.get ${}\n    local.get ${}\n    i32.sub\n    local.set ${}\n", src1, src2, dest)),
                Instruction::Mul { dest, src1, src2 } => wat.push_str(&format!("    local.get ${}\n    local.get ${}\n    i32.mul\n    local.set ${}\n", src1, src2, dest)),
                Instruction::Div { dest, src1, src2 } => wat.push_str(&format!("    local.get ${}\n    local.get ${}\n    i32.div_s\n    local.set ${}\n", src1, src2, dest)),
                Instruction::Ret { reg } => wat.push_str(&format!("    local.get ${}\n    return\n", reg)),
                _ => {}
            }
        }
        Ok(wat)
    }
}

fn parse_reg(s: &str) -> Result<u8> {
    s.strip_prefix('R').and_then(|s| s.parse().ok()).ok_or_else(|| anyhow!("Invalid register: {}", s))
}
