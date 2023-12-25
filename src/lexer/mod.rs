use crate::token::Token;

pub struct Lexer<'a> {
    pub source: &'a str,
    pub tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn scan_tokens(&self) {}
}
