//! Main tokenizing engine implementation.

use crate::cursor::Cursor;
use crate::token::Token;

pub struct Lexer {
    cursor: Cursor,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            cursor: Cursor::new(input),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.cursor.current() {
            if ch.is_whitespace() {
                self.cursor.advance();
                continue;
            }

            // Skip line comments (`// ...`)
            if ch == '/' && self.cursor.peek() == Some('/') {
                while let Some(c) = self.cursor.current() {
                    if c == '\n' {
                        break;
                    }
                    self.cursor.advance();
                }
                continue;
            }

            match ch {
                '=' => {
                    self.cursor.advance();
                    tokens.push(Token::Assign);
                }
                ':' => {
                    self.cursor.advance();
                    tokens.push(Token::Colon);
                }
                ';' => {
                    self.cursor.advance();
                    tokens.push(Token::Semicolon);
                }
                ',' => {
                    self.cursor.advance();
                    tokens.push(Token::Comma);
                }
                '+' => {
                    self.cursor.advance();
                    tokens.push(Token::Plus);
                }
                '-' => {
                    self.cursor.advance();
                    tokens.push(Token::Minus);
                }
                '*' => {
                    self.cursor.advance();
                    tokens.push(Token::Star);
                }
                '/' => {
                    self.cursor.advance();
                    tokens.push(Token::Slash);
                }
                '@' => {
                    self.cursor.advance();
                    tokens.push(Token::At);
                }
                '(' => {
                    self.cursor.advance();
                    tokens.push(Token::LParen);
                }
                ')' => {
                    self.cursor.advance();
                    tokens.push(Token::RParen);
                }
                '[' => {
                    self.cursor.advance();
                    tokens.push(Token::LBracket);
                }
                ']' => {
                    self.cursor.advance();
                    tokens.push(Token::RBracket);
                }
                '|' => {
                    if self.cursor.peek() == Some('>') {
                        self.cursor.advance();
                        self.cursor.advance();
                        tokens.push(Token::PipeGreater);
                    } else {
                        panic!("[LEXER ERROR] Unexpected character '|'");
                    }
                }
                '.' => {
                    if self.cursor.peek() == Some('.') {
                        self.cursor.advance();
                        self.cursor.advance();
                        tokens.push(Token::DotDot);
                    } else if self.cursor.peek() == Some('+') {
                        self.cursor.advance();
                        self.cursor.advance();
                        tokens.push(Token::DotPlus);
                    } else if self.cursor.peek() == Some('-') {
                        self.cursor.advance();
                        self.cursor.advance();
                        tokens.push(Token::DotMinus);
                    } else if self.cursor.peek() == Some('*') {
                        self.cursor.advance();
                        self.cursor.advance();
                        tokens.push(Token::DotStar);
                    } else if self.cursor.peek() == Some('/') {
                        self.cursor.advance();
                        self.cursor.advance();
                        tokens.push(Token::DotSlash);
                    } else {
                        panic!("[LEXER ERROR] Unexpected character '.'");
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut ident = String::new();
                    while let Some(c) = self.cursor.current() {
                        if c.is_alphanumeric() || c == '_' {
                            ident.push(c);
                            self.cursor.advance();
                        } else {
                            break;
                        }
                    }
                    if ident == "let" {
                        tokens.push(Token::Let);
                    } else if ident == "train" {
                        tokens.push(Token::Train);
                    } else {
                        tokens.push(Token::Ident(ident));
                    }
                }
                '0'..='9' => {
                    let mut num_str = String::new();
                    let mut is_decimal = false;

                    while let Some(c) = self.cursor.current() {
                        if c.is_ascii_digit() {
                            num_str.push(c);
                            self.cursor.advance();
                        } else if c == '.' && self.cursor.peek() != Some('.') && !is_decimal {
                            is_decimal = true;
                            num_str.push(c);
                            self.cursor.advance();
                        } else {
                            break;
                        }
                    }

                    let val: f64 = num_str.parse().unwrap();
                    tokens.push(Token::Number(val));
                }
                _ => panic!("[LEXER ERROR] Unexpected character '{}'", ch),
            }
        }

        tokens.push(Token::Eof);
        tokens
    }
}