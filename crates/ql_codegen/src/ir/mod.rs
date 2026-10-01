//! `ir` - Intermediate Representation Engine for QLang computation graphs.

pub mod autodiff;
pub mod builder;
pub mod graph;
pub mod node;

pub use autodiff::{AutodiffPass, BackwardOp};
pub use builder::IRBuilder;
pub use graph::IRGraph;
pub use node::{IRNode, IRNodeType, NodeId};