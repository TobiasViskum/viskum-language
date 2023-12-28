use std::fmt;

#[derive(Debug, Clone)]
pub enum Literal {
    Num(f64),
    Str(String),
    Null,
    True,
    False,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Num(x) => write!(f, "{x}"),
            Literal::Str(str) => write!(f, "\"{str}\""),
            Literal::Null => write!(f, "null"),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
        }
    }
}
