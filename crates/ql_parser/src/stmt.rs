//! Parser rules for top-level statements, bindings, and directives.

use crate::parser::Parser;
use ql_ast::*;
use ql_lexer::Token;

impl Parser {
    pub fn parse_statement(&mut self) -> Statement {
        match self.current() {
            Token::Let => self.parse_let_statement(),
            Token::Train => self.parse_train_statement(),
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

    fn parse_train_statement(&mut self) -> Statement {
        self.advance(); // consume `train`
        self.expect(Token::LParen);

        let loss_var = if let Token::Ident(id) = self.current().clone() {
            self.advance();
            id
        } else {
            panic!("[PARSER ERROR] Expected loss variable identifier in train(...)");
        };

        self.expect(Token::Comma);

        let lr = if let Token::Number(val) = self.current() {
            let num = *val;
            self.advance();
            num
        } else {
            panic!("[PARSER ERROR] Expected learning rate number in train(...)");
        };

        self.expect(Token::Comma);

        let epochs = if let Token::Number(val) = self.current() {
            let num = *val as usize;
            self.advance();
            num
        } else {
            panic!("[PARSER ERROR] Expected epochs number in train(...)");
        };

        self.expect(Token::RParen);
        self.expect(Token::Semicolon);

        Statement::Train {
            loss_var,
            lr,
            epochs,
        }
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
}