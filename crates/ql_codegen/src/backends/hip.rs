//! AMD Radeon / ROCm HIP Target Code Generator Backend.

use crate::backends::CodeBackend;
use ql_ast::Program;
use ql_checker::TypeChecker;

pub struct HipBackend;

impl HipBackend {
    pub fn new() -> Self {
        Self
    }
}

impl CodeBackend for HipBackend {
    fn generate(&mut self, _program: &Program, _checker: &TypeChecker) -> String {
        unimplemented!("[HIP BACKEND] AMD ROCm/HIP backend code generation will be implemented here.")
    }
}