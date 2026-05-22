# The "Nvs" Bridge Family
Universal FFI bindings for Nλvescript.

| Bridge | Language | Implementation Method |
| :--- | :--- | :--- |
| **PyNvs** | Python | ctypes / Python C API |
| **cNvs** | C/C++ | Native Shared Library (.so/.dylib) |
| **GoNvs** | Go | cgo |
| **Nvsr** | Rust | direct FFI (`libc` crate) |
| **SwiftNvs** | Swift | `@_cdecl` / C-Interop |
| **JvNvs** | Java | JNI |

## Standardized Usage
Every "Nvs" bridge exports a unified structure:
```c
typedef struct {
    void* (*init)();
    int (*run)(void* ctx, const char* path);
    void (*free)(void* ctx);
} NvsAPI;
```
