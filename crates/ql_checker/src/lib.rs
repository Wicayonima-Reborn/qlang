//! `ql_checker` - Semantic analysis, symbol resolution, and shape checker for QLang.

pub mod checker;
pub mod env;
pub mod shapes;
pub mod types;

pub use checker::TypeChecker;
pub use env::SymbolTable;
pub use types::ResolvedType;