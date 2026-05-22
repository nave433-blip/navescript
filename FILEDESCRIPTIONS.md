# Navescript v2.0 File Descriptions

This document provides an exhaustive map of the `navescript` codebase, detailing the purpose and responsibility of each core file.

## 核心引擎 (Core Engine Pipeline) - `src/`

| File | Responsibility |
| :--- | :--- |
| `main.rs` | Entry point. Orchestrates the CLI, parses arguments, and manages the execution lifecycle. |
| `ns_lexer.rs` | Universal Lexer. Recognizes exhaustive tokens, operators, and 60+ keywords across languages. |
| `ns_parser.rs` | Smart Parser. Converts token streams into an advanced AST with support for complex types, patterns, and architectural blocks. |
| `types.rs` | Hindley-Milner Type System. Handles unification, inference, and fuzzy identifier suggestions. |
| `ns_compiler.rs` | Target Compiler. Translates AST nodes into Navescript Intermediate Representation (IR). |
| `ir.rs` | Intermediate Representation (IR). Defines the instruction set and step-based execution format. |
| `runtime.rs` | Execution Engine. A high-performance WASI-compatible runtime with sandboxing and resource limit enforcement. |
| `polyglot.rs` | Universal Polyglot Bridge. Orchestrates foreign code execution across 15+ compiled and interpreted languages. |

## 标准库 (Standard Library) - `src/stdlib/`

| File | Module | Responsibility |
| :--- | :--- | :--- |
| `mod.rs` | Registry | Centralized registration of all standard library modules into the runtime. |
| `console.rs` | Console | Standard I/O operations (log, error, clear). |
| `fs.rs` | File System | High-level, capability-guarded file operations. |
| `http.rs` | Network | Asynchronous HTTP client (GET/POST) using `reqwest`. |
| `crypto.rs` | Security | Comprehensive suite (SHA-2/3, Argon2id, Ed25519) for hashing and signatures. |
| `intl.rs` | Internationalization | Locale-aware formatting (Currency, Dates) using `icu4x`. |
| `system.rs` | System | OS-level orchestration (Process management, Environment, Hardware telemetry). |

## 外部接口与协议 (Interfaces & Protocols) - `src/` & `wit/`

| File | Category | Responsibility |
| :--- | :--- | :--- |
| `sqlite.rs` | Database | Host-side binding for native SQLite integration. |
| `postgres.rs` | Database | Asynchronous PostgreSQL client handler. |
| `mysql.rs` | Database | Asynchronous MySQL client handler. |
| `redis.rs` | Database | Asynchronous RESP-based Redis client. |
| `db.rs` | Database | Unified database abstraction facade (facade for all DB drivers). |
| `websocket.rs` | Network | Asynchronous, bidirectional WebSocket client. |
| `grpc.rs` | Network | Foundations for gRPC client/server interaction. |
| `ldap.rs` | Network | Enterprise directory access bridge. |
| `serial.rs` | Hardware | Asynchronous Serial (RS-232) interface. |
| `usb.rs` | Hardware | Low-level USB (libusb) device handler. |
| `hid.rs` | Hardware | Human Interface Device (HID) peripheral driver. |
| `i2c.rs` | Hardware | I2C sensor/bus interface. |
| `spi.rs` | Hardware | High-speed SPI peripheral interface. |

## 性能与分布式 (Performance & Distributed) - `src/`

| File | Responsibility |
| :--- | :--- |
| `cache.rs` | High-performance, thread-safe in-memory caching using `moka`. |
| `scheduler.rs` | Worker Pool manager for efficient CPU-intensive task distribution. |
| `batch.rs` | High-throughput parallel task execution engine. |
| `distributed.rs` | Distributed computing bridge (Dask, Ray, etc.). |
| `queue.rs` | Distributed task queue integration (AMQP/RabbitMQ). |
| `metrics.rs` | Prometheus telemetry exporter for runtime monitoring. |
| `tracing.rs` | Distributed tracing handler using OpenTelemetry (OTLP). |

---

## 自动化审计与路线图 (Audit & Roadmap) - Root

| File | Purpose |
| :--- | :--- |
| `TO DO LIST.py` | The master audit file for missing v1.0 features. |
| `missingsubfeatures.py` | Detailed inventory of 500+ granular sub-feature requirements. |
| `missingsymbols.py` | Complete reference for all known language symbols and context variables. |
| `fixthese.py` | Deep-dive inventory of obscure edge cases and internationalization gaps. |
| `ROADMAP.md` | The high-level implementation tracking document for v2.0. |
| `todolist.py` | Interactive CLI tool for tracking implementation progress. |

---

## SDK 与 文档 (SDK & Docs)

- `sdk/`: Universal FFI headers and language bindings.
- `include/`: C-compatible API headers for runtime embedding.
- `examples/`: Comprehensive test scripts demonstrating polyglot and hardware capabilities.
