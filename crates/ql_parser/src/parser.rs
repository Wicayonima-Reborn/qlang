//! Core Recursive Descent Parser engine for QLang.

use ql_ast::*;
use ql_lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    #[inline]
    pub fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    pub fn advance(&mut self) -> &Token {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        &self.tokens[self.pos - 1]
    }

    pub fn expect(&mut self, expected: Token) {
        if self.current() == &expected {
            self.advance();
        } else {
            panic!("[PARSER ERROR] Expected {:?}, found {:?}", expected, self.current());
        }
    }

    /// Entry point for parsing an entire source file into a `Program` AST node.
    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();
        while self.current() != &Token::Eof {
            statements.push(self.parse_statement());
        }
        Program { statements }
    }
}