<div align="center">

# 📐 QLang Compiler Engine

**A lightweight, Rust-powered matrix & vector DSL compiler targeting C with bundled TinyCC.**

[![Rust](https://img.shields.io/badge/rust-2021-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux-lightgrey.svg?style=flat-square)](#)

[Quick Start](#-quick-start) •
[Architecture](#-architecture) •
[Language Features](#-language-features) •
[Contributing](#-contributing)

</div>

---

## 💡 Overview

**QLang** is a domain-specific language (DSL) and compiler engine designed for high-performance matrix and vector manipulation.

The compiler performs **static type and shape checking** during compilation, then generates portable **C source code** that can be compiled and executed using a bundled TinyCC (`tcc`) toolchain.

The goal is to provide a concise, high-level syntax for numerical operations while keeping the generated runtime lightweight and portable.

## ✨ Key Features

- **Matrix & Vector Primitives** — Native syntax for defining and manipulating numerical data.
- **2D Slicing** — Supports matrix slicing with `M[r1..r2, c1..c2]` syntax.
- **Transpose Operations** — Built-in matrix transposition through `transpose(M)`.
- **Static Shape Checking** — Matrix dimensions are validated during compilation to catch invalid operations early.
- **Rust-Powered Compiler** — A modular compiler pipeline implemented in Rust.
- **C Code Generation** — Validated QLang programs are translated into C.
- **Bundled TinyCC** — Compiles generated C code without requiring a large external C toolchain.
- **CLI Workflow** — Run QLang scripts directly or build standalone executables.

---

## 🏗️ Architecture

QLang is organized as a modular Rust workspace:

| Crate / Module | Responsibility |
| :--- | :--- |
| `crates/ql_lexer` | Tokenizes QLang source code into a stream of tokens. |
| `crates/ql_parser` | Parses tokens into an Abstract Syntax Tree (AST). |
| `crates/ql_ast` | Defines AST structures, nodes, expressions, and operators. |
| `crates/ql_checker` | Performs type inference and static matrix shape validation. |
| `crates/ql_codegen` | Converts the validated AST into C source code. |
| `src/main.rs` | Provides the main CLI entrypoint and commands such as `run` and `build`. |
| `tcc/` | Bundled TinyCC toolchain used to compile generated C code. |

### Compilation Pipeline

```text
QLang Source
     │
     ▼
┌─────────────┐
│   Lexer     │
└──────┬──────┘
       ▼
┌─────────────┐
│   Parser    │
└──────┬──────┘
       ▼
┌─────────────┐
│     AST     │
└──────┬──────┘
       ▼
┌─────────────┐
│   Checker   │
│ Type + Shape│
└──────┬──────┘
       ▼
┌─────────────┐
│   Codegen   │
│   QLang → C │
└──────┬──────┘
       ▼
┌─────────────┐
│    TinyCC   │
└──────┬──────┘
       ▼
  Native Binary
```

---

## 🚀 Quick Start

### Prerequisites

- [Rust Toolchain](https://www.rust-lang.org/tools/install) with Rust 2021 edition
- A local checkout of the repository
- Bundled TinyCC available in `tcc/`

### Run a QLang Script

Execute a QLang source file directly:

```bash
cargo run -- run test.ql
```

### Build an Executable

Compile a QLang source file into a standalone executable:

```bash
cargo run -- build test.ql -o test.exe
```

On Linux, use an appropriate executable name or output path:

```bash
cargo run -- build test.ql -o test
```

---

## 📝 Language Features

### Matrix Literals

Matrices can be declared using nested array syntax:

```qlang
let M = [
    [1.0, 2.0, 3.0],
    [4.0, 5.0, 6.0],
    [7.0, 8.0, 9.0]
];
```

### 2D Slicing

Select a rectangular region from a matrix:

```qlang
let sub = M[0..2, 1..3];
```

### Transpose

Transpose a matrix using the built-in `transpose` operation:

```qlang
let t = transpose(M);
```

### Printing

Matrix values can be printed directly:

```qlang
print(sub);
print(t);
```

### Complete Example

```qlang
// Define a 3x3 Matrix
let M = [
    [1.0, 2.0, 3.0],
    [4.0, 5.0, 6.0],
    [7.0, 8.0, 9.0]
];

// 2D Slicing & Transpose
let sub = M[0..2, 1..3];
let t = transpose(M);

print(sub);
print(t);
```

Save the example as `test.ql`, then run:

```bash
cargo run -- run test.ql
```

---

## 📁 Project Structure

```text
qlc/
├── crates/
│   ├── ql_ast/
│   ├── ql_checker/
│   ├── ql_codegen/
│   ├── ql_lexer/
│   └── ql_parser/
├── src/
│   └── main.rs
├── tcc/
├── test.ql
├── Cargo.toml
└── README.md
```

---

## 🔧 Development

Build the workspace:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

Run the compiler in development mode:

```bash
cargo run -- run test.ql
```

For a release build:

```bash
cargo build --release
```

---

## 📌 Roadmap

Potential areas for future development include:

- [ ] Expanded vector operations
- [ ] Matrix arithmetic and linear algebra primitives
- [ ] More comprehensive compile-time diagnostics
- [ ] Additional C code-generation optimizations
- [ ] Cross-platform TinyCC packaging
- [ ] Standard library for numerical operations
- [ ] Improved CLI diagnostics and developer tooling

---

## 🤝 Contributing

Contributions are welcome.

Before submitting a change:

1. Create a focused branch for your work.
2. Keep compiler stages modular and independently testable.
3. Add or update tests for new language behavior.
4. Run the test suite with `cargo test`.
5. Keep generated C code valid and portable where possible.
6. Open a pull request describing the change and its motivation.

---

## 📄 License

QLang is released under the [MIT License](LICENSE).
