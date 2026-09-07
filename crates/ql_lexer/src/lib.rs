#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Ident(String),
    Colon,
    Assign,
    Semicolon,
    Number(f64),
    Decimal(String),
    Plus,
    Minus,
    Star,
    Slash,
    At,
    PipeGreater,
    DotPlus,
    DotMinus,
    DotStar,
    DotSlash,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    DotDot,
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.pos + 1 < self.input.len() {
            Some(self.input[self.pos + 1])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
                continue;
            }

            // Support Komentar //
            if ch == '/' && self.peek_char() == Some('/') {
                while let Some(c) = self.current_char() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
                continue;
            }

            match ch {
                '=' => {
                    self.advance();
                    tokens.push(Token::Assign);
                }
                ':' => {
                    self.advance();
                    tokens.push(Token::Colon);
                }
                ';' => {
                    self.advance();
                    tokens.push(Token::Semicolon);
                }
                ',' => {
                    self.advance();
                    tokens.push(Token::Comma);
                }
                '+' => {
                    self.advance();
                    tokens.push(Token::Plus);
                }
                '-' => {
                    self.advance();
                    tokens.push(Token::Minus);
                }
                '*' => {
                    self.advance();
                    tokens.push(Token::Star);
                }
                '/' => {
                    self.advance();
                    tokens.push(Token::Slash);
                }
                '@' => {
                    self.advance();
                    tokens.push(Token::At);
                }
                '(' => {
                    self.advance();
                    tokens.push(Token::LParen);
                }
                ')' => {
                    self.advance();
                    tokens.push(Token::RParen);
                }
                '[' => {
                    self.advance();
                    tokens.push(Token::LBracket);
                }
                ']' => {
                    self.advance();
                    tokens.push(Token::RBracket);
                }
                '|' => {
                    if self.peek_char() == Some('>') {
                        self.advance();
                        self.advance();
                        tokens.push(Token::PipeGreater);
                    } else {
                        panic!("[LEXER ERROR] Unexpected character '|'");
                    }
                }
                '.' => {
                    if self.peek_char() == Some('.') {
                        self.advance();
                        self.advance();
                        tokens.push(Token::DotDot);
                    } else if self.peek_char() == Some('+') {
                        self.advance();
                        self.advance();
                        tokens.push(Token::DotPlus);
                    } else if self.peek_char() == Some('-') {
                        self.advance();
                        self.advance();
                        tokens.push(Token::DotMinus);
                    } else if self.peek_char() == Some('*') {
                        self.advance();
                        self.advance();
                        tokens.push(Token::DotStar);
                    } else if self.peek_char() == Some('/') {
                        self.advance();
                        self.advance();
                        tokens.push(Token::DotSlash);
                    } else {
                        panic!("[LEXER ERROR] Unexpected character '.'");
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut ident = String::new();
                    while let Some(c) = self.current_char() {
                        if c.is_alphanumeric() || c == '_' {
                            ident.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if ident == "let" {
                        tokens.push(Token::Let);
                    } else {
                        tokens.push(Token::Ident(ident));
                    }
                }
                '0'..='9' => {
                    let mut num_str = String::new();
                    let mut is_decimal = false;

                    while let Some(c) = self.current_char() {
                        if c.is_ascii_digit() {
                            num_str.push(c);
                            self.advance();
                        } else if c == '.' && self.peek_char() != Some('.') && !is_decimal {
                            is_decimal = true;
                            num_str.push(c);
                            self.advance();
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