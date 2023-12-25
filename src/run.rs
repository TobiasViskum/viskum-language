use crate::print_util::print_error;
use crate::lexer::Lexer;
use crate::token::Token;

use std::io::{ self, BufRead };
use std::io::Result;
use std::process;

fn run(source: &str) {
    let lexer = Lexer { source: source, tokens: vec![] };

    let tokens: Vec<Token> = lexer.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}

pub fn run_file(path: &String) -> Result<()> {
    match std::fs::read_to_string(path) {
        Ok(str) => run(str.as_str()),
        Err(e) => {
            print_error(format!("There was an error while reading file: {}", e).as_str());
            process::exit(64);
        }
    }

    Ok(())
}

pub fn run_prompt() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        print!("> ");
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            run(line.as_str());
        } else {
            break;
        }
    }
}
