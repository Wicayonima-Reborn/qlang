//! Precedence-climbing operator and expression parser implementation.

use crate::parser::Parser;
use ql_ast::*;
use ql_lexer::Token;

impl Parser {
    pub fn parse_expression(&mut self) -> Expr {
        self.parse_pipeline()
    }

    fn parse_pipeline(&mut self) -> Expr {
        let mut left = self.parse_additive();

        while self.current() == &Token::PipeGreater {
            self.advance();
            let right = self.parse_additive();
            left = Expr::Binary {
                op: BinaryOp::Pipe,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_additive(&mut self) -> Expr {
        let mut left = self.parse_multiplicative();

        while matches!(self.current(), Token::Plus | Token::Minus | Token::DotPlus | Token::DotMinus) {
            let op = match self.current() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                Token::DotPlus => BinaryOp::ElementAdd,
                Token::DotMinus => BinaryOp::ElementSub,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_multiplicative();
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_multiplicative(&mut self) -> Expr {
        let mut left = self.parse_primary();

        while matches!(self.current(), Token::Star | Token::Slash | Token::At | Token::DotStar | Token::DotSlash) {
            let op = match self.current() {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::At => BinaryOp::MatMul,
                Token::DotStar => BinaryOp::ElementMul,
                Token::DotSlash => BinaryOp::ElementDiv,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_primary();
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_primary(&mut self) -> Expr {
        // Handle unary minus (e.g. `-2.0`)
        let mut is_negative = false;
        if self.current() == &Token::Minus {
            is_negative = true;
            self.advance();
        }

        let mut expr = match self.current().clone() {
            Token::Number(n) => {
                self.advance();
                let val = if is_negative { -n } else { n };
                Expr::Number(val)
            }
            Token::Decimal(s) => {
                self.advance();
                let val = if is_negative { format!("-{}", s) } else { s };
                Expr::Decimal(val)
            }
            Token::Ident(id) => {
                if is_negative {
                    panic!("[PARSER ERROR] Unary minus on identifier '{}' not supported directly", id);
                }
                self.advance();
                if self.current() == &Token::LParen {
                    self.advance();
                    let mut args = Vec::new();
                    if self.current() != &Token::RParen {
                        loop {
                            args.push(self.parse_expression());
                            if self.current() == &Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(Token::RParen);
                    Expr::Call { callee: id, args }
                } else {
                    Expr::Variable(id)
                }
            }
            Token::LBracket => {
                if is_negative {
                    panic!("[PARSER ERROR] Unary minus on array literal not supported");
                }
                self.parse_array_literal()
            }
            Token::LParen => {
                if is_negative {
                    panic!("[PARSER ERROR] Unary minus before '(' not supported directly");
                }
                self.advance();
                let inner = self.parse_expression();
                self.expect(Token::RParen);
                inner
            }
            _ => panic!("[PARSER ERROR] Unexpected token {:?}", self.current()),
        };

        // Postfix Slicing operators: `arr[r1..r2]` or `mat[r1..r2, c1..c2]`
        if self.current() == &Token::LBracket {
            self.advance();

            let r_start = if let Token::Number(n) = self.current() {
                let val = *n as usize;
                self.advance();
                val
            } else {
                0
            };

            self.expect(Token::DotDot);

            let r_end = if let Token::Number(n) = self.current() {
                let val = *n as usize;
                self.advance();
                val
            } else {
                panic!("[PARSER ERROR] Expected end index for slice");
            };

            if self.current() == &Token::Comma {
                self.advance(); // consume ',' for 2D Matrix Slice

                let c_start = if let Token::Number(n) = self.current() {
                    let val = *n as usize;
                    self.advance();
                    val
                } else {
                    0
                };

                self.expect(Token::DotDot);

                let c_end = if let Token::Number(n) = self.current() {
                    let val = *n as usize;
                    self.advance();
                    val
                } else {
                    panic!("[PARSER ERROR] Expected end index for matrix column slice");
                };

                self.expect(Token::RBracket);

                expr = Expr::MatrixSlice {
                    target: Box::new(expr),
                    r_start,
                    r_end,
                    c_start,
                    c_end,
                };
            } else {
                self.expect(Token::RBracket);

                expr = Expr::Slice {
                    target: Box::new(expr),
                    start: r_start,
                    end: r_end,
                };
            }
        }

        expr
    }

    fn parse_array_literal(&mut self) -> Expr {
        self.advance();

        if self.current() == &Token::LBracket {
            let mut rows = Vec::new();
            while self.current() == &Token::LBracket {
                self.advance();
                let mut row = Vec::new();
                while self.current() != &Token::RBracket {
                    row.push(self.parse_expression());
                    if self.current() == &Token::Comma {
                        self.advance();
                    }
                }
                self.expect(Token::RBracket);
                rows.push(row);
                if self.current() == &Token::Comma {
                    self.advance();
                }
            }
            self.expect(Token::RBracket);
            Expr::Matrix(rows)
        } else {
            let mut elems = Vec::new();
            while self.current() != &Token::RBracket {
                elems.push(self.parse_expression());
                if self.current() == &Token::Comma {
                    self.advance();
                }
            }
            self.expect(Token::RBracket);
            Expr::Vector(elems)
        }
    }
}