<div align="center">

# QLang

**Quant Lang**

A lightweight programming language for matrix and vector computation, targeted at AI/ML and numerical operations.

[Quick Start](#quick-start) •
[Architecture](#architecture) •
[Language Features](#language-features) •
[Contributing](#contributing)

</div>

## Overview

QLang (Quant Lang) is a lightweight programming language designed for numerical computation, tensor and matrix operations, and neural network primitives.

QLang programs are statically checked for type and shape compatibility, transpiled to C, and compiled into native executables using a bundled TinyCC toolchain.

## Why QLang?

QLang is designed to keep numerical and AI forward-pass code concise while providing compile-time validation for matrix operations and preventing common runtime shape mismatches.

* Linear algebra operations for matrices and vectors
* Matrix multiplication using `*` or `@`
* Element-wise matrix addition and subtraction
* Static type and shape checking
* Matrix generators such as `zeros()` and `random()`
* Element-wise activation functions such as `relu()` and `sigmoid()`
* Mean Squared Error evaluation with `mse_loss()`
* Dataset loading with `read_csv()`
* Two-dimensional matrix slicing
* Matrix transposition
* Single-line comments using `//`
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

The following example demonstrates a simple one-layer AI/ML forward pass:

```qlang
// Read a dataset from a CSV file with 2 rows and 2 columns
let X = read_csv(2, 2);

// Initialize the weight matrix
let W = [
    [0.5, 0.1],
    [-0.2, 0.8]
];

// Define the target values
let Target = [
    [1.0, 0.0],
    [0.0, 1.0]
];

// Perform matrix multiplication and apply sigmoid activation
let Z = X * W;
let Pred = sigmoid(Z);

// Calculate the Mean Squared Error loss
let loss = mse_loss(Pred, Target);

print(loss);
```

## Architecture

QLang uses a modular compiler pipeline:

```text
QLang Source (.ql)
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

QLang provides built-in support for numerical data structures, including one-dimensional vectors and two-dimensional matrices.

Matrix multiplication can be written using either the `*` or `@` operator:

```qlang
let C = A * B;
let D = A @ B;
```

The `*` and `@` operators perform matrix multiplication when applied to compatible matrices. Matrix addition and subtraction are performed element-wise:

```qlang
let Sum = A + B;
let Difference = A - B;
```

Matrices must have compatible dimensions for multiplication and matching dimensions for element-wise addition or subtraction.

### Static Type and Shape Checking

Matrix dimensions and types are validated during compilation to help detect invalid operations before execution.

For example, multiplying a `2 × 3` matrix by a `3 × 2` matrix produces a `2 × 2` matrix:

```text
Matrix(2,3) * Matrix(3,2) -> Matrix(2,2)
```

Invalid dimensions result in a compile-time error before C code generation.

### Matrix Generators

QLang provides built-in functions for creating commonly used matrices:

```qlang
let Z = zeros(3, 3);
let R = random(2, 4);
```

* `zeros(rows, cols)` creates a matrix initialized with zero values.
* `random(rows, cols)` creates a matrix populated with generated values.

### Activation Functions

QLang includes element-wise activation functions commonly used in neural network computations:

```qlang
let Activated = sigmoid(M);
let Filtered = relu(M);
```

* `relu(M)` applies the Rectified Linear Unit function element-wise.
* `sigmoid(M)` applies the sigmoid function element-wise.

### Loss Evaluation

The built-in `mse_loss()` function calculates the Mean Squared Error between prediction and target matrices:

```qlang
let loss = mse_loss(Pred, Target);
```

The prediction and target matrices must have matching dimensions.

### Dataset I/O

CSV datasets can be loaded using `read_csv(rows, cols)`:

```qlang
let Data = read_csv(10, 5);
```

The function reads the specified number of rows and columns from a CSV file and returns the values as a matrix.

### Matrix Slicing

Two-dimensional matrices can be sliced using row and column ranges:

```qlang
let sub = M[0..2, 1..3];
```

### Matrix Transposition

Matrices can be transposed using the built-in `transpose()` function:

```qlang
let t = transpose(M);
```

### Single-Line Comments

QLang supports single-line comments using the `//` syntax:

```qlang
// This is a single-line comment
let M = zeros(2, 2);
```

### C Code Generation

QLang translates source programs into C code as an intermediate compilation step. The generated code includes the required runtime helpers for matrix operations, activations, loss evaluation, and related numerical functionality.

### Native Executables

The generated C code is compiled into a native executable using the bundled TinyCC toolchain.

## Status

QLang is an active early-stage domain-specific programming language and compiler.

**Phase 1: Foundational Compiler and Matrix Engine** is complete. This phase includes parsing, static type and shape checking, C code generation, matrix operations, matrix generators, activation functions, dataset loading, loss evaluation, and single-line comments.

**Phase 2: Autodiff Engine, Gradient Computation, and SGD Optimizer** is currently under development.

The language, compiler, and standard library may continue to evolve as development progresses.

## Contributing

Contributions, experiments, and feedback are welcome.

To contribute:

1. Fork the repository.

2. Create a branch for your changes:

   ```bash
   git checkout -b feat/your-feature
   ```

3. Implement and test your changes:

   ```bash
   cargo test
   cargo run -- run test.ql
   ```

4. Open a pull request with a clear description.

For compiler development, refer to the source code and issue tracker.

## License

QLang is licensed under the MIT License.

The full license text is available in the [LICENSE](LICENSE) file.
