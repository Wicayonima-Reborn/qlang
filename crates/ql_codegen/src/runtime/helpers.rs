//! C-Runtime helper function definitions emitted into generated binaries.

pub fn emit_c_runtime_helpers(code: &mut String) {
    code.push_str("// QLang C Runtime Helpers\n");
    code.push_str("void print_vector(const double* v, int len) {\n");
    code.push_str("    printf(\"[\");\n");
    code.push_str("    for(int i = 0; i < len; i++) {\n");
    code.push_str("        printf(\"%.2f%s\", v[i], (i == len - 1) ? \"\" : \", \");\n");
    code.push_str("    }\n");
    code.push_str("    printf(\"]\\n\");\n");
    code.push_str("}\n\n");

    code.push_str("void print_matrix(const double* m, int rows, int cols) {\n");
    code.push_str("    printf(\"[\\n\");\n");
    code.push_str("    for(int r = 0; r < rows; r++) {\n");
    code.push_str("        printf(\"  [\");\n");
    code.push_str("        for(int c = 0; c < cols; c++) {\n");
    code.push_str("            printf(\"%.2f%s\", m[r * cols + c], (c == cols - 1) ? \"\" : \", \");\n");
    code.push_str("        }\n");
    code.push_str("        printf(\"]\\n\");\n");
    code.push_str("    }\n");
    code.push_str("    printf(\"]\\n\");\n");
    code.push_str("}\n\n");

    code.push_str("void read_csv_file(double* out, int rows, int cols, const char* filename) {\n");
    code.push_str("    FILE* f = fopen(filename, \"r\");\n");
    code.push_str("    if(!f) { printf(\"[RUNTIME ERROR] Failed to open CSV file: %s\\n\", filename); exit(1); }\n");
    code.push_str("    char line[1024];\n");
    code.push_str("    int r = 0;\n");
    code.push_str("    while(fgets(line, sizeof(line), f) && r < rows) {\n");
    code.push_str("        char* tok = strtok(line, \",\");\n");
    code.push_str("        int c = 0;\n");
    code.push_str("        while(tok && c < cols) {\n");
    code.push_str("            out[r * cols + c] = atof(tok);\n");
    code.push_str("            tok = strtok(NULL, \",\");\n");
    code.push_str("            c++;\n");
    code.push_str("        }\n");
    code.push_str("        r++;\n");
    code.push_str("    }\n");
    code.push_str("    fclose(f);\n");
    code.push_str("}\n\n");

    code.push_str("void mat_zeros(double* out, int size) {\n");
    code.push_str("    for(int i = 0; i < size; i++) out[i] = 0.0;\n");
    code.push_str("}\n\n");

    code.push_str("void mat_random(double* out, int size) {\n");
    code.push_str("    for(int i = 0; i < size; i++) out[i] = (double)rand() / (double)RAND_MAX;\n");
    code.push_str("}\n\n");

    code.push_str("void mat_relu(double* out, const double* in, int size) {\n");
    code.push_str("    for(int i = 0; i < size; i++) out[i] = (in[i] < 0.0) ? 0.0 : in[i];\n");
    code.push_str("}\n\n");

    code.push_str("void mat_sigmoid(double* out, const double* in, int size) {\n");
    code.push_str("    for(int i = 0; i < size; i++) out[i] = 1.0 / (1.0 + exp(-in[i]));\n");
    code.push_str("}\n\n");

    code.push_str("double mat_mse_loss(const double* pred, const double* target, int size) {\n");
    code.push_str("    double sum = 0.0;\n");
    code.push_str("    for(int i = 0; i < size; i++) {\n");
    code.push_str("        double diff = pred[i] - target[i];\n");
    code.push_str("        sum += diff * diff;\n");
    code.push_str("    }\n");
    code.push_str("    return sum / (double)size;\n");
    code.push_str("}\n\n");

    code.push_str("void vec_mat_mul(double* out, const double* v, const double* m, int v_len, int m_cols) {\n");
    code.push_str("    for(int j = 0; j < m_cols; j++) {\n");
    code.push_str("        out[j] = 0.0;\n");
    code.push_str("        for(int i = 0; i < v_len; i++) {\n");
    code.push_str("            out[j] += v[i] * m[i * m_cols + j];\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    code.push_str("void mat_mat_mul(double* out, const double* a, const double* b, int m, int n, int p) {\n");
    code.push_str("    for(int i = 0; i < m; i++) {\n");
    code.push_str("        for(int j = 0; j < p; j++) {\n");
    code.push_str("            out[i * p + j] = 0.0;\n");
    code.push_str("            for(int k = 0; k < n; k++) {\n");
    code.push_str("                out[i * p + j] += a[i * n + k] * b[k * p + j];\n");
    code.push_str("            }\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    code.push_str("void vec_relu(double* v, int len) {\n");
    code.push_str("    for(int i = 0; i < len; i++) {\n");
    code.push_str("        if(v[i] < 0.0) v[i] = 0.0;\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    code.push_str("void vec_elem_op(double* out, const double* a, const double* b, int len, char op) {\n");
    code.push_str("    for(int i = 0; i < len; i++) {\n");
    code.push_str("        switch(op) {\n");
    code.push_str("            case '*': out[i] = a[i] * b[i]; break;\n");
    code.push_str("            case '/': out[i] = a[i] / b[i]; break;\n");
    code.push_str("            case '+': out[i] = a[i] + b[i]; break;\n");
    code.push_str("            case '-': out[i] = a[i] - b[i]; break;\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    code.push_str("void vec_slice(double* out, const double* in, int start, int end) {\n");
    code.push_str("    int idx = 0;\n");
    code.push_str("    for(int i = start; i < end; i++) {\n");
    code.push_str("        out[idx++] = in[i];\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    code.push_str("void mat_slice(double* out, const double* in, int in_cols, int r_start, int r_end, int c_start, int c_end) {\n");
    code.push_str("    int idx = 0;\n");
    code.push_str("    for(int r = r_start; r < r_end; r++) {\n");
    code.push_str("        for(int c = c_start; c < c_end; c++) {\n");
    code.push_str("            out[idx++] = in[r * in_cols + c];\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    code.push_str("void mat_transpose(double* out, const double* in, int rows, int cols) {\n");
    code.push_str("    for(int r = 0; r < rows; r++) {\n");
    code.push_str("        for(int c = 0; c < cols; c++) {\n");
    code.push_str("            out[c * rows + r] = in[r * cols + c];\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    code.push_str("void vec_broadcast_op(double* out, const double* v, double s, int len, char op, int scalar_first) {\n");
    code.push_str("    for(int i = 0; i < len; i++) {\n");
    code.push_str("        double a = scalar_first ? s : v[i];\n");
    code.push_str("        double b = scalar_first ? v[i] : s;\n");
    code.push_str("        switch(op) {\n");
    code.push_str("            case '+': out[i] = a + b; break;\n");
    code.push_str("            case '-': out[i] = a - b; break;\n");
    code.push_str("            case '*': out[i] = a * b; break;\n");
    code.push_str("            case '/': out[i] = a / b; break;\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");
}