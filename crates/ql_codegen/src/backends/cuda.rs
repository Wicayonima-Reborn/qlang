//! NVIDIA RTX / CUDA Target Code Generator Backend.

use crate::backends::CodeBackend;
use ql_ast::Program;
use ql_checker::TypeChecker;

pub struct CudaBackend;

impl CudaBackend {
    pub fn new() -> Self {
        Self
    }
}

impl CodeBackend for CudaBackend {
    fn generate(&mut self, _program: &Program, _checker: &TypeChecker) -> String {
        unimplemented!("[CUDA BACKEND] NVIDIA CUDA backend code generation will be implemented here.")
    }
}