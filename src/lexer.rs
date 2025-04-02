use crate::token::Token;

pub struct Lexer<'a> {
    pub source: Vec<char>,
    pub tokens: Vec<Token<'a>>,
    pub location: usize,
    // location is the current character (in source) that we're looking at
    // location is 0-indexed
}

#[allow(unused)]
impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            location: 0,
        }
    }

    /// gets the char at the current location
    fn this(&self) -> char {
        self.source[self.location]
    }

    /// gets the next char without increasing location
    fn peek(&self) -> Option<&char> {
        match self.location {
            i if (i + 1) >= self.src_len() => None,
            _ => Some(&self.source[self.location + 1]),
        }
    }

    /// gets the previous char without decreasing location
    fn prev(&self) -> Option<&char> {
        match self.location {
            0 => None,
            _ => Some(&self.source[self.location - 1]),
        }
    }

    fn push_token(&mut self, token: Token<'a>) {
        self.tokens.push(token);
    }

    // TODO: check for 'off by 1' errors for "is_last_char()" (and any fn that calls it)
    fn is_last_char(&self) -> bool {
        // returns true if there's no more chars left
        // "abcd"
        //     ^ is_last_char() would return true here b/c self.location == 3
        self.location >= self.src_len()
    }

    fn src_len(&self) -> usize {
        self.source.len()
    }

    fn is_whitespace(&self) -> bool {
        match self.source[self.location] {
            ' ' | '\n' | '\t' | '\r' | '\0' => true,
            _ => false,
        }
    }
}
