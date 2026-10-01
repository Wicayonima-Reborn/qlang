//! `ir` - Intermediate Representation Engine for QLang computation graphs.

pub mod autodiff;
pub mod graph;
pub mod node;

pub use autodiff::{AutodiffPass, BackwardOp};
pub use graph::IRGraph;
pub use node::{IRNode, IRNodeType, NodeId};