# Nλvescript (NaveScript) v3.0

**Author:** Evan Shipley  
**Status:** Universal Polyglot Runtime (v3.0.0)  

Nλvescript v3.0 is an exhaustive, high-performance, polyglot coding language and interpreter designed to bridge virtually every major programming language, system capability, and hardware target under a single, highly-optimized, and secure runtime.

---

## 🎯 The Universal Polyglot Advantage

Nλvescript v3.0 transcends traditional interop tools by acting as a **Universal Orchestration Layer**. It eliminates the need for manual glue code (cgo, PyO3, etc.) by providing a unified, type-safe interface to the world's most powerful ecosystems.

### Features
- **Universal Polyglot Bridge**: Native execution and data transmutation for **20+ languages** (Python, JS, Go, Rust, C, C++, Ruby, PHP, Swift, Kotlin, Zig, Nim, and more).
- **Exhaustive Grammar**: Complete support for every modern operator and architectural keyword.
- **Production-Grade Type System**: A robust Hindley-Milner engine with **Union/Intersection types**, **Generics with constraints**, **Conditional types**, and **`infer` keyword** support.
- **Intelligent Error Correction**: Built-in **Levenshtein-based fuzzy suggestion engine** and robust syntax synchronization.
- **Hardened Security**: Granular capability control (Strict/Permissive) and a full cryptographic suite (Argon2id, Ed25519, SHA-3, RSA).
- **Legitimate Security Research Suite**: Native binary analysis (ELF/PE/Mach-O), process forensics, network telemetry, and fuzzer primitives.
- **Omni-Language Capabilities**: Unique dimensional/unit-aware types and dynamic contextual dialects.

---

## 🚀 Key Feature Categories

### 1. Multi-Protocol Networking
- **HTTP/gRPC/WebSocket**: High-performance, asynchronous clients with built-in connection pooling.
- **Enterprise Protocols**: Native **LDAP** integration and advanced network security.

### 2. High-Scale Data & ML
- **Distributed Computing**: Direct orchestration of **Apache Spark**, **Dask**, and **Ray**.
- **Data Analysis**: Native wrappers for **Pandas** and high-speed **Meilisearch** integration.

### 3. Deep Hardware Interfacing
- **Peripheral Suite**: Full support for **Serial (RS-232)**, **USB (libusb)**, **HID**, **I2C**, and **SPI**.

### 4. Media & Observability
- **Processing**: Native **Image** and **Audio** manipulation; **ffmpeg** integration.
- **Telemetry**: Real-time **Prometheus** metrics and **OpenTelemetry (OTLP)** tracing.

---

## 📂 Repository Architecture

- `src/`: Core engine pipeline (Lexer, Parser, HM Type Checker, Compiler, Runtime).
- `src/drivers/`: Unified database and hardware driver stack.
- `src/security/`: Native security research and forensics modules.
- `src/stdlib/`: Exhaustive standard library including Internationalization (ICU4X).
- `wit/`: WASI 0.2 / Component Model contract definitions.

---

## 🛠 Command-Line Interface (CLI)

```bash
# Execute a script with resource limits
navescript run script.ns --timeout 30000 --memory-limit 1024 --sandbox strict

# Run a snippet directly
navescript eval 'print(crypto_sha256("hello"))'

# Manage the polyglot environment
navescript polyglot status
```

---

## 🏃 Installation

```bash
# Requires Rust (2021 edition)
git clone https://github.com/naveos/navescript
cd navescript
cargo build --release
```

## ⚖️ License
MIT License
