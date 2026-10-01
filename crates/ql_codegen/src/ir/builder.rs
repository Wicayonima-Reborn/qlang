//! AST-to-IR Graph converter builder.

use crate::ir::graph::IRGraph;
use crate::ir::node::NodeId;
use ql_ast::*;
use ql_checker::{ResolvedType, TypeChecker};

pub struct IRBuilder<'a> {
    pub graph: IRGraph,
    checker: &'a TypeChecker,
}

impl<'a> IRBuilder<'a> {
    pub fn new(checker: &'a TypeChecker) -> Self {
        Self {
            graph: IRGraph::new(),
            checker,
        }
    }

    pub fn build_from_program(&mut self, program: &Program) {
        for stmt in &program.statements {
            if let Statement::Let { name, ty: _, value } = stmt {
                let val_node = self.build_from_expr(value);
                // Assign name to node mapping
                self.graph.name_to_node.insert(name.clone(), val_node);
            }
        }
    }

    pub fn build_from_expr(&mut self, expr: &Expr) -> NodeId {
        match expr {
            Expr::Variable(name) => {
                if let Some(&node_id) = self.graph.name_to_node.get(name) {
                    node_id
                } else {
                    let ty = self.checker.infer_expression_type(expr);
                    let shape = match ty {
                        ResolvedType::Matrix { rows, cols, .. } => vec![rows, cols],
                        ResolvedType::Vector { len, .. } => vec![len],
                        _ => vec![1],
                    };
                    self.graph.add_input(name, shape, true)
                }
            }
            Expr::Number(n) => {
                let id = self.graph.nodes.len();
                let node = crate::ir::node::IRNode {
                    id,
                    node_type: crate::ir::node::IRNodeType::Constant(*n),
                    shape: vec![1],
                    requires_grad: false,
                };
                self.graph.nodes.push(node);
                id
            }
            Expr::Binary { op, left, right } => {
                let left_id = self.build_from_expr(left);
                let right_id = self.build_from_expr(right);

                let ty = self.checker.infer_expression_type(expr);
                let shape = match ty {
                    ResolvedType::Matrix { rows, cols, .. } => vec![rows, cols],
                    ResolvedType::Vector { len, .. } => vec![len],
                    _ => vec![1],
                };

                self.graph.add_binary(op.clone(), left_id, right_id, shape)
            }
            Expr::Call { callee, args } => {
                if (callee == "relu" || callee == "sigmoid") && !args.is_empty() {
                    let arg_id = self.build_from_expr(&args[0]);
                    self.graph.add_activation(callee, arg_id)
                } else if callee == "mse_loss" && args.len() == 2 {
                    let pred_id = self.build_from_expr(&args[0]);
                    let target_id = self.build_from_expr(&args[1]);

                    let id = self.graph.nodes.len();
                    let node = crate::ir::node::IRNode {
                        id,
                        node_type: crate::ir::node::IRNodeType::Loss {
                            kind: callee.clone(),
                            pred: pred_id,
                            target: target_id,
                        },
                        shape: vec![1],
                        requires_grad: true,
                    };
                    self.graph.nodes.push(node);
                    id
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}