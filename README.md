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

QLang (Quant Lang) is a lightweight programming language designed for numerical computation, tensor and matrix operations, neural network primitives, and automatic differentiation.

QLang programs are statically checked for type and shape compatibility, transpiled to C with numerical and autodiff runtime support, and compiled into native executables using a bundled TinyCC toolchain.

## Why QLang?

QLang is designed to keep numerical and AI/ML code concise while providing compile-time validation for matrix operations and built-in support for model training.

* Linear algebra operations for matrices and vectors
* Matrix multiplication using `*` or `@`
* Element-wise matrix addition and subtraction
* Static type and shape checking
* Matrix generators such as `zeros()` and `random()`
* Element-wise activation functions such as `relu()` and `sigmoid()`
* Mean Squared Error with `mse_loss()`
* Dataset loading with `read_csv()`
* Two-dimensional matrix slicing
* Matrix transposition
* Reverse-mode automatic differentiation
* SGD optimization through `train()`
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

The following example demonstrates a simple AI/ML training workflow in QLang:

```qlang
// Load a dataset
let X = read_csv(2, 2);

// Initialize model weights
let W = [
    [0.5, 0.1],
    [-0.2, 0.8]
];

// Define target values
let Target = [
    [1.0, 0.0],
    [0.0, 1.0]
];

// Forward pass
let Z = X @ W;
let Pred = sigmoid(Z);

// Calculate loss
let loss = mse_loss(Pred, Target);

// Train using reverse-mode autodiff and SGD
train(loss, 0.5, 50);

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
       ├── Numerical Runtime
       └── Autodiff Runtime
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

QLang provides built-in support for one-dimensional vectors and two-dimensional matrices.

Matrix multiplication can be written using either the `*` or `@` operator:

```qlang
let C = A * B;
let D = A @ B;
```

Matrix addition and subtraction are performed element-wise:

```qlang
let Sum = A + B;
let Difference = A - B;
```

Matrices must have compatible dimensions for multiplication and matching dimensions for element-wise operations.

### Static Type and Shape Checking

Matrix dimensions and types are validated during compilation to detect invalid operations before execution.

For example:

```text
Matrix(2,3) * Matrix(3,2) -> Matrix(2,2)
```

Invalid dimensions result in a compile-time error before C code generation.

### Matrix Generators

QLang provides built-in functions for creating matrices:

```qlang
let Z = zeros(3, 3);
let R = random(2, 4);
```

- `zeros(rows, cols)` creates a zero-initialized matrix.
- `random(rows, cols)` creates a matrix populated with generated values.

### Activation Functions

QLang includes element-wise activation functions commonly used in neural network computations:

```qlang
let Activated = sigmoid(M);
let Filtered = relu(M);
```

- `relu(M)` applies the Rectified Linear Unit function element-wise.
- `sigmoid(M)` applies the sigmoid function element-wise.

### Loss Evaluation

The built-in `mse_loss()` function calculates Mean Squared Error between prediction and target matrices:

```qlang
let loss = mse_loss(Pred, Target);
```

The prediction and target matrices must have matching dimensions.

### Dataset I/O

CSV datasets can be loaded using `read_csv(rows, cols)`:

```qlang
let Data = read_csv(10, 5);
```

The function reads the specified dataset dimensions and returns the values as a matrix.

### Automatic Differentiation

QLang includes a reverse-mode automatic differentiation engine for supported numerical operations.

Gradients are generated as part of the C runtime during code generation, allowing QLang programs to perform backward propagation without manually defining gradient calculations.

### SGD Training

The built-in `train()` statement provides an integrated training loop using Stochastic Gradient Descent:

```qlang
train(loss, learning_rate, epochs);
```

For example:

```qlang
train(loss, 0.5, 50);
```

The compiler generates the required backward-pass and optimization logic for the supported operations.

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

QLang supports single-line comments using `//`:

```qlang
// Initialize a matrix
let M = zeros(2, 2);
```

### C Code Generation

QLang translates validated source programs into C code as an intermediate compilation step.

Generated code includes the runtime helpers required for supported matrix operations, activation functions, loss evaluation, and automatic differentiation.

### Native Executables

The generated C code is compiled into a native executable using the bundled TinyCC toolchain.

## Status

QLang is an active early-stage domain-specific programming language and compiler.

**Phase 1: Foundational Compiler and Matrix Engine — COMPLETED**

Includes the lexer, parser, AST, static type and shape checking, C code generation, matrix operations, matrix generators, activation functions, dataset loading, loss evaluation, slicing, and comments.

**Phase 2: Autodiff Engine, Gradient Computation, and SGD Optimizer — COMPLETED**

Includes the `train()` statement, reverse-mode automatic differentiation, gradient runtime helpers, backward propagation, and automated SGD training loop generation.

**Phase 3: Model Persistence & Generalized Computation Graph — IN PROGRESS**

Planned features include:

- Model weight export and import
- `save_weights()` and `load_weights()`
- Dynamic multi-layer computation graph tracing
- Expanded numerical standard library
- Additional optimizers such as Adam
- Additional neural network operations such as Softmax

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
