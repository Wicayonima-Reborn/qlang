//! C-Runtime autodiff backward pass routines and SGD optimizer functions.

pub fn emit_c_autodiff_helpers(code: &mut String) {
    code.push_str("// --- Autodiff Gradient Helpers ---\n");
    code.push_str("void mse_loss_backward(const double* pred, const double* target, double* dPred, int size) {\n");
    code.push_str("    for(int i = 0; i < size; i++) dPred[i] = (2.0 / (double)size) * (pred[i] - target[i]);\n");
    code.push_str("}\n\n");

    code.push_str("void sigmoid_backward(const double* dPred, const double* pred, double* dZ, int size) {\n");
    code.push_str("    for(int i = 0; i < size; i++) dZ[i] = dPred[i] * pred[i] * (1.0 - pred[i]);\n");
    code.push_str("}\n\n");

    code.push_str("void relu_backward(const double* dOut, const double* in, double* dIn, int size) {\n");
    code.push_str("    for(int i = 0; i < size; i++) dIn[i] = (in[i] > 0.0) ? dOut[i] : 0.0;\n");
    code.push_str("}\n\n");

    code.push_str("void sgd_update(double* param, const double* dParam, double lr, int size) {\n");
    code.push_str("    for(int i = 0; i < size; i++) param[i] -= lr * dParam[i];\n");
    code.push_str("}\n\n");
}