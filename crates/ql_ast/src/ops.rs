//! Binary and mathematical operators supported by QLang core.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    /// Matrix multiplication (`@`)
    MatMul,
    /// Pipeline operator (`|>`)
    Pipe,
    /// Element-wise operations (`.*`, `./`, `.+=`, `.-`)
    ElementMul,
    ElementDiv,
    ElementAdd,
    ElementSub,
    /// Standard arithmetic
    Add,
    Sub,
    Mul,
    Div,
}