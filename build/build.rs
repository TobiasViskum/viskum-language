mod generate_ast;
use generate_ast::*;
use std::io;

fn main() -> io::Result<()> {
    generate_ast(&"src".to_string())?;
    Ok(())
}
