use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Num(f64),
    Str(String),
    Bool(bool),
    Null,
}

impl Literal {
    pub fn to_type_string(&self) -> String {
        match self {
            Literal::Bool(_) => "bool".to_string(),
            Literal::Null => "null".to_string(),
            Literal::Num(_) => "number".to_string(),
            Literal::Str(_) => "string".to_string(),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Num(x) => write!(f, "{x}"),
            Literal::Str(str) => write!(f, "{str}"),
            Literal::Null => write!(f, "null"),
            Literal::Bool(b) => if *b { write!(f, "true") } else { write!(f, "false") }
        }
    }
}
