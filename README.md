<div align="center">

# QLang

A lightweight programming language for matrix and vector computation.

[Quick Start](#quick-start) •
[Architecture](#architecture) •
[Language Features](#language-features) •
[Contributing](#contributing)

</div>

## Overview

QLang is a lightweight programming language designed for matrix and vector computation.

QLang programs are compiled to C and then converted into native executables using a bundled TinyCC toolchain.

## Why QLang?

QLang is designed to keep numerical code concise while providing compile-time validation for matrix operations.

* Matrix and vector operations
* Static type and shape checking
* Two-dimensional matrix slicing
* Matrix transposition
* C code generation
* Native executable output
* Bundled TinyCC compiler

## Quick Start

### Build the Compiler

```bash
cargo build
```

### Run a QLang Program

```bash
cargo run -- run test.ql
```

### Build a Native Executable

```bash
cargo run -- build test.ql -o test.exe
```

## Example

```qlang
let M = [
    [1.0, 2.0, 3.0],
    [4.0, 5.0, 6.0],
    [7.0, 8.0, 9.0]
];

let sub = M[0..2, 1..3];
let t = transpose(M);

print(sub);
print(t);
```

## Architecture

QLang uses a modular compiler pipeline:

```text
QLang Source
     │
     ▼
   Lexer
     │
     ▼
   Parser
     │
     ▼
     AST
     │
     ▼
  Checker
     │
     ▼
 C Codegen
     │
     ▼
   TinyCC
     │
     ▼
Native Binary
```

The compiler is implemented in Rust and organized into the following crates:

```text
crates/
├── ql_ast
├── ql_checker
├── ql_codegen
├── ql_lexer
└── ql_parser
```

## Language Features

### Matrix and Vector Operations

QLang provides built-in support for numerical data structures, including matrices and vectors.

### Static Type and Shape Checking

Matrix dimensions and types are validated during compilation to help detect invalid operations before execution.

### Matrix Slicing

Two-dimensional matrices can be sliced using row and column ranges:

```qlang
let sub = M[0..2, 1..3];
```

### Matrix Transposition

Matrices can be transposed using the built-in `transpose` function:

```qlang
let t = transpose(M);
```

### C Code Generation

QLang translates source programs into C code as an intermediate compilation step.

### Native Executables

The generated C code is compiled into a native executable using the bundled TinyCC toolchain.

## Status

QLang is an early-stage programming language and compiler.

The language, compiler, and standard library are under active development. APIs, syntax, and project structure may change over time.

## Contributing

Contributions, experiments, and feedback are welcome.

To contribute:

1. Fork the repository.
2. Create a branch for your changes.
3. Implement and test your changes.
4. Open a pull request with a clear description.

For compiler development, refer to the source code and issue tracker.

## License

QLang is licensed under the MIT License.
