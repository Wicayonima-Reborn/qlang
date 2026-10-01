//! Automatic Reverse-Mode Differentiation Pass over IRGraph.

use crate::ir::graph::IRGraph;
use crate::ir::node::{IRNodeType, NodeId};

#[derive(Debug, Clone)]
pub struct BackwardOp {
    pub grad_output: String, // misal "dPred"
    pub grad_input: String,  // misal "dZ"
    pub op_type: String,     // misal "sigmoid_backward"
    pub node_id: NodeId,
}

pub struct AutodiffPass;

impl AutodiffPass {
    /// Builds gradient backward pass execution chain from a Loss Node
    pub fn build_backward_chain(graph: &IRGraph, loss_node_id: NodeId) -> Vec<BackwardOp> {
        let mut backward_ops = Vec::new();

        // Trace topologically backwards from loss node
        let mut nodes_to_visit = vec![loss_node_id];

        while let Some(current_id) = nodes_to_visit.pop() {
            let node = &graph.nodes[current_id];

            match &node.node_type {
                IRNodeType::Loss { kind, pred, target: _ } => {
                    if kind == "mse_loss" {
                        backward_ops.push(BackwardOp {
                            grad_output: format!("grad_node_{}", current_id),
                            grad_input: format!("grad_node_{}", pred),
                            op_type: "mse_loss_backward".to_string(),
                            node_id: current_id,
                        });
                        nodes_to_visit.push(*pred);
                    }
                }
                IRNodeType::Activation { kind, input } => {
                    backward_ops.push(BackwardOp {
                        grad_output: format!("grad_node_{}", current_id),
                        grad_input: format!("grad_node_{}", input),
                        op_type: format!("{}_backward", kind),
                        node_id: current_id,
                    });
                    nodes_to_visit.push(*input);
                }
                IRNodeType::Binary { op, left, right } => {
                    match op {
                        ql_ast::BinaryOp::MatMul => {
                            // dW = X^T @ dZ, dX = dZ @ W^T
                            if graph.nodes[*right].requires_grad {
                                backward_ops.push(BackwardOp {
                                    grad_output: format!("grad_node_{}", current_id),
                                    grad_input: format!("grad_node_{}", right),
                                    op_type: "matmul_backward_weight".to_string(),
                                    node_id: current_id,
                                });
                            }
                            if graph.nodes[*left].requires_grad {
                                nodes_to_visit.push(*left);
                            }
                            if graph.nodes[*right].requires_grad {
                                nodes_to_visit.push(*right);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        backward_ops
    }
}