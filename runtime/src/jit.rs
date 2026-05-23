// navescript/runtime/src/jit.rs (Expanded with LOAD/STORE/DIV and improved register mapping)

use crate::loader::{Instruction, NasModule};
use crate::nasi::syscall_handler;
use memmap2::MmapMut;
use std::collections::HashMap;
use std::ptr;
use crate::gc::SimpleAllocator; // Import the allocator

// Maximum size of the linear memory for a NAS module (e.g., 64KB)
const LINEAR_MEMORY_SIZE: usize = 64 * 1024;

// Represents a compiled, executable module.
pub struct ExecutableModule {
    code: MmapMut,   // Executable memory region
    memory: MmapMut, // Linear memory for the NAS module
    allocator: SimpleAllocator, // Allocator tied to this module's memory
}

impl ExecutableModule {
    // Unsafe because it executes raw machine code.
    pub unsafe fn run(&mut self) -> u64 {
        let memory_ptr = self.memory.as_mut_ptr();
        let code_fn: extern "C" fn(*mut u8) -> u64 = std::mem::transmute(self.code.as_ptr());
        code_fn(memory_ptr) // Pass memory base pointer to the NAS function
    }
}

// Map conceptual NAS registers (R0-R3) to x86_64 registers
// R0 -> RAX, R1 -> RCX, R2 -> RDX, R3 -> RBX (common calling convention order for args)
fn nas_reg_to_x64(reg_id: u8) -> u8 {
    match reg_id {
        0 => 0, // RAX
        1 => 1, // RCX
        2 => 2, // RDX
        3 => 3, // RBX
        _ => 0, // Fallback to RAX
    }
}

// The JIT compilation engine.
pub struct JitEngine {
    current_code_offset: usize,
    label_offsets: HashMap<String, usize>,
    patch_list: Vec<(String, usize, Instruction)>, // (label, offset, instruction_type for size)
}

impl JitEngine {
    pub fn new() -> Self {
        JitEngine {
            current_code_offset: 0,
            label_offsets: HashMap::new(),
            patch_list: Vec::new(),
        }
    }

    // Helper to emit raw bytes
    fn emit_bytes(&mut self, code_ptr: *mut u8, bytes: &[u8]) {
        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), code_ptr.add(self.current_code_offset), bytes.len());
        }
        self.current_code_offset += bytes.len();
    }

    pub fn compile(&mut self, module: &NasModule) -> Result<ExecutableModule, String> {
        let mut code_mmap = MmapMut::map_anon(4096).map_err(|e| e.to_string())?;
        let code_ptr = code_mmap.as_mut_ptr();
        self.current_code_offset = 0;
        self.label_offsets.clear();
        self.patch_list.clear();

        let mut linear_memory = MmapMut::map_anon(LINEAR_MEMORY_SIZE).map_err(|e| e.to_string())?;
        let mut allocator = SimpleAllocator::new(LINEAR_MEMORY_SIZE); // One allocator per module

        unsafe {
            // Function prologue: push rbp, mov rbp, rsp
            self.emit_bytes(code_ptr, &[0x55, 0x48, 0x89, 0xE5]);
            // Save RDI (arg1, which is memory_ptr) to a callee-saved register (R14)
            self.emit_bytes(code_ptr, &[0x49, 0x89, 0xF6]); // mov r14, rdi (memory_ptr base)
        }

        for instruction in &module.instructions {
            match instruction {
                Instruction::LABEL(name) => {
                    self.label_offsets.insert(name.clone(), self.current_code_offset);
                    println!("[JIT] Label {}: at offset {}", name, self.current_code_offset);
                },
                Instruction::LI(reg, imm) => {
                    println!("[JIT] Compiling: LI R{}, {}", reg, imm);
                    let x64_reg = nas_reg_to_x64(*reg);
                    self.emit_bytes(code_ptr, &[0x48, 0xB8 + x64_reg]); // mov R(x64_reg), imm64
                    self.emit_bytes(code_ptr, &imm.to_le_bytes()); // 8-byte immediate
                },
                Instruction::ADD(dest, src1, src2) => {
                    println!("[JIT] Compiling: ADD R{}, R{}, R{}", dest, src1, src2);
                    // R_dest = R_src1 + R_src2
                    let x64_dest = nas_reg_to_x64(*dest);
                    let x64_src1 = nas_reg_to_x64(*src1);
                    let x64_src2 = nas_reg_to_x64(*src2);
                    self.emit_bytes(code_ptr, &[0x48, 0x8B, 0xC8 + x64_src2 * 8 + x64_src1]); // mov r_dest, r_src1
                    self.emit_bytes(code_ptr, &[0x48, 0x01, 0xC0 + x64_src1 * 8 + x64_dest]); // add r_dest, r_src2
                },
                Instruction::SUB(dest, src1, src2) => {
                    println!("[JIT] Compiling: SUB R{}, R{}, R{}", dest, src1, src2);
                    let x64_dest = nas_reg_to_x64(*dest);
                    let x64_src1 = nas_reg_to_x64(*src1);
                    let x64_src2 = nas_reg_to_x64(*src2);
                    self.emit_bytes(code_ptr, &[0x48, 0x8B, 0xC8 + x64_src2 * 8 + x64_src1]); // mov r_dest, r_src1
                    self.emit_bytes(code_ptr, &[0x48, 0x29, 0xC0 + x64_src1 * 8 + x64_dest]); // sub r_dest, r_src2
                },
                Instruction::MUL(dest, src1, src2) => {
                    println!("[JIT] Compiling: MUL R{}, R{}, R{}", dest, src1, src2);
                    // IMUL r_dest, r_src1, r_src2 (simplified to 3-operand form if possible)
                    // Basic: mov rax, src1; imul src2; mov dest, rax
                    let x64_dest = nas_reg_to_x64(*dest);
                    let x64_src1 = nas_reg_to_x64(*src1);
                    let x64_src2 = nas_reg_to_x64(*src2);
                    self.emit_bytes(code_ptr, &[0x48, 0x8B, 0xC8 + x64_src1 * 8]); // mov rax, r_src1
                    self.emit_bytes(code_ptr, &[0x48, 0xF7, 0xEA + x64_src2]); // imul r_src2 (operates on rax)
                    self.emit_bytes(code_ptr, &[0x48, 0x89, 0xC8 + x64_dest]); // mov r_dest, rax
                },
                Instruction::DIV(dest, src1, src2) => {
                    println!("[JIT] Compiling: DIV R{}, R{}, R{}", dest, src1, src2);
                    // mov rax, src1; xor rdx, rdx; div src2; mov dest, rax
                    let x64_dest = nas_reg_to_x64(*dest);
                    let x64_src1 = nas_reg_to_x64(*src1);
                    let x64_src2 = nas_reg_to_x64(*src2);
                    self.emit_bytes(code_ptr, &[0x48, 0x8B, 0xC8 + x64_src1 * 8]); // mov rax, r_src1
                    self.emit_bytes(code_ptr, &[0x48, 0x31, 0xD2]); // xor rdx, rdx
                    self.emit_bytes(code_ptr, &[0x48, 0xF7, 0xF8 + x64_src2]); // div r_src2 (operates on rax, rdx)
                    self.emit_bytes(code_ptr, &[0x48, 0x89, 0xC8 + x64_dest]); // mov r_dest, rax
                },
                Instruction::LOAD(reg, addr_str) => {
                    println!("[JIT] Compiling: LOAD R{}, {}", reg, addr_str);
                    if let Some(offset_val) = parse_address_offset(addr_str) {
                         let x64_reg = nas_reg_to_x64(*reg);
                         self.emit_bytes(code_ptr, &[0x4C, 0x8B, 0x86]); // mov rAX, [r14 + disp32] (for R8, etc.)
                         self.emit_bytes(code_ptr, &offset_val.to_le_bytes()); // disp32 (4 bytes)
                    } else {
                        eprintln!("[JIT Warning] LOAD: Cannot parse address string: {}", addr_str);
                        self.emit_bytes(code_ptr, &[0x90]); // NOP
                    }
                },
                Instruction::STORE(reg, addr_str) => {
                    println!("[JIT] Compiling: STORE R{}, {}", reg, addr_str);
                    if let Some(offset_val) = parse_address_offset(addr_str) {
                        let x64_reg = nas_reg_to_x64(*reg);
                        self.emit_bytes(code_ptr, &[0x4C, 0x89, 0x86]); // mov [r14 + disp32], rAX
                        self.emit_bytes(code_ptr, &offset_val.to_le_bytes()); // disp32 (4 bytes)
                    } else {
                        eprintln!("[JIT Warning] STORE: Cannot parse address string: {}", addr_str);
                        self.emit_bytes(code_ptr, &[0x90]); // NOP
                    }
                },
                Instruction::JMP(label) => {
                    println!("[JIT] Compiling: JMP {}", label);
                    self.emit_bytes(code_ptr, &[0xE9]); // jmp rel32
                    self.patch_list.push((label.clone(), self.current_code_offset, instruction.clone()));
                    self.emit_bytes(code_ptr, &[0x00, 0x00, 0x00, 0x00]); // Placeholder for relative address
                },
                Instruction::JZ(reg, label) => {
                    println!("[JIT] Compiling: JZ R{}, {}", reg, label);
                    let x64_reg = nas_reg_to_x64(*reg);
                    self.emit_bytes(code_ptr, &[0x48, 0x83, 0xF8 + x64_reg, 0x00]); // cmp r_reg, 0
                    self.emit_bytes(code_ptr, &[0x0F, 0x84]); // jz rel32
                    self.patch_list.push((label.clone(), self.current_code_offset, instruction.clone()));
                    self.emit_bytes(code_ptr, &[0x00, 0x00, 0x00, 0x00]);
                },
                Instruction::CALL(label) => {
                    println!("[JIT] Compiling: CALL {}", label);
                    self.emit_bytes(code_ptr, &[0xE8]); // call rel32
                    self.patch_list.push((label.clone(), self.current_code_offset, instruction.clone()));
                    self.emit_bytes(code_ptr, &[0x00, 0x00, 0x00, 0x00]);
                },
                Instruction::RET => {
                    println!("[JIT] Compiling: RET");
                    self.emit_bytes(code_ptr, &[0xC3]); // ret
                },
                Instruction::PUSH(reg) => {
                    println!("[JIT] Compiling: PUSH R{}", reg);
                    let x64_reg = nas_reg_to_x64(*reg);
                    self.emit_bytes(code_ptr, &[0x50 + x64_reg]); // push r_reg
                },
                Instruction::POP(reg) => {
                    println!("[JIT] Compiling: POP R{}", reg);
                    let x64_reg = nas_reg_to_x64(*reg);
                    self.emit_bytes(code_ptr, &[0x58 + x64_reg]); // pop r_reg
                },
                Instruction::SYSCALL(id, arg1, arg2, arg3) => {
                    println!("[JIT] Compiling: SYSCALL {}", id);
                    unsafe {
                        let handler_addr = syscall_handler as *const () as u64;
                        self.emit_bytes(code_ptr, &[0xBF, (*id & 0xFF) as u8, ((*id >> 8) & 0xFF) as u8, ((*id >> 16) & 0xFF) as u8, ((*id >> 24) & 0xFF) as u8]); // mov edi, id
                        self.emit_bytes(code_ptr, &[0x49, 0x89, 0xF6]); // mov rsi, r14 (memory_ptr)
                        self.emit_bytes(code_ptr, &[0xBA, (*arg1 & 0xFF) as u8, ((*arg1 >> 8) & 0xFF) as u8, ((*arg1 >> 16) & 0xFF) as u8, ((*arg1 >> 24) & 0xFF) as u8]); // mov edx, arg1
                        self.emit_bytes(code_ptr, &[0xB9, (*arg2 & 0xFF) as u8, ((*arg2 >> 8) & 0xFF) as u8, ((*arg2 >> 16) & 0xFF) as u8, ((*arg2 >> 24) & 0xFF) as u8]); // mov ecx, arg2
                        self.emit_bytes(code_ptr, &[0x41, 0xB8, (*arg3 & 0xFF) as u8, ((*arg3 >> 8) & 0xFF) as u8, ((*arg3 >> 16) & 0xFF) as u8, ((*arg3 >> 24) & 0xFF) as u8]); // mov r8d, arg3
                        
                        self.emit_bytes(code_ptr, &[0x48, 0xB8]); // mov rax, handler_addr
                        self.emit_bytes(code_ptr, &handler_addr.to_le_bytes());
                        self.emit_bytes(code_ptr, &[0xFF, 0xD0]); // call rax
                    }
                },
                Instruction::LABEL(_) => { /* Handled separately for insertion */ },
            }
        }

        // Backpatch labels
        for (label, offset_pos, instr_type) in &self.patch_list {
            if let Some(target_offset) = self.label_offsets.get(label) {
                let instruction_length = match instr_type {
                    Instruction::JMP(_) => 5,   // jmp rel32 (opcode 1 byte + imm 4 bytes)
                    Instruction::JZ(_,_) => 6,  // jz rel32 (opcode 2 bytes + imm 4 bytes)
                    Instruction::CALL(_) => 5, // call rel32 (opcode 1 byte + imm 4 bytes)
                    _ => 0, // Should not happen
                };
                let relative_offset = (*target_offset as isize - (*offset_pos as isize + instruction_length as isize)) as i32;
                unsafe {
                    ptr::copy_nonoverlapping(relative_offset.to_le_bytes().as_ptr(), code_ptr.add(*offset_pos), 4);
                }
            } else {
                return Err(format!("Undefined label: {}", label));
            }
        }

        unsafe {
            // Epilogue: mov rsp, rbp; pop rbp; ret
            self.emit_bytes(code_ptr, &[0x48, 0x89, 0xEC, 0x5D, 0xC3]); // mov rsp, rbp; pop rbp; ret
        }
        
        // Make the memory executable
        let mut code = code_mmap.make_exec().map_err(|e| e.to_string())?;

        Ok(ExecutableModule { code, memory: linear_memory, allocator })
    }
}

// Helper to parse address strings like "R15 + 8" to an offset.
fn parse_address_offset(addr_str: &str) -> Option<u32> {
    let parts: Vec<&str> = addr_str.splitn(3, ' ').collect();
    if parts.len() == 3 && parts[0] == "R15" && parts[1] == "+" {
        parts[2].parse::<u32>().ok()
    } else if parts.len() == 1 {
        parts[0].parse::<u32>().ok() // Direct offset
    } else {
        None
    }
}
