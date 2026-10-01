//! Runtime generation modules.

pub mod autodiff;
pub mod helpers;
pub mod opencl;
pub mod tensor;

pub use autodiff::emit_c_autodiff_helpers;
pub use helpers::emit_c_runtime_helpers;
pub use opencl::emit_opencl_runtime_helpers;
pub use tensor::emit_c_tensor_runtime;