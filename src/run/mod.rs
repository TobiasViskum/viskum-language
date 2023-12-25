mod run_file;
mod run_prompt;

pub use crate::run::run_file::run_file;
pub use crate::run::run_prompt::run_prompt;

use crate::error_handler::ErrorHandler;

pub fn run(source: &[u8]) {
    let lexer = Lexer {};

    let tokens: Vec<Token> = lexer.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}
