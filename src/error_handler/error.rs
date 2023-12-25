pub struct Error<'a> {
    str: &'a str,
    line: u32,
    column: u16,
    file: &'a str,
}

impl<'a> Error<'a> {
    pub fn to_string(&self) -> &str {
        self.str
        // Format error later
    }
}
