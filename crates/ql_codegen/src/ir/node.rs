//! IR Node representations for computation graph nodes.

use ql_ast::BinaryOp;

pub type NodeId = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum IRNodeType {
    /// Variable or constant data array (e.g. Input X, Weights W, Bias B)
    Input { name: String },
    /// Scalar constant
    Constant(f64),
    /// Binary matrix/tensor operation (@, +, -, *, .*, etc.)
    Binary {
        op: BinaryOp,
        left: NodeId,
        right: NodeId,
    },
    /// Activation function intrinsics (sigmoid, relu)
    Activation {
        kind: String,
        input: NodeId,
    },
    /// Loss calculation node
    Loss {
        kind: String,
        pred: NodeId,
        target: NodeId,
    },
}

#[derive(Debug, Clone)]
pub struct IRNode {
    pub id: NodeId,
    pub node_type: IRNodeType,
    pub shape: Vec<usize>,
    pub requires_grad: bool,
}