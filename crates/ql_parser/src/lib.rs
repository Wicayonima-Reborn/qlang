use ql_ast::*;
use ql_lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> &Token {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        &self.tokens[self.pos - 1]
    }

    fn expect(&mut self, expected: Token) {
        if self.current() == &expected {
            self.advance();
        } else {
            panic!("[PARSER ERROR] Expected {:?}, found {:?}", expected, self.current());
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();
        while self.current() != &Token::Eof {
            statements.push(self.parse_statement());
        }
        Program { statements }
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current() {
            Token::Let => self.parse_let_statement(),
            _ => {
                let expr = self.parse_expression();
                self.expect(Token::Semicolon);
                Statement::Expression(expr)
            }
        }
    }

    fn parse_let_statement(&mut self) -> Statement {
        self.advance();

        let name = if let Token::Ident(id) = self.current().clone() {
            self.advance();
            id
        } else {
            panic!("[PARSER ERROR] Expected identifier after 'let'");
        };

        let mut ty = None;
        if self.current() == &Token::Colon {
            self.advance();
            ty = Some(self.parse_type_annotation());
        }

        self.expect(Token::Assign);
        let value = self.parse_expression();
        self.expect(Token::Semicolon);

        Statement::Let { name, ty, value }
    }

    fn parse_type_annotation(&mut self) -> TypeAnnotation {
        match self.current().clone() {
            Token::Ident(id) => {
                self.advance();
                if id == "dec" {
                    TypeAnnotation::Dec
                } else if id == "f64" {
                    TypeAnnotation::F64
                } else {
                    panic!("[PARSER ERROR] Unknown type annotation '{}'", id);
                }
            }
            _ => panic!("[PARSER ERROR] Expected type annotation"),
        }
    }

    fn parse_expression(&mut self) -> Expr {
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
        let mut expr = match self.current().clone() {
            Token::Number(n) => {
                self.advance();
                Expr::Number(n)
            }
            Token::Decimal(s) => {
                self.advance();
                Expr::Decimal(s)
            }
            Token::Ident(id) => {
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
            Token::LBracket => self.parse_array_literal(),
            Token::LParen => {
                self.advance();
                let inner = self.parse_expression();
                self.expect(Token::RParen);
                inner
            }
            _ => panic!("[PARSER ERROR] Unexpected token {:?}", self.current()),
        };

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
                self.advance(); // consume ',' untuk 2D Matrix Slice

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