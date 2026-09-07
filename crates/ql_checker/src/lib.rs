use ql_ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedType {
    F64,
    Dec,
    Vector { elem: Box<ResolvedType>, len: usize },
    Matrix { elem: Box<ResolvedType>, rows: usize, cols: usize },
    Void,
}

pub struct TypeChecker {
    symbol_table: HashMap<String, ResolvedType>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut symbol_table = HashMap::new();
        symbol_table.insert("relu".to_string(), ResolvedType::Void);
        symbol_table.insert("print".to_string(), ResolvedType::Void);
        symbol_table.insert("transpose".to_string(), ResolvedType::Void);
        symbol_table.insert("zeros".to_string(), ResolvedType::Void);
        symbol_table.insert("random".to_string(), ResolvedType::Void);

        TypeChecker { symbol_table }
    }

    pub fn check_program(&mut self, program: &Program) {
        println!("--- Semantic Analysis & Type/Shape Checking ---");
        for stmt in &program.statements {
            self.check_statement(stmt);
        }
    }

    fn check_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let { name, ty: _, value } => {
                let resolved = self.infer_expression_type(value);
                println!("[TYPE CHECK] Variable '{}' resolved as {:?}", name, resolved);
                self.symbol_table.insert(name.clone(), resolved);
            }
            Statement::Expression(expr) => {
                self.infer_expression_type(expr);
            }
        }
    }

    pub fn infer_expression_type(&self, expr: &Expr) -> ResolvedType {
        match expr {
            Expr::Number(_) => ResolvedType::F64,
            Expr::Decimal(_) => ResolvedType::Dec,
            Expr::Variable(name) => self
                .symbol_table
                .get(name)
                .cloned()
                .unwrap_or_else(|| panic!("[COMPILE ERROR] Undefined variable '{}'", name)),
            Expr::Vector(elems) => {
                let len = elems.len();
                let elem_ty = if len > 0 {
                    self.infer_expression_type(&elems[0])
                } else {
                    ResolvedType::F64
                };
                ResolvedType::Vector {
                    elem: Box::new(elem_ty),
                    len,
                }
            }
            Expr::Matrix(rows) => {
                let row_cnt = rows.len();
                let col_cnt = if row_cnt > 0 { rows[0].len() } else { 0 };
                let elem_ty = if row_cnt > 0 && col_cnt > 0 {
                    self.infer_expression_type(&rows[0][0])
                } else {
                    ResolvedType::F64
                };
                ResolvedType::Matrix {
                    elem: Box::new(elem_ty),
                    rows: row_cnt,
                    cols: col_cnt,
                }
            }
            Expr::Slice { target, start, end } => {
                let target_ty = self.infer_expression_type(target);
                if start >= end {
                    panic!("[SHAPE ERROR] Invalid slice range: {}..{}", start, end);
                }
                let slice_len = end - start;

                match target_ty {
                    ResolvedType::Vector { len, elem } => {
                        if *end > len {
                            panic!("[SHAPE ERROR] Slice index {} out of bounds for Vector({})", end, len);
                        }
                        println!("[SHAPE CHECK PASSED] Vector({})[{}..{}] -> Vector({})", len, start, end, slice_len);
                        ResolvedType::Vector { elem, len: slice_len }
                    }
                    _ => panic!("[TYPE ERROR] 1D Slicing is only supported on Vectors"),
                }
            }
            Expr::MatrixSlice { target, r_start, r_end, c_start, c_end } => {
                let target_ty = self.infer_expression_type(target);
                if r_start >= r_end || c_start >= c_end {
                    panic!("[SHAPE ERROR] Invalid matrix slice range");
                }
                let out_rows = r_end - r_start;
                let out_cols = c_end - c_start;

                match target_ty {
                    ResolvedType::Matrix { rows, cols, elem } => {
                        if *r_end > rows || *c_end > cols {
                            panic!("[SHAPE ERROR] Matrix slice indices out of bounds");
                        }
                        println!(
                            "[SHAPE CHECK PASSED] Matrix({},{})[{}..{}, {}..{}] -> Matrix({},{})",
                            rows, cols, r_start, r_end, c_start, c_end, out_rows, out_cols
                        );
                        ResolvedType::Matrix {
                            elem,
                            rows: out_rows,
                            cols: out_cols,
                        }
                    }
                    _ => panic!("[TYPE ERROR] 2D Slicing is only supported on Matrices"),
                }
            }
            Expr::Binary { op, left, right } => match op {
                BinaryOp::MatMul => {
                    let left_ty = self.infer_expression_type(left);
                    let right_ty = self.infer_expression_type(right);

                    match (&left_ty, &right_ty) {
                        (
                            ResolvedType::Vector { len: v_len, elem: _ },
                            ResolvedType::Matrix { rows: m_rows, cols: m_cols, elem: m_elem },
                        ) => {
                            if *v_len != *m_rows {
                                panic!(
                                    "[SHAPE ERROR] Vector({}) cannot multiply Matrix({},{})",
                                    v_len, m_rows, m_cols
                                );
                            }
                            println!(
                                "[SHAPE CHECK PASSED] Vector({}) @ Matrix({},{}) -> Vector({})",
                                v_len, m_rows, m_cols, m_cols
                            );
                            ResolvedType::Vector {
                                elem: m_elem.clone(),
                                len: *m_cols,
                            }
                        }
                        (
                            ResolvedType::Matrix { rows: r1, cols: c1, elem: m_elem },
                            ResolvedType::Matrix { rows: r2, cols: c2, elem: _ },
                        ) => {
                            if *c1 != *r2 {
                                panic!(
                                    "[SHAPE ERROR] Matrix dimensions mismatch for multiplication: ({},{}) vs ({},{})",
                                    r1, c1, r2, c2
                                );
                            }
                            println!(
                                "[SHAPE CHECK PASSED] Matrix({},{}) * Matrix({},{}) -> Matrix({},{})",
                                r1, c1, r2, c2, r1, c2
                            );
                            ResolvedType::Matrix {
                                elem: m_elem.clone(),
                                rows: *r1,
                                cols: *c2,
                            }
                        }
                        _ => panic!("[TYPE ERROR] Invalid types for '@' or MatMul operator"),
                    }
                }
                BinaryOp::Pipe => {
                    let left_ty = self.infer_expression_type(left);
                    let _right_ty = self.infer_expression_type(right);
                    left_ty
                }
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                    let left_ty = self.infer_expression_type(left);
                    let right_ty = self.infer_expression_type(right);

                    match (&left_ty, &right_ty) {
                        // 1. Matrix * Matrix
                        (
                            ResolvedType::Matrix { rows: r1, cols: c1, elem: m_elem },
                            ResolvedType::Matrix { rows: r2, cols: c2, elem: _ },
                        ) if matches!(op, BinaryOp::Mul) => {
                            if *c1 != *r2 {
                                panic!(
                                    "[SHAPE ERROR] Matrix dimensions mismatch for multiplication: ({},{}) vs ({},{})",
                                    r1, c1, r2, c2
                                );
                            }
                            println!(
                                "[SHAPE CHECK PASSED] Matrix({},{}) * Matrix({},{}) -> Matrix({},{})",
                                r1, c1, r2, c2, r1, c2
                            );
                            ResolvedType::Matrix {
                                elem: m_elem.clone(),
                                rows: *r1,
                                cols: *c2,
                            }
                        }
                        // 2. Matrix +/- Matrix
                        (
                            ResolvedType::Matrix { rows: r1, cols: c1, elem: m_elem },
                            ResolvedType::Matrix { rows: r2, cols: c2, elem: _ },
                        ) if matches!(op, BinaryOp::Add | BinaryOp::Sub) => {
                            if *r1 != *r2 || *c1 != *c2 {
                                panic!(
                                    "[SHAPE ERROR] Matrix dimensions mismatch for element-wise operation: ({},{}) vs ({},{})",
                                    r1, c1, r2, c2
                                );
                            }
                            println!(
                                "[SHAPE CHECK PASSED] Matrix({},{}) +/- Matrix({},{}) -> Matrix({},{})",
                                r1, c1, r2, c2, r1, c1
                            );
                            ResolvedType::Matrix {
                                elem: m_elem.clone(),
                                rows: *r1,
                                cols: *c1,
                            }
                        }
                        // 3. Vector & Scalar Broadcasting
                        (ResolvedType::Vector { len, elem }, ResolvedType::F64)
                        | (ResolvedType::F64, ResolvedType::Vector { len, elem }) => {
                            println!("[BROADCAST CHECK PASSED] Vector({}) with Scalar F64", len);
                            ResolvedType::Vector {
                                elem: elem.clone(),
                                len: *len,
                            }
                        }
                        // 4. Matrix & Scalar Broadcasting
                        (ResolvedType::Matrix { rows, cols, elem }, ResolvedType::F64)
                        | (ResolvedType::F64, ResolvedType::Matrix { rows, cols, elem }) => {
                            println!("[BROADCAST CHECK PASSED] Matrix({},{}) with Scalar F64", rows, cols);
                            ResolvedType::Matrix {
                                elem: elem.clone(),
                                rows: *rows,
                                cols: *cols,
                            }
                        }
                        // 5. Standard Scalar Ops
                        (ResolvedType::F64, ResolvedType::F64) => ResolvedType::F64,
                        _ => panic!("[TYPE ERROR] Unsupported operands for standard binary op"),
                    }
                }
                BinaryOp::ElementMul | BinaryOp::ElementDiv | BinaryOp::ElementAdd | BinaryOp::ElementSub => {
                    let left_ty = self.infer_expression_type(left);
                    let right_ty = self.infer_expression_type(right);

                    match (&left_ty, &right_ty) {
                        (
                            ResolvedType::Vector { len: l1, elem: e1 },
                            ResolvedType::Vector { len: l2, elem: _ },
                        ) => {
                            if l1 != l2 {
                                panic!(
                                    "[SHAPE ERROR] Vector length mismatch for element-wise operation: {} vs {}",
                                    l1, l2
                                );
                            }
                            println!("[SHAPE CHECK PASSED] Vector({}) element-wise Vector({})", l1, l2);
                            ResolvedType::Vector {
                                elem: e1.clone(),
                                len: *l1,
                            }
                        }
                        (
                            ResolvedType::Matrix { rows: r1, cols: c1, elem: e1 },
                            ResolvedType::Matrix { rows: r2, cols: c2, elem: _ },
                        ) => {
                            if r1 != r2 || c1 != c2 {
                                panic!(
                                    "[SHAPE ERROR] Matrix dimensions mismatch: ({},{}) vs ({},{})",
                                    r1, c1, r2, c2
                                );
                            }
                            println!(
                                "[SHAPE CHECK PASSED] Matrix({},{}) element-wise Matrix({},{})",
                                r1, c1, r2, c2
                            );
                            ResolvedType::Matrix {
                                elem: e1.clone(),
                                rows: *r1,
                                cols: *c1,
                            }
                        }
                        _ => panic!("[TYPE ERROR] Invalid operands for element-wise operation"),
                    }
                }
            },
            Expr::Call { callee, args } => {
                if !self.symbol_table.contains_key(callee) {
                    panic!("[COMPILE ERROR] Undefined function '{}'", callee);
                }
                if callee == "print" && !args.is_empty() {
                    return self.infer_expression_type(&args[0]);
                }
                if callee == "transpose" && !args.is_empty() {
                    let arg_ty = self.infer_expression_type(&args[0]);
                    if let ResolvedType::Matrix { rows, cols, elem } = arg_ty {
                        println!("[SHAPE CHECK PASSED] transpose Matrix({},{}) -> Matrix({},{})", rows, cols, cols, rows);
                        return ResolvedType::Matrix {
                            elem,
                            rows: cols,
                            cols: rows,
                        };
                    } else {
                        panic!("[TYPE ERROR] 'transpose' only supports Matrix types");
                    }
                }
                if (callee == "zeros" || callee == "random") && args.len() == 2 {
                    if let (Expr::Number(r), Expr::Number(c)) = (&args[0], &args[1]) {
                        let rows = *r as usize;
                        let cols = *c as usize;
                        println!("[GENERATOR CHECK PASSED] {}({},{}) -> Matrix({},{})", callee, rows, cols, rows, cols);
                        return ResolvedType::Matrix {
                            elem: Box::new(ResolvedType::F64),
                            rows,
                            cols,
                        };
                    } else {
                        panic!("[TYPE ERROR] '{}' expects integer literal arguments for rows and columns", callee);
                    }
                }
                ResolvedType::Void
            }
        }
    }
}