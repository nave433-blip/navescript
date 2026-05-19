# Nλvescript (NaveScript)

**Author:** Evan Shipley

A universal, polyglot coding language and interpreter built for performance and portability. Nλvescript is the "Universal Omni-Language," designed to bridge multiple languages, capabilities, and hardware targets under a single, highly-optimized runtime.

## 🚀 Core Features (v0.3.0)

- **WASI 0.2 (Preview 2)**: Built on the WebAssembly Component Model for modern, composable binaries.
- **Component Model**: True "LEGO brick" modularity with rich, language-agnostic types via WIT.
- **GraalVM Polyglot**: Bridge for zero-copy execution of Python, JavaScript, and Ruby.
- **Capability-Based Security**: Explicit declaration of required OS permissions at the "World" level.
- **High-Performance Runtime**: Powered by **Wasmtime** component-model engine.

## 🛠 Installation

```bash
# Requires Rust and Cargo
git clone https://github.com/naveos/navescript
cd navescript
cargo build --release
```

## 🏃 Usage

### Run a script (as a Component)
```bash
cargo run -- run examples/wasi_test.nave
```

### Compile to WebAssembly Component (WAT)
```bash
cargo run -- compile examples/wasi_test.nave --output component.wat
```

### Check Bridge Status
```bash
cargo run -- status
```

## 📂 Repository Structure

- `src/parser.rs`: Component and World definition parser.
- `src/ir.rs`: Intermediate Representation for Component Interfaces.
- `src/compiler.rs`: Translates Nλ-IR to WASI 0.2 Component WAT.
- `src/runtime.rs`: Wasmtime component-model execution engine.
- `src/polyglot.rs`: GraalVM cross-language bridge architecture.
- `wit/navescript.wit`: Interface definition for the Nλvescript world.

## ⚖️ License

MIT License
