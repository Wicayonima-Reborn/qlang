//! AST Traversal interface for IR construction and semantic analysis.

use crate::ast::{Expr, Program, Statement};

/// Standard visitor pattern trait. Implement this to walk the AST
/// without cluttering node data structures.
pub trait AstVisitor<T> {
    fn visit_program(&mut self, program: &Program) -> T;
    fn visit_statement(&mut self, stmt: &Statement) -> T;
    fn visit_expr(&mut self, expr: &Expr) -> T;
}