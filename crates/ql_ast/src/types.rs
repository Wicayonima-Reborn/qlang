//! Type definitions and annotations for QLang AST.

/// Primitive scalar precision types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DType {
    Dec,
    F64,
    I64,
}

/// Type hints/annotations supplied by user or inferred during checking.
/// Note: Tensor(Vec<usize>) is our bridge to arbitrary N-D arrays.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeAnnotation {
    Dec,
    F64,
    Vector(usize),
    Matrix(usize, usize),
    /// Experimental N-Dimensional shape layout: [d0, d1, ..., dn]
    Tensor(Vec<usize>),
}