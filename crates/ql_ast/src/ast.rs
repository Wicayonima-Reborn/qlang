//! Abstract Syntax Tree node definitions.

use crate::ops::BinaryOp;
use crate::types::TypeAnnotation;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Decimal(String),
    Variable(String),
    Vector(Vec<Expr>),
    Matrix(Vec<Vec<Expr>>),

    /// 1D Slicing: `arr[start..end]`
    Slice {
        target: Box<Expr>,
        start: usize,
        end: usize,
    },

    /// 2D Sub-matrix slicing: `mat[r_start..r_end, c_start..c_end]`
    MatrixSlice {
        target: Box<Expr>,
        r_start: usize,
        r_end: usize,
        c_start: usize,
        c_end: usize,
    },

    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    /// Function or intrinsic invocation (e.g. `sigmoid(x)`, `read_csv(...)`)
    Call {
        callee: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    /// Variable declaration or binding: `let x: Matrix(2, 2) = ...`
    Let {
        name: String,
        ty: Option<TypeAnnotation>,
        value: Expr,
    },

    /// Top-level bare expression evaluation
    Expression(Expr),

    /// First-class training loop directive: `train(loss, lr, epochs)`
    Train {
        loss_var: String,
        lr: f64,
        epochs: usize,
    },
}

/// Root node representing a parsed QLang source file.
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}