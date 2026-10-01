//! Runtime generation modules.

pub mod autodiff;
pub mod helpers;
pub mod tensor;

pub use tensor::emit_c_tensor_runtime;