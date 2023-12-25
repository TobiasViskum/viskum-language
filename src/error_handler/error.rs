pub struct ViskumError<'a> {
    str: &'a str,
    line: usize,
    column: usize,
    file: &'a str,
}

impl<'a> ViskumError<'a> {
    pub fn to_string(&self) -> &str {
        self.str
        // Format error later
    }
}
