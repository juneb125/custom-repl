use crate::token::Token;

pub struct Lexer<'a> {
    pub source: &'a [u8],
    pub tokens: Vec<Token<'a>>,
    pub location: usize,
    // location is the current character (in source) that we're looking at
    // location is 0-indexed
}

#[allow(unused)]
impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            tokens: Vec::new(),
            location: 0,
        }
    }

    fn this(&self) -> char {
        self.source[self.location].into()
    }

    // TODO: check for 'off by 1' errors for "is_last_char()" (and any fn that calls it)
    fn is_last_char(&self) -> bool {
        // returns true if there's no more chars left
        // "abcd"
        //     ^ is_last_char() would return true here b/c self.location == 3
        (self.location + 1) >= self.source.len()
    }

    // gets the next char without increasing location
    fn peek(&self) -> Option<char> {
        if self.is_last_char() {
            // out of bounds
            return None;
        }
        Some(self.source[self.location + 1].into())
    }

    // just increases location
    fn advance(&mut self) -> Result<(), &str> {
        if self.is_last_char() {
            return Err("out of bounds");
        }
        self.location += 1;
        Ok(())
    }

    fn push_token(&mut self, token: Token<'a>) {
        self.tokens.push(token);
    }

    fn is_whitespace(&self) -> bool {
        match self.source[self.location] {
            b' ' | b'\n' | b'\t' | b'\r' | b'\0' => true,
            _ => false,
        }
    }
}
