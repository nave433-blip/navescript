# The Navescript Book

Welcome to the official documentation for Navescript (NvS).

## Introduction
Navescript is a sovereign, self-hosting, polyglot language designed for dynamic instrumentation and high-performance execution.

## Core Concepts
- **NAS (Navescript Assembly)**: The low-level target for all NvS programs.
- **NASI (Navescript Application System Interface)**: A secure, sovereign system call interface.
- **Self-Hosting**: The NvS compiler and VM are written in NvS.

## Standard Library
- `std.ui`: Native GUI framework.
- `std.db`: Native document database.
- `std.ai`: Native tensor and ML hooks.
- `std.async`: Advanced asynchronous primitives.

## Tooling
- `nvs`: The core runtime and compiler.
- `nvc`: The Navescript Crate Manager.
- `nvm`: The Navescript Version Manager.

## Getting Started
```bash
nvc init my_app
cd my_app
nvc build
nvs run bin/app
```
