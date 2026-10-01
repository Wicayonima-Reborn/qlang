//! Type resolution representations for QLang semantic analysis.

/// Fully resolved concrete data types produced after semantic pass.
#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedType {
    F64,
    Dec,
    Vector {
        elem: Box<ResolvedType>,
        len: usize,
    },
    Matrix {
        elem: Box<ResolvedType>,
        rows: usize,
        cols: usize,
    },
    /// Represents non-returning calls or uninitialized bindings.
    Void,
}