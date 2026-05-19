// NASM (Nλvescript Assembly) Reference Specification
// Mimics WASM text format but optimized for the Nλvescript VM

/* 
   NASM Primitives:
   - Registers: R0-R15
   - Stack: Managed natively by the Nλvescript VM
   - Memory: Capability-guarded linear memory
*/

// Example NASM source (nasm_example.nasm)
module core_math {
    export add;
    
    // NASM is RISC-like, mapping directly to Nλvescript VM opcodes
    func add(R0: i32, R1: i32) -> i32 {
        LOAD R0, 0      // Load arg0
        LOAD R1, 1      // Load arg1
        ADD  R0, R1     // R0 = R0 + R1
        RET  R0         // Return result
    }
}
