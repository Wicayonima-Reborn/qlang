//! Low-level character stream cursor for safe buffer traversal.

pub struct Cursor {
    input: Vec<char>,
    pos: usize,
}

impl Cursor {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    #[inline]
    pub fn current(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    #[inline]
    pub fn peek(&self) -> Option<char> {
        self.input.get(self.pos + 1).copied()
    }

    #[inline]
    pub fn advance(&mut self) {
        self.pos += 1;
    }
}