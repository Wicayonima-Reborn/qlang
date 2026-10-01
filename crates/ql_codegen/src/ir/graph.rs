//! Directed Acyclic Graph (DAG) for QLang computation graph representation.

use crate::ir::node::{IRNode, IRNodeType, NodeId};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct IRGraph {
    pub nodes: Vec<IRNode>,
    pub name_to_node: HashMap<String, NodeId>,
}

impl IRGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an input variable to the graph
    pub fn add_input(&mut self, name: &str, shape: Vec<usize>, requires_grad: bool) -> NodeId {
        let id = self.nodes.len();
        let node = IRNode {
            id,
            node_type: IRNodeType::Input { name: name.to_string() },
            shape,
            requires_grad,
        };
        self.nodes.push(node);
        self.name_to_node.insert(name.to_string(), id);
        id
    }

    /// Add a binary operation node
    pub fn add_binary(
        &mut self,
        op: ql_ast::BinaryOp,
        left: NodeId,
        right: NodeId,
        out_shape: Vec<usize>,
    ) -> NodeId {
        let id = self.nodes.len();
        let requires_grad = self.nodes[left].requires_grad || self.nodes[right].requires_grad;

        let node = IRNode {
            id,
            node_type: IRNodeType::Binary { op, left, right },
            shape: out_shape,
            requires_grad,
        };
        self.nodes.push(node);
        id
    }

    /// Add an activation node (Sigmoid, ReLU)
    pub fn add_activation(&mut self, kind: &str, input: NodeId) -> NodeId {
        let id = self.nodes.len();
        let shape = self.nodes[input].shape.clone();
        let requires_grad = self.nodes[input].requires_grad;

        let node = IRNode {
            id,
            node_type: IRNodeType::Activation {
                kind: kind.to_string(),
                input,
            },
            shape,
            requires_grad,
        };
        self.nodes.push(node);
        id
    }

    /// Retrieve topological ordering of nodes for execution
    pub fn topological_sort(&self) -> Vec<NodeId> {
        // Linear ordering for nodes as they are appended topologically
        (0..self.nodes.len()).collect()
    }
}