// navescript/runtime/src/jit.rs (Expanded)

use crate::loader::{Instruction, NasModule};
use crate::nasi::syscall_handler;
use memmap2::MmapMut;
use std::collections::HashMap;
use std::ptr;

// Maximum size of the linear memory for a NAS module (e.g., 64KB)
const LINEAR_MEMORY_SIZE: usize = 64 * 1024;

// Represents a compiled, executable module.
pub struct ExecutableModule {
    code: MmapMut,   // Executable memory region
    memory: MmapMut, // Linear memory for the NAS module
}

impl ExecutableModule {
    // Unsafe because it executes raw machine code.
    pub unsafe fn run(&mut self) -> u64 {
        let memory_ptr = self.memory.as_mut_ptr();
        let code_fn: extern "C" fn(*mut u8) -> u64 = std::mem::transmute(self.code.as_ptr());
        code_fn(memory_ptr)
    }
}

// The JIT compilation engine.
pub struct JitEngine {
    registers: [u64; 16], // Conceptual registers R0-R15
    current_code_offset: usize,
    label_offsets: HashMap<String, usize>,
    patch_list: Vec<(String, usize)>, // List of labels to backpatch
}

impl JitEngine {
    pub fn new() -> Self {
        JitEngine {
            registers: [0; 16],
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

        unsafe {
            // Function prologue: push rbp, mov rbp, rsp
            self.emit_bytes(code_ptr, &[0x55, 0x48, 0x89, 0xE5]);
            // Save RDI (arg1, which is memory_ptr) to a callee-saved register (R14)
            self.emit_bytes(code_ptr, &[0x49, 0x89, 0xFE]); // mov r14, rdi
        }

        for instruction in &module.instructions {
            match instruction {
                Instruction::LABEL(name) => {
                    self.label_offsets.insert(name.clone(), self.current_code_offset);
                    println!("[JIT] Label {}: at offset {}", name, self.current_code_offset);
                },
                Instruction::LI(reg, imm) => {
                    println!("[JIT] Compiling: LI R{}, {}", reg, imm);
                    // movabs rax, imm64 (conceptual R0=RAX for simplicity)
                    // TODO: Map R0-R15 to actual physical registers or stack slots
                    self.emit_bytes(code_ptr, &[0x48, 0xB8]); // mov rax, imm64
                    self.emit_bytes(code_ptr, &imm.to_le_bytes()); // 8-byte immediate
                },
                Instruction::ADD(dest, src1, src2) => {
                    println!("[JIT] Compiling: ADD R{}, R{}, R{}", dest, src1, src2);
                    // Simplified: just add R0 + R0 and put in R0
                    self.emit_bytes(code_ptr, &[0x48, 0x01, 0xC0]); // add rax, rax
                },
                Instruction::SUB(dest, src1, src2) => {
                    println!("[JIT] Compiling: SUB R{}, R{}, R{}", dest, src1, src2);
                    self.emit_bytes(code_ptr, &[0x48, 0x29, 0xC0]); // sub rax, rax (simplified)
                },
                Instruction::MUL(dest, src1, src2) => {
                    println!("[JIT] Compiling: MUL R{}, R{}, R{}", dest, src1, src2);
                    self.emit_bytes(code_ptr, &[0x48, 0xF7, 0xE0]); // mul rax (simplified)
                },
                Instruction::DIV(dest, src1, src2) => {
                    println!("[JIT] Compiling: DIV R{}, R{}, R{}", dest, src1, src2);
                    self.emit_bytes(code_ptr, &[0x48, 0x99, 0x48, 0xF7, 0xF9]); // cqo, idiv rcx (simplified)
                },
                Instruction::LOAD(reg, addr_str) => {
                    println!("[JIT] Compiling: LOAD R{}, {}", reg, addr_str);
                    // Example: mov rax, [r14 + offset] (if addr_str is offset)
                    // TODO: Parse addr_str ("R15 + 8") and emit memory access
                    // For now, emit a placeholder.
                    self.emit_bytes(code_ptr, &[0x90]); // NOP
                },
                Instruction::STORE(reg, addr_str) => {
                    println!("[JIT] Compiling: STORE R{}, {}", reg, addr_str);
                    // Example: mov [r14 + offset], rax
                    // TODO: Parse addr_str and emit memory store
                    self.emit_bytes(code_ptr, &[0x90]); // NOP
                },
                Instruction::JMP(label) => {
                    println!("[JIT] Compiling: JMP {}", label);
                    self.emit_bytes(code_ptr, &[0xE9]); // jmp rel32
                    self.patch_list.push((label.clone(), self.current_code_offset)); // Store offset to backpatch
                    self.emit_bytes(code_ptr, &[0x00, 0x00, 0x00, 0x00]); // Placeholder for relative address
                },
                Instruction::JZ(reg, label) => {
                    println!("[JIT] Compiling: JZ R{}, {}", reg, label);
                    // cmp R_reg, 0; jz rel32
                    self.emit_bytes(code_ptr, &[0x48, 0x83, 0xF8, 0x00]); // cmp rax, 0 (simplified R0=RAX)
                    self.emit_bytes(code_ptr, &[0x0F, 0x84]); // jz rel32
                    self.patch_list.push((label.clone(), self.current_code_offset));
                    self.emit_bytes(code_ptr, &[0x00, 0x00, 0x00, 0x00]);
                },
                Instruction::CALL(label) => {
                    println!("[JIT] Compiling: CALL {}", label);
                    self.emit_bytes(code_ptr, &[0xE8]); // call rel32
                    self.patch_list.push((label.clone(), self.current_code_offset));
                    self.emit_bytes(code_ptr, &[0x00, 0x00, 0x00, 0x00]);
                },
                Instruction::RET => {
                    println!("[JIT] Compiling: RET");
                    // Function epilogue would handle actual return
                    self.emit_bytes(code_ptr, &[0xC3]); // ret (for simple functions)
                },
                Instruction::PUSH(reg) => {
                    println!("[JIT] Compiling: PUSH R{}", reg);
                    self.emit_bytes(code_ptr, &[0x50 + *reg]); // push r<reg> (simplified)
                },
                Instruction::POP(reg) => {
                    println!("[JIT] Compiling: POP R{}", reg);
                    self.emit_bytes(code_ptr, &[0x58 + *reg]); // pop r<reg> (simplified)
                },
                Instruction::SYSCALL(id, arg1, arg2, arg3) => {
                    println!("[JIT] Compiling: SYSCALL {}", id);
                    // Simplified:
                    // mov rdi, id; mov rsi, mem_ptr; mov rdx, arg1; mov rcx, arg2
                    // call syscall_handler
                    unsafe {
                        let handler_addr = syscall_handler as *const () as u64;
                        // Save registers if needed (not done here for simplicity)
                        
                        self.emit_bytes(code_ptr, &[0xBF, (*id & 0xFF) as u8, ((*id >> 8) & 0xFF) as u8, ((*id >> 16) & 0xFF) as u8, ((*id >> 24) & 0xFF) as u8]); // mov edi, id
                        self.emit_bytes(code_ptr, &[0x49, 0x89, 0xF6]); // mov rsi, r14 (memory_ptr)
                        self.emit_bytes(code_ptr, &[0xBA, (*arg1 & 0xFF) as u8, ((*arg1 >> 8) & 0xFF) as u8, ((*arg1 >> 16) & 0xFF) as u8, ((*arg1 >> 24) & 0xFF) as u8]); // mov edx, arg1
                        self.emit_bytes(code_ptr, &[0xB9, (*arg2 & 0xFF) as u8, ((*arg2 >> 8) & 0xFF) as u8, ((*arg2 >> 16) & 0xFF) as u8, ((*arg2 >> 24) & 0xFF) as u8]); // mov ecx, arg2
                        self.emit_bytes(code_ptr, &[0x41, 0xB8, (*arg3 & 0xFF) as u8, ((*arg3 >> 8) & 0xFF) as u8, ((*arg3 >> 16) & 0xFF) as u8, ((*arg3 >> 24) & 0xFF) as u8]); // mov r8d, arg3
                        
                        self.emit_bytes(code_ptr, &[0x48, 0xB8]); // mov rax, handler_addr
                        self.emit_bytes(code_ptr, &handler_addr.to_le_bytes());
                        self.emit_bytes(code_ptr, &[0xFF, 0xD0]); // call rax
                        // Result in RAX is implicitly in R0 for NAS
                    }
                },
            }
        }

        // Backpatch labels
        for (label, offset_pos) in &self.patch_list {
            if let Some(target_offset) = self.label_offsets.get(label) {
                // Calculate relative offset: target - (current_instruction_address + instruction_length)
                // instruction_length for rel32 is 4 bytes
                let relative_offset = (*target_offset as isize - (offset_pos + 4) as isize) as i32;
                unsafe {
                    ptr::copy_nonoverlapping(relative_offset.to_le_bytes().as_ptr(), code_ptr.add(*offset_pos), 4);
                }
            } else {
                return Err(format!("Undefined label: {}", label));
            }
        }

        unsafe {
            // Epilogue: mov rsp, rbp; pop rbp; ret
            self.emit_bytes(code_ptr, &[0x48, 0x89, 0xEC, 0x5D, 0xC3]);
        }
        
        // Make the memory executable
        let mut code = code_mmap.make_exec().map_err(|e| e.to_string())?;

        Ok(ExecutableModule { code, memory: linear_memory })
    }
}
