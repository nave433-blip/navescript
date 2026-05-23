# Navescript Assembly Specification (NAS) v1

## 1. Overview
NAS is a portable, sandboxed assembly language for the Navescript ecosystem. It is the compilation target for the `nvsc` compiler and is executed by the `nas-run` JIT runtime.

## 2. Memory
- Each module operates in a sandboxed, linear memory space.
- Memory can be grown via the `mem_grow` NASI call.

## 3. Registers
- 16 general-purpose 64-bit registers: `R0` - `R15`.
- `SP` (Stack Pointer), `BP` (Base Pointer), `IP` (Instruction Pointer).

## 4. Instruction Set (Example)
- `LI <reg>, <imm>`: Load immediate value into register.
- `MOV <dest_reg>, <src_reg>`: Move value between registers.
- `LOAD <reg>, <addr_reg>`: Load value from memory address in `addr_reg` into `reg`.
- `STORE <addr_reg>, <reg>`: Store value from `reg` into memory address in `addr_reg`.
- `ADD/SUB/MUL/DIV <dest>, <src1>, <src2>`: Arithmetic.
- `JMP <label>`: Unconditional jump.
- `JZ <reg>, <label>`: Jump if `reg` is zero.
- `CALL <label>`: Call a function.
- `RET`: Return from function.
- `SYSCALL <id>`: Execute a NASI system call.
- `FFI <lib_name>, <func_name>`: Call an external C function.

---
# Navescript Application System Interface (NASI) v1

## Filesystem (`fs`)
- `10`: `fs_open(path_ptr, path_len, flags) -> handle`
- `11`: `fs_read(handle, buf_ptr, buf_len) -> bytes_read`
- `12`: `fs_write(handle, buf_ptr, buf_len) -> bytes_written`
- `13`: `fs_close(handle)`

## Memory (`mem`)
- `50`: `mem_grow(pages) -> previous_pages`
- `51`: `gc_alloc(size) -> ptr`

## Networking (`net`)
- `100`: `net_socket_open(family, type) -> handle`
- `101`: `net_socket_connect(handle, addr_ptr, addr_len) -> err`
- `102`: `net_socket_write(handle, data_ptr, data_len) -> bytes_written`

## Foreign Function Interface (`ffi`)
- `200`: `ffi_load(lib_path_ptr, lib_path_len) -> lib_handle`
- `201`: `ffi_call(lib_handle, func_name_ptr, func_name_len, args_ptr, args_len) -> result`
