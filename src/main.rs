mod print_util;
mod run;
mod error_handler;
mod lexer;
mod token;
mod parser;
mod interpreter;
mod util;
mod expr;
mod stmt;
mod ast_printer;
mod environment;
mod viskum_callable;
mod viskum_function;

use print_util::print_error;
use run::Viskum;
use std::env::args;
use std::process;

fn main() {
    let args: Vec<String> = args().collect();

    let viskum = Viskum::new();

    if args.len() > 2 {
        print_error("Unexpected number of arguments. Expected 0-1 arguments.");
        process::exit(64);
    }

    if args.len() == 2 {
        let _ = viskum.run_file(&args[1]);
    } else {
        viskum.run_prompt();
    }
}
