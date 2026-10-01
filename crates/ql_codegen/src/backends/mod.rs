//! Multi-hardware backend interfaces and abstraction traits.

pub mod cpu;
pub mod cuda;
pub mod hip;
pub mod metal;
pub mod opencl;

pub use cpu::CpuBackend;

use ql_ast::Program;
use ql_checker::TypeChecker;

/// Common code generation contract for target architectures.
pub trait CodeBackend {
    fn generate(&mut self, program: &Program, checker: &TypeChecker) -> String;
}