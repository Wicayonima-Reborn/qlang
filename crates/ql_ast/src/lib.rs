#[derive(Debug, Clone)]
pub enum TypeAnnotation {
    Dec,
    F64,
    Vector(usize),
    Matrix(usize, usize),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    MatMul,     // @
    Pipe,       // |>
    ElementMul, // .*
    ElementDiv, // ./
    ElementAdd, // .+
    ElementSub, // .-
    Add,        // +
    Sub,        // -
    Mul,        // *
    Div,        // /
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Decimal(String),
    Variable(String),
    Vector(Vec<Expr>),
    Matrix(Vec<Vec<Expr>>),
    Slice {
        target: Box<Expr>,
        start: usize,
        end: usize,
    },
    MatrixSlice {
        target: Box<Expr>,
        r_start: usize,
        r_end: usize,
        c_start: usize,
        c_end: usize,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        callee: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        name: String,
        ty: Option<TypeAnnotation>,
        value: Expr,
    },
    Expression(Expr),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}