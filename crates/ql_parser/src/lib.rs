//! `ql_parser` - Syntactic parser producing Abstract Syntax Trees from token streams.

pub mod expr;
pub mod parser;
pub mod stmt;

pub use parser::Parser;