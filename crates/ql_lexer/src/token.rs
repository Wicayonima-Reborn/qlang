//! Token definitions for QLang lexical scanner.

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,
    Train,

    // Identifiers & Literals
    Ident(String),
    Number(f64),
    Decimal(String),

    // Delimiters & Structural Symbols
    Colon,
    Assign,
    Semicolon,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    DotDot, // `..` range operator for slicing

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    At,           // `@` MatMul
    PipeGreater,  // `|>` Pipeline

    // Element-wise Tensor Operators
    DotPlus,  // `.+=` or `.+`
    DotMinus, // `.-`
    DotStar,  // `.*`
    DotSlash, // `./`

    // Control
    Eof,
}