//! `ql_codegen` - Multi-backend code generation workspace for QLang compiler.

pub mod backends;
pub mod runtime;
pub mod ir;

pub use backends::cpu::CpuBackend;
pub use backends::cuda::CudaBackend;
pub use backends::hip::HipBackend;
pub use backends::metal::MetalBackend;
pub use backends::opencl::OpenCLBackend;
pub use backends::CodeBackend;

use ql_ast::Program;
use ql_checker::TypeChecker;

/// Primary runner facing compiler CLI
pub struct CodeGenerator {
    backend: Box<dyn CodeBackend>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            backend: Box::new(CpuBackend::new()),
        }
    }

    pub fn with_backend(backend: Box<dyn CodeBackend>) -> Self {
        Self { backend }
    }

    pub fn generate(&mut self, program: &Program, checker: &TypeChecker) -> String {
        self.backend.generate(program, checker)
    }
}