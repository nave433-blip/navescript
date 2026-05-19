# Nλvescript Omni-Bridge SDK

This directory contains the universal FFI bindings and SDK headers for embedding Nλvescript into any host language.

## Architecture
Nλvescript exposes a stable C-ABI (`nave_api.h`). All language-specific SDKs wrap this C-ABI to provide idiomatic interop (e.g., Python packages, Go modules, Java JNI).

## 1. C-ABI Interface (`include/nave_api.h`)
```c
#ifndef NAVE_API_H
#define NAVE_API_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    const char* name;
    void* handle;
} NaveContext;

// Initialize Nλvescript Runtime
NaveContext* nave_init();

// Execute a Nλvescript module
int nave_run(NaveContext* ctx, const char* module_path);

// Transmute data from Nλvescript to Host
const char* nave_get_result(NaveContext* ctx, const char* var_name);

void nave_free(NaveContext* ctx);

#ifdef __cplusplus
}
#endif

#endif
```

## 2. Universal Embedding SDKs (`sdk/`)
- `sdk/python/nave.py`: Uses `ctypes` to load `libnave.so`.
- `sdk/go/nave.go`: Uses `cgo` and `nave_api.h`.
- `sdk/rust/src/lib.rs`: Direct FFI binding to the core runtime.
- `sdk/swift/Nave.swift`: Uses `@_cdecl` and `NaveContext`.
- `sdk/java/Nave.java`: Uses `JNI` to call `libnave.so`.
- `sdk/perl/Nave.pm`: Uses `XS` to load the runtime.

## 3. Omni-Transpilation (The "Bridge")
You can use the SDKs to create bidirectional pipes:
1. **Host -> Nλvescript**: Language A -> `nave_run()` -> Nλvescript.
2. **Nλvescript -> Transmute**: Nλvescript -> JSON -> Language B.

Example (Go calling Perl via Nλvescript):
```go
// Go calling NaveScript
res := C.nave_run(ctx, C.CString("perl_script.nave"))
// NaveScript handles the internal execution/transmutation
```
