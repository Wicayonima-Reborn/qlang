//! Apple Silicon (macOS Metal API) Target Code Generator Backend.

use crate::backends::CodeBackend;
use ql_ast::Program;
use ql_checker::TypeChecker;

pub struct MetalBackend;

impl MetalBackend {
    pub fn new() -> Self {
        Self
    }
}

impl CodeBackend for MetalBackend {
    fn generate(&mut self, _program: &Program, _checker: &TypeChecker) -> String {
        unimplemented!("[METAL BACKEND] Apple Silicon Metal backend code generation will be implemented here.")
    }
}