#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Ident(String),
    Assign,
    Number(f64),
    Decimal(String),
    LBracket,
    RBracket,
    LParen,
    RParen,
    Comma,
    Semicolon,
    Colon,
    At,          // @
    PipeGreater, // |>
    DotDot,      // ..
    DotStar,     // .*
    DotSlash,    // ./
    DotPlus,     // .+
    DotMinus,    // .-
    Plus,
    Minus,
    Star,
    Slash,
    Eof,
}

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(&ch) = self.chars.peek() {
            match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    self.chars.next();
                }
                '=' => {
                    self.chars.next();
                    tokens.push(Token::Assign);
                }
                '[' => {
                    self.chars.next();
                    tokens.push(Token::LBracket);
                }
                ']' => {
                    self.chars.next();
                    tokens.push(Token::RBracket);
                }
                '(' => {
                    self.chars.next();
                    tokens.push(Token::LParen);
                }
                ')' => {
                    self.chars.next();
                    tokens.push(Token::RParen);
                }
                ',' => {
                    self.chars.next();
                    tokens.push(Token::Comma);
                }
                ';' => {
                    self.chars.next();
                    tokens.push(Token::Semicolon);
                }
                ':' => {
                    self.chars.next();
                    tokens.push(Token::Colon);
                }
                '@' => {
                    self.chars.next();
                    tokens.push(Token::At);
                }
                '|' => {
                    self.chars.next();
                    if let Some('>') = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::PipeGreater);
                    }
                }
                '.' => {
                    self.chars.next();
                    if let Some(&next_ch) = self.chars.peek() {
                        match next_ch {
                            '.' => {
                                self.chars.next();
                                tokens.push(Token::DotDot);
                            }
                            '*' => {
                                self.chars.next();
                                tokens.push(Token::DotStar);
                            }
                            '/' => {
                                self.chars.next();
                                tokens.push(Token::DotSlash);
                            }
                            '+' => {
                                self.chars.next();
                                tokens.push(Token::DotPlus);
                            }
                            '-' => {
                                self.chars.next();
                                tokens.push(Token::DotMinus);
                            }
                            _ => panic!("[LEXER ERROR] Unexpected character after '.'"),
                        }
                    } else {
                        panic!("[LEXER ERROR] Unexpected EOF after '.'");
                    }
                }
                '+' => {
                    self.chars.next();
                    tokens.push(Token::Plus);
                }
                '-' => {
                    self.chars.next();
                    tokens.push(Token::Minus);
                }
                '*' => {
                    self.chars.next();
                    tokens.push(Token::Star);
                }
                '/' => {
                    self.chars.next();
                    tokens.push(Token::Slash);
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut ident = String::new();
                    while let Some(&c) = self.chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            ident.push(c);
                            self.chars.next();
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
                    let mut is_dec = false;

                    while let Some(&c) = self.chars.peek() {
                        if c.is_ascii_digit() {
                            num_str.push(c);
                            self.chars.next();
                        } else if c == '.' {
                            let mut clone_iter = self.chars.clone();
                            clone_iter.next();
                            if let Some(&next_c) = clone_iter.peek() {
                                if next_c.is_ascii_digit() {
                                    num_str.push('.');
                                    self.chars.next();
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        } else if c == 'd' {
                            is_dec = true;
                            self.chars.next();
                            break;
                        } else {
                            break;
                        }
                    }

                    if is_dec {
                        tokens.push(Token::Decimal(num_str));
                    } else {
                        let val: f64 = num_str.parse().unwrap_or_else(|_| {
                            panic!("[LEXER ERROR] Failed to parse float literal: '{}'", num_str);
                        });
                        tokens.push(Token::Number(val));
                    }
                }
                _ => {
                    self.chars.next();
                }
            }
        }

        tokens.push(Token::Eof);
        tokens
    }
}