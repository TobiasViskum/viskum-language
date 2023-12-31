mod token_type;
mod literal;

use std::fmt;

pub use self::token_type::TokenType;
pub use self::literal::Literal;

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Token { ttype, lexeme, literal, line }
    }

    pub fn invalid(line: Option<usize>) -> Token {
        Token::new(TokenType::Invalid, "".to_string(), None, if let Some(line) = line {
            line
        } else {
            0
        })
    }

    pub fn eof(line: usize) -> Token {
        Token::new(TokenType::Eof, "".to_string(), None, line)
    }

    pub fn is(&self, ttype: TokenType) -> bool {
        ttype == self.ttype
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {}", self.ttype, self.lexeme, if let Some(literal) = &self.literal {
            literal.to_string()
        } else {
            "None".to_string()
        })
    }
}

/* 
pub enum Token {
    Literal {lexeme: String, literal: <...>}
}
*/
