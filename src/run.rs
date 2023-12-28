use std::rc::Rc;
use std::cell::RefCell;

use crate::ast_printer::AstPrinter;
use crate::error_handler::ErrorHandler;
use crate::parser::Parser;
use crate::print_util::print_error;
use crate::lexer::Lexer;

use std::io::{ self, BufRead, Result, stdout, Write };
use std::process;

fn run(source: &str) {
    let error_handler: Rc<RefCell<ErrorHandler>> = Rc::new(RefCell::new(ErrorHandler::new()));

    let mut lexer = Lexer::new(source.to_string(), &error_handler);

    let tokens = lexer.scan_tokens();

    let has_error = &error_handler.borrow().has_error();

    if *has_error {
        error_handler.borrow_mut().print_errors();
    } else {
        if let Ok(tokens) = tokens {
            let mut parser = Parser::new(tokens, &error_handler);

            if let Some(expr) = parser.parse() {
                // println!("{:?}", expr)
                AstPrinter.print(&expr);
            } else {
                error_handler.borrow_mut().print_errors()
            }
        }
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

    print!("> ");
    let _ = stdout().flush();

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            run(line.as_str());
            print!("> ");
            let _ = stdout().flush();
        } else {
            break;
        }
    }
}
