mod print_util;
mod run;
mod error_handler;
mod lexer;
mod token;
mod parser;
mod interpreter;
mod util;
mod expr;

use print_util::print_error;

use crate::run::{ run_file, run_prompt };

use std::env::args;
use std::process;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        print_error("Unexpected number of arguments. Expected 0-1 arguments.");
        process::exit(64);
    }

    if args.len() == 2 {
        let _ = run_file(&args[1]);
    } else {
        run_prompt();
    }
}
