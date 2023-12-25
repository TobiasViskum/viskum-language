mod token_type;

use std::fmt;

use token_type::TokenType;

//Possibly rename to "Literal"
#[derive(Debug)]
pub enum Object {
    Num(f64),
    Str(String),
    Null,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(str) => write!(f, "\"{x}\""),
            Object::Null => write!(f, "null"),
            Object::True => write!(f, "true"),
            Object::False => write!(f, "false"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

impl Token {}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {}", self.ttype, self.lexeme, if let Some(literal) = self.literal {
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
