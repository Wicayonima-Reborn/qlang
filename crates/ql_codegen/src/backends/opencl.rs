//! Intel iGPU & Generic OpenCL Target Code Generator Backend.

use crate::backends::CodeBackend;
use ql_ast::Program;
use ql_checker::TypeChecker;

pub struct OpenCLBackend;

impl OpenCLBackend {
    pub fn new() -> Self {
        Self
    }
}

impl CodeBackend for OpenCLBackend {
    fn generate(&mut self, _program: &Program, _checker: &TypeChecker) -> String {
        unimplemented!("[OPENCL BACKEND] Intel OpenCL backend code generation will be implemented here.")
    }
}