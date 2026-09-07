use ql_ast::*;
use ql_checker::{ResolvedType, TypeChecker};

pub struct CodeGenerator {
    var_counter: usize,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator { var_counter: 0 }
    }

    fn new_temp(&mut self) -> String {
        self.var_counter += 1;
        format!("t{}", self.var_counter)
    }

    pub fn generate(&mut self, program: &Program, checker: &TypeChecker) -> String {
        println!("\n--- Generating C Target Code ---");
        let mut code = String::new();

        code.push_str("#include <stdio.h>\n");
        code.push_str("#include <stdlib.h>\n");
        code.push_str("#include <string.h>\n");
        code.push_str("#include <math.h>\n");
        code.push_str("#include <time.h>\n\n");

        // Runtime Helpers
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

        code.push_str("int main() {\n");
        code.push_str("    srand(12345);\n");

        for stmt in &program.statements {
            match stmt {
                Statement::Let { name, ty: _, value } => {
                    let val_var = self.generate_expr(value, &mut code, checker);
                    let ty = checker.infer_expression_type(value);

                    match ty {
                        ResolvedType::Vector { len, elem: _ } => {
                            code.push_str(&format!("    double {}[{}] = {{0}};\n", name, len));
                            code.push_str(&format!(
                                "    for(int i=0; i<{}; i++) {}[i] = {}[i];\n",
                                len, name, val_var
                            ));
                        }
                        ResolvedType::Matrix { rows, cols, elem: _ } => {
                            let total = rows * cols;
                            code.push_str(&format!("    double {}[{}] = {{0}};\n", name, total));
                            code.push_str(&format!(
                                "    for(int i=0; i<{}; i++) {}[i] = {}[i];\n",
                                total, name, val_var
                            ));
                        }
                        ResolvedType::F64 | ResolvedType::Dec => {
                            code.push_str(&format!("    double {} = {};\n", name, val_var));
                        }
                        _ => {}
                    }
                }
                Statement::Expression(expr) => {
                    self.generate_expr(expr, &mut code, checker);
                }
            }
        }

        code.push_str("    return 0;\n");
        code.push_str("}\n");

        println!("[QLC] C code generated in 'output.c'");
        code
    }

    fn generate_expr(&mut self, expr: &Expr, code: &mut String, checker: &TypeChecker) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::Decimal(d) => d.clone(),
            Expr::Variable(v) => v.clone(),
            Expr::Vector(elems) => {
                let temp = self.new_temp();
                let len = elems.len();
                code.push_str(&format!("    double {}[{}] = {{", temp, len));
                for (i, elem) in elems.iter().enumerate() {
                    let val = self.generate_expr(elem, code, checker);
                    code.push_str(&val);
                    if i < len - 1 {
                        code.push_str(", ");
                    }
                }
                code.push_str("};\n");
                temp
            }
            Expr::Matrix(rows) => {
                let temp = self.new_temp();
                let r_cnt = rows.len();
                let c_cnt = if r_cnt > 0 { rows[0].len() } else { 0 };
                let total = r_cnt * c_cnt;

                code.push_str(&format!("    double {}[{}] = {{", temp, total));
                let mut first = true;
                for row in rows {
                    for elem in row {
                        if !first {
                            code.push_str(", ");
                        }
                        let val = self.generate_expr(elem, code, checker);
                        code.push_str(&val);
                        first = false;
                    }
                }
                code.push_str("};\n");
                temp
            }
            Expr::Slice { target, start, end } => {
                let target_var = self.generate_expr(target, code, checker);
                let slice_len = end - start;
                let temp = self.new_temp();

                code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, slice_len));
                code.push_str(&format!(
                    "    vec_slice({}, {}, {}, {});\n",
                    temp, target_var, start, end
                ));
                temp
            }
            Expr::MatrixSlice { target, r_start, r_end, c_start, c_end } => {
                let target_var = self.generate_expr(target, code, checker);
                let target_ty = checker.infer_expression_type(target);

                if let ResolvedType::Matrix { rows: _, cols: in_cols, elem: _ } = target_ty {
                    let out_rows = r_end - r_start;
                    let out_cols = c_end - c_start;
                    let total = out_rows * out_cols;

                    let temp = self.new_temp();
                    code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, total));
                    code.push_str(&format!(
                        "    mat_slice({}, {}, {}, {}, {}, {}, {});\n",
                        temp, target_var, in_cols, r_start, r_end, c_start, c_end
                    ));
                    temp
                } else {
                    panic!("[CODEGEN ERROR] Expected Matrix target for 2D slice");
                }
            }
            Expr::Binary { op, left, right } => match op {
                BinaryOp::MatMul => {
                    let left_var = self.generate_expr(left, code, checker);
                    let right_var = self.generate_expr(right, code, checker);

                    let left_ty = checker.infer_expression_type(left);
                    let right_ty = checker.infer_expression_type(right);

                    match (left_ty, right_ty) {
                        (
                            ResolvedType::Vector { len: v_len, elem: _ },
                            ResolvedType::Matrix { rows: _, cols: m_cols, elem: _ },
                        ) => {
                            let temp = self.new_temp();
                            code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, m_cols));
                            code.push_str(&format!(
                                "    vec_mat_mul({}, {}, {}, {}, {});\n",
                                temp, left_var, right_var, v_len, m_cols
                            ));
                            temp
                        }
                        (
                            ResolvedType::Matrix { rows: r1, cols: c1, elem: _ },
                            ResolvedType::Matrix { rows: _r2, cols: c2, elem: _ },
                        ) => {
                            let total = r1 * c2;
                            let temp = self.new_temp();
                            code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, total));
                            code.push_str(&format!(
                                "    mat_mat_mul({}, {}, {}, {}, {}, {});\n",
                                temp, left_var, right_var, r1, c1, c2
                            ));
                            temp
                        }
                        _ => panic!("[CODEGEN ERROR] Unsupported MatMul operand types"),
                    }
                }
                BinaryOp::Pipe => {
                    let left_var = self.generate_expr(left, code, checker);
                    let left_ty = checker.infer_expression_type(left);

                    if let Expr::Call { callee, args: _ } = &**right {
                        if callee == "relu" {
                            if let ResolvedType::Vector { len, elem: _ } = left_ty {
                                code.push_str(&format!("    vec_relu({}, {});\n", left_var, len));
                            }
                        }
                    }
                    left_var
                }
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                    let left_var = self.generate_expr(left, code, checker);
                    let right_var = self.generate_expr(right, code, checker);

                    let left_ty = checker.infer_expression_type(left);
                    let right_ty = checker.infer_expression_type(right);

                    let op_char = match op {
                        BinaryOp::Add => '+',
                        BinaryOp::Sub => '-',
                        BinaryOp::Mul => '*',
                        BinaryOp::Div => '/',
                        _ => unreachable!(),
                    };

                    let temp = self.new_temp();
                    match (&left_ty, &right_ty) {
                        (
                            ResolvedType::Matrix { rows: r1, cols: c1, elem: _ },
                            ResolvedType::Matrix { rows: _r2, cols: c2, elem: _ },
                        ) if matches!(op, BinaryOp::Mul) => {
                            let total = r1 * c2;
                            code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, total));
                            code.push_str(&format!(
                                "    mat_mat_mul({}, {}, {}, {}, {}, {});\n",
                                temp, left_var, right_var, r1, c1, c2
                            ));
                            temp
                        }
                        (
                            ResolvedType::Matrix { rows: r1, cols: c1, elem: _ },
                            ResolvedType::Matrix { rows: _r2, cols: _c2, elem: _ },
                        ) if matches!(op, BinaryOp::Add | BinaryOp::Sub) => {
                            let total = r1 * c1;
                            code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, total));
                            code.push_str(&format!(
                                "    vec_elem_op({}, {}, {}, {}, '{}');\n",
                                temp, left_var, right_var, total, op_char
                            ));
                            temp
                        }
                        (ResolvedType::Vector { len, elem: _ }, ResolvedType::F64) => {
                            code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, len));
                            code.push_str(&format!(
                                "    vec_broadcast_op({}, {}, {}, {}, '{}', 0);\n",
                                temp, left_var, right_var, len, op_char
                            ));
                            temp
                        }
                        (ResolvedType::F64, ResolvedType::Vector { len, elem: _ }) => {
                            code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, len));
                            code.push_str(&format!(
                                "    vec_broadcast_op({}, {}, {}, {}, '{}', 1);\n",
                                temp, right_var, left_var, len, op_char
                            ));
                            temp
                        }
                        (ResolvedType::Matrix { rows, cols, elem: _ }, ResolvedType::F64) => {
                            let total = rows * cols;
                            code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, total));
                            code.push_str(&format!(
                                "    vec_broadcast_op({}, {}, {}, {}, '{}', 0);\n",
                                temp, left_var, right_var, total, op_char
                            ));
                            temp
                        }
                        (ResolvedType::F64, ResolvedType::Matrix { rows, cols, elem: _ }) => {
                            let total = rows * cols;
                            code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, total));
                            code.push_str(&format!(
                                "    vec_broadcast_op({}, {}, {}, {}, '{}', 1);\n",
                                temp, right_var, left_var, total, op_char
                            ));
                            temp
                        }
                        _ => format!("({} {} {})", left_var, op_char, right_var),
                    }
                }
                BinaryOp::ElementMul | BinaryOp::ElementDiv | BinaryOp::ElementAdd | BinaryOp::ElementSub => {
                    let left_var = self.generate_expr(left, code, checker);
                    let right_var = self.generate_expr(right, code, checker);

                    let left_ty = checker.infer_expression_type(left);
                    let op_char = match op {
                        BinaryOp::ElementMul => '*',
                        BinaryOp::ElementDiv => '/',
                        BinaryOp::ElementAdd => '+',
                        BinaryOp::ElementSub => '-',
                        _ => unreachable!(),
                    };

                    let temp = self.new_temp();
                    match left_ty {
                        ResolvedType::Vector { len, elem: _ } => {
                            code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, len));
                            code.push_str(&format!(
                                "    vec_elem_op({}, {}, {}, {}, '{}');\n",
                                temp, left_var, right_var, len, op_char
                            ));
                        }
                        ResolvedType::Matrix { rows, cols, elem: _ } => {
                            let total = rows * cols;
                            code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, total));
                            code.push_str(&format!(
                                "    vec_elem_op({}, {}, {}, {}, '{}');\n",
                                temp, left_var, right_var, total, op_char
                            ));
                        }
                        _ => panic!("[CODEGEN ERROR] Element-wise operation on non-array type"),
                    }
                    temp
                }
            },
            Expr::Call { callee, args } => {
                if callee == "print" && !args.is_empty() {
                    let arg_var = self.generate_expr(&args[0], code, checker);
                    let arg_type = checker.infer_expression_type(&args[0]);

                    match arg_type {
                        ResolvedType::Vector { len, elem: _ } => {
                            code.push_str(&format!("    print_vector({}, {});\n", arg_var, len));
                        }
                        ResolvedType::Matrix { rows, cols, elem: _ } => {
                            code.push_str(&format!("    print_matrix({}, {}, {});\n", arg_var, rows, cols));
                        }
                        ResolvedType::F64 | ResolvedType::Dec => {
                            code.push_str(&format!("    printf(\"%.4f\\n\", {});\n", arg_var));
                        }
                        _ => {}
                    }
                }
                if callee == "read_csv" && args.len() == 2 {
                    if let (Expr::Number(r), Expr::Number(c)) = (&args[0], &args[1]) {
                        let rows = *r as usize;
                        let cols = *c as usize;
                        let total = rows * cols;
                        let temp = self.new_temp();
                        code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, total));
                        code.push_str(&format!("    read_csv_file({}, {}, {}, \"data.csv\");\n", temp, rows, cols));
                        return temp;
                    }
                }
                if callee == "mse_loss" && args.len() == 2 {
                    let pred_var = self.generate_expr(&args[0], code, checker);
                    let target_var = self.generate_expr(&args[1], code, checker);
                    let pred_ty = checker.infer_expression_type(&args[0]);

                    if let ResolvedType::Matrix { rows, cols, elem: _ } = pred_ty {
                        let total = rows * cols;
                        let temp = self.new_temp();
                        code.push_str(&format!("    double {} = mat_mse_loss({}, {}, {});\n", temp, pred_var, target_var, total));
                        return temp;
                    }
                }
                if callee == "relu" && !args.is_empty() {
                    let arg_var = self.generate_expr(&args[0], code, checker);
                    let arg_type = checker.infer_expression_type(&args[0]);

                    if let ResolvedType::Matrix { rows, cols, elem: _ } = arg_type {
                        let total = rows * cols;
                        let temp = self.new_temp();
                        code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, total));
                        code.push_str(&format!("    mat_relu({}, {}, {});\n", temp, arg_var, total));
                        return temp;
                    }
                }
                if callee == "sigmoid" && !args.is_empty() {
                    let arg_var = self.generate_expr(&args[0], code, checker);
                    let arg_type = checker.infer_expression_type(&args[0]);

                    if let ResolvedType::Matrix { rows, cols, elem: _ } = arg_type {
                        let total = rows * cols;
                        let temp = self.new_temp();
                        code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, total));
                        code.push_str(&format!("    mat_sigmoid({}, {}, {});\n", temp, arg_var, total));
                        return temp;
                    }
                }
                if callee == "transpose" && !args.is_empty() {
                    let arg_var = self.generate_expr(&args[0], code, checker);
                    let arg_type = checker.infer_expression_type(&args[0]);

                    if let ResolvedType::Matrix { rows, cols, elem: _ } = arg_type {
                        let total = rows * cols;
                        let temp = self.new_temp();
                        code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, total));
                        code.push_str(&format!(
                            "    mat_transpose({}, {}, {}, {});\n",
                            temp, arg_var, rows, cols
                        ));
                        return temp;
                    }
                }
                if callee == "zeros" && args.len() == 2 {
                    if let (Expr::Number(r), Expr::Number(c)) = (&args[0], &args[1]) {
                        let rows = *r as usize;
                        let cols = *c as usize;
                        let total = rows * cols;
                        let temp = self.new_temp();
                        code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, total));
                        code.push_str(&format!("    mat_zeros({}, {});\n", temp, total));
                        return temp;
                    }
                }
                if callee == "random" && args.len() == 2 {
                    if let (Expr::Number(r), Expr::Number(c)) = (&args[0], &args[1]) {
                        let rows = *r as usize;
                        let cols = *c as usize;
                        let total = rows * cols;
                        let temp = self.new_temp();
                        code.push_str(&format!("    double {}[{}] = {{0}};\n", temp, total));
                        code.push_str(&format!("    mat_random({}, {});\n", temp, total));
                        return temp;
                    }
                }
                String::new()
            }
        }
    }
}