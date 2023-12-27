use colorize::{ self, AnsiColor };

pub struct ViskumError {
    msg: String,
    line: usize,
    column: usize,
    file: String,
}

impl ViskumError {
    pub fn new(msg: String, line: usize, column: usize, file: String) -> Self {
        ViskumError { msg, line, column, file }
    }

    pub fn to_string(&self) -> String {
        let p1 = "[error]".red().bold();

        let p2 = vec![self.file.to_string(), ":".to_string()].join("").red();

        let p3 = vec!["line".to_string(), self.line.to_string()].join(" ").red();

        let p4 = ", ".red();

        format!(
            "{} {}{}{} ({})",
            p1,
            vec![p2, p3].join(" "),
            p4,
            self.msg.to_string().red(),
            self.column
        )
    }
}
