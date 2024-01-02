use colorize::{ self, AnsiColor };

use crate::token::{ Token, Literal };

#[derive(Debug, PartialEq)]
pub enum AbortReason {
    Break,
    Continue,
    Return(Literal),
}

#[derive(Debug)]
pub struct ViskumError {
    msg: String,
    token: Token,
    file: String,
    abort_reason: Option<AbortReason>,
    abort_value: Option<Literal>,
}

impl ViskumError {
    pub fn new(msg: &str, token: Token, file: &str) -> Self {
        ViskumError {
            msg: msg.to_string(),
            token,
            file: file.to_string(),
            abort_reason: None,
            abort_value: None,
        }
    }
    pub fn new_with_abort(msg: &str, token: Token, file: &str, reason: AbortReason) -> Self {
        ViskumError {
            msg: msg.to_string(),
            token,
            file: file.to_string(),
            abort_reason: Some(reason),
            abort_value: None,
        }
    }

    pub fn is_abort_error(&self, abort_reason: AbortReason) -> bool {
        match &self.abort_reason {
            Some(AbortReason::Return(_)) => true,
            Some(reason) => reason == &abort_reason,
            None => false,
        }
    }

    pub fn get_abort_value(&self) -> Option<Literal> {
        self.abort_reason.as_ref().and_then(|reason| {
            match reason {
                AbortReason::Return(value) => Some(value.clone()),
                _ => None,
            }
        })
    }

    pub fn to_string(&self) -> String {
        let p1 = "[error]".red().bold();

        let p2 = vec![self.file.to_string(), ":".to_string()].join("").red();

        let p3 = vec!["line".to_string(), self.token.line.to_string()].join(" ").red();

        let p4 = ", ".red();

        format!("{} {}{}{} ({})", p1, vec![p2, p3].join(" "), p4, self.msg.to_string().red(), 0)
    }
}
