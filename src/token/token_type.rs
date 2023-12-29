#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Colon,
    QuestionMark,
    Slash,
    Star,
    Power,
    Factorial,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

    And,
    Class,
    Else,
    False,
    For,
    If,
    Null,
    Or,
    Print,
    Return,
    New,
    This,
    True,
    Let,
    While,

    Eof,

    Invalid,
}

impl TokenType {
    // pub fn to_string(&self) -> String {
    //     let str = match self {
    //         Self::LeftParen => "left parenthesis",
    //         Self::RightParen => "right parenthesis",
    //         Self::LeftBrace => "left brace",
    //         Self::RightBrace => "right brace",

    //     };

    //     str.to_string()
    // }

    pub fn to_lexeme(&self) -> String {
        let str = match self {
            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::LeftBrace => "{",
            Self::RightBrace => "}",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::Minus => "-",
            Self::Plus => "+",
            Self::Semicolon => ";",
            Self::Colon => ":",
            Self::QuestionMark => "?",
            Self::Slash => "/",
            Self::Star => "*",
            Self::Power => "^",
            Self::Factorial => "!",

            Self::Bang => "!",
            Self::BangEqual => "!=",
            Self::Equal => "=",
            Self::EqualEqual => "==",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::Less => "<",
            Self::LessEqual => "<=",

            Self::Identifier => "identifier",
            Self::String => "string",
            Self::Number => "number",

            Self::And => "and",
            Self::Class => "class",
            Self::Else => "else",
            Self::False => "false",
            Self::For => "for",
            Self::If => "if",
            Self::Null => "null",
            Self::Or => "or",
            Self::Print => "print",
            Self::Return => "return",
            Self::New => "new",
            Self::This => "this",
            Self::True => "true",
            Self::Let => "let",
            Self::While => "while",

            Self::Eof => "end of file",

            Self::Invalid => "invalid",
        };

        str.to_string()
    }
}
