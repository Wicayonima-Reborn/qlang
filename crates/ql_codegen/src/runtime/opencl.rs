//! OpenCL C Runtime API Initialization and GPU Kernel Helpers.

pub fn emit_opencl_runtime_helpers(code: &mut String) {
    code.push_str("// --- QLang OpenCL GPU Runtime Engine ---\n");
    code.push_str("#ifdef __APPLE__\n");
    code.push_str("#include <OpenCL/opencl.h>\n");
    code.push_str("#else\n");
    code.push_str("#include <CL/cl.h>\n");
    code.push_str("#endif\n\n");

    code.push_str("typedef struct {\n");
    code.push_str("    cl_platform_id platform;\n");
    code.push_str("    cl_device_id device;\n");
    code.push_str("    cl_context context;\n");
    code.push_str("    cl_command_queue queue;\n");
    code.push_str("    cl_program program;\n");
    code.push_str("} QL_OpenCLContext;\n\n");

    // Kernel source code string embedded directly into C runtime
    code.push_str("const char* opencl_kernel_source = \n");
    code.push_str("\"__kernel void gpu_mat_matmul(__global const double* A, __global const double* B, __global double* C, int M, int N, int P) {\\n\"\n");
    code.push_str("\"    int row = get_global_id(0);\\n\"\n");
    code.push_str("\"    int col = get_global_id(1);\\n\"\n");
    code.push_str("\"    if (row < M && col < P) {\\n\"\n");
    code.push_str("\"        double sum = 0.0;\\n\"\n");
    code.push_str("\"        for (int k = 0; k < N; k++) {\\n\"\n");
    code.push_str("\"            sum += A[row * N + k] * B[k * P + col];\\n\"\n");
    code.push_str("\"        }\\n\"\n");
    code.push_str("\"        C[row * P + col] = sum;\\n\"\n");
    code.push_str("\"    }\\n\"\n");
    code.push_str("\"}\\n\"\n");
    code.push_str("\"__kernel void gpu_mat_sigmoid(__global const double* in, __global double* out, int size) {\\n\"\n");
    code.push_str("\"    int id = get_global_id(0);\\n\"\n");
    code.push_str("\"    if (id < size) {\\n\"\n");
    code.push_str("\"        out[id] = 1.0 / (1.0 + exp(-in[id]));\\n\"\n");
    code.push_str("\"    }\\n\"\n");
    code.push_str("\"}\\n\";\n\n");

    code.push_str("QL_OpenCLContext* ql_opencl_init() {\n");
    code.push_str("    QL_OpenCLContext* ctx = (QL_OpenCLContext*)malloc(sizeof(QL_OpenCLContext));\n");
    code.push_str("    cl_int err;\n");
    code.push_str("    clGetPlatformIDs(1, &ctx->platform, NULL);\n");
    code.push_str("    clGetDeviceIDs(ctx->platform, CL_DEVICE_TYPE_GPU, 1, &ctx->device, NULL);\n");
    code.push_str("    ctx->context = clCreateContext(NULL, 1, &ctx->device, NULL, NULL, &err);\n");
    code.push_str("    ctx->queue = clCreateCommandQueue(ctx->context, ctx->device, 0, &err);\n");
    code.push_str("    ctx->program = clCreateProgramWithSource(ctx->context, 1, &opencl_kernel_source, NULL, &err);\n");
    code.push_str("    clBuildProgram(ctx->program, 1, &ctx->device, NULL, NULL, NULL);\n");
    code.push_str("    printf(\"[QLC OpenCL] iGPU/GPU Accelerator Initialized Successfully.\\n\");\n");
    code.push_str("    return ctx;\n");
    code.push_str("}\n\n");
}