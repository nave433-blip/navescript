# Nλvescript (NaveScript)

**Author:** Evan Shipley  
**Status:** Production-Hardened Polyglot Runtime (v0.3.2)  

Nλvescript is the "Universal Omni-Language," a high-performance, polyglot coding language and interpreter designed to bridge multiple languages, capabilities, and hardware targets under a single, highly-optimized, and secure runtime.

---

## 🎯 Why Nλvescript?

Traditional interop tools like `cgo` (Go), `PyO3` (Python-Rust), or `rust-cxx` are fragmented and require custom glue code for every language pair. Nλvescript solves this by acting as a **Universal Polyglot Layer**:

| Tool | Scope | Complexity | Performance |
| :--- | :--- | :--- | :--- |
| **cgo** | Go ↔ C | High (manual FFI) | Moderate |
| **PyO3** | Rust ↔ Python | Moderate (manual binding) | High |
| **Nλvescript** | **Universal** | **Low (Unified IR)** | **Near-Native** |

### The Nλvescript Advantage:
- **Unified IR (Intermediate Representation)**: Compiles foreign code into a single, high-performance IR, eliminating language-specific FFI wrappers.
- **Interactive Stdin**: Native `input` operation for real-time user interaction.
- **Complex Control Flow**: Production-ready `if`, `for_each`, and `try/catch` logic.
- **WASI 0.2 Component Model**: Uses WIT (WebAssembly Interface Types) for secure, language-agnostic modularity.
- **Polyglot Transmutation**: Seamlessly converts foreign data structures into native Nλvescript types.
- **Hardened Security**: Includes a load-time **NASM Verifier** for memory safety and a **NASI Capability Host** for granular system-call security.

---

## 🌍 Code Translation & Interop

Nλvescript features a unique **Polyglot Transmutation Engine** that allows it to translate, execute, and verify code from: **Python, JavaScript, Ruby, Bash, and Perl**.

- **Transmutation Loop**:
    1. **Translate**: Accepts foreign code.
    2. **Execute**: Runs in a sandboxed, capability-negotiated environment.
    3. **Transmute**: Converts foreign output/state into native JSON representations.
    4. **Verify**: Runs native Nλvescript logic against foreign outputs to ensure semantic correctness.
    5. **Assert**: Performs rigorous equality checks (Assertion engine) to guarantee translation fidelity.

---

## 🚀 Core Features (v0.3.2)

- **NASM (Nλvescript Assembly)**: RISC-like, human-readable assembly format for low-level systems programming.
- **NASI (Nλvescript System Interface)**: Self-hosted, capability-based interface for accessing OS resources securely.
- **Native Operations**: 
    - `http_get`, `http_post`: Native high-performance networking.
    - `file_read`, `file_write`: Capability-guarded I/O.
    - `shell_exec`: Controlled OS interaction.
    - `input`: Interactive command-line support.
    - `json_parse`: Native transmutation of string-data to IR types.
- **NVsodide**: Browser-resident WASM runtime with JS-bridge security negotiation.
- **Omni-Bridge SDK**: Standardized FFI layer (`include/nave_api.h`) for embedding in **Rust, Go, Swift, Java, C++, and Python**.

---

## 🛠 Installation

```bash
# Requires Rust and Cargo
git clone https://github.com/nave433-blip/navescript
cd navescript
cargo build --release
```

---

## 🏃 Usage

### Run a script (as a Component)
```bash
cargo run -- run examples/wasi_test.nave
```

### Compile to WebAssembly Component (WAT)
```bash
cargo run -- compile examples/nasm_example.nave --output demo.wat
```

### Check Polyglot Bridge Status
```bash
cargo run -- status
```

---

## 📂 Repository Structure

- `src/`: Core engine (Compiler, Runtime, Parser, IR).
- `examples/`: NASM/NASI reference specs and test programs.
- `sdk/`: Universal FFI bindings for host languages (Rust, Go, Python, etc.).
- `wit/`: Interface definitions for WASI 0.2 component interoperability.
- `plugins/`: Multi-language plugin registry.

## ⚖️ License
MIT License
