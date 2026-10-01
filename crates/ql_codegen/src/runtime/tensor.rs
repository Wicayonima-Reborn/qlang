//! C Runtime definitions for QL_Tensor memory management and device buffers.

pub fn emit_c_tensor_runtime(code: &mut String) {
    code.push_str("// --- QLang Tensor Memory Management Runtime ---\n");
    code.push_str("typedef enum {\n");
    code.push_str("    DEV_CPU = 0,\n");
    code.push_str("    DEV_OPENCL = 1,\n");
    code.push_str("    DEV_CUDA = 2\n");
    code.push_str("} QL_DeviceType;\n\n");

    code.push_str("typedef struct {\n");
    code.push_str("    double* data;         // Host RAM memory pointer\n");
    code.push_str("    void* device_ptr;     // Device VRAM pointer (cl_mem or CUdeviceptr)\n");
    code.push_str("    int rows;\n");
    code.push_str("    int cols;\n");
    code.push_str("    int total_size;\n");
    code.push_str("    QL_DeviceType device;\n");
    code.push_str("} QL_Tensor;\n\n");

    code.push_str("QL_Tensor* ql_tensor_alloc(int rows, int cols) {\n");
    code.push_str("    QL_Tensor* t = (QL_Tensor*)malloc(sizeof(QL_Tensor));\n");
    code.push_str("    t->rows = rows;\n");
    code.push_str("    t->cols = cols;\n");
    code.push_str("    t->total_size = rows * cols;\n");
    code.push_str("    t->data = (double*)calloc(t->total_size, sizeof(double));\n");
    code.push_str("    t->device_ptr = NULL;\n");
    code.push_str("    t->device = DEV_CPU;\n");
    code.push_str("    return t;\n");
    code.push_str("}\n\n");

    code.push_str("void ql_tensor_free(QL_Tensor* t) {\n");
    code.push_str("    if(!t) return;\n");
    code.push_str("    if(t->data) free(t->data);\n");
    code.push_str("    // Note: device_ptr cleanup will be handled by GPU backend context\n");
    code.push_str("    free(t);\n");
    code.push_str("}\n\n");
}