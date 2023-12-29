use colorize::{ self, AnsiColor };

use crate::token::Token;

#[derive(Debug)]
pub struct ViskumError {
    msg: String,
    token: Token,
    file: String,
}

impl ViskumError {
    pub fn new(msg: &str, token: Token, file: &str) -> Self {
        ViskumError { msg: msg.to_string(), token, file: file.to_string() }
    }

    pub fn to_string(&self) -> String {
        let p1 = "[error]".red().bold();

        let p2 = vec![self.file.to_string(), ":".to_string()].join("").red();

        let p3 = vec!["line".to_string(), self.token.line.to_string()].join(" ").red();

        let p4 = ", ".red();

        format!("{} {}{}{} ({})", p1, vec![p2, p3].join(" "), p4, self.msg.to_string().red(), 0)
    }
}
