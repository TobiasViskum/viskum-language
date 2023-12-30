use std::rc::Rc;
use std::cell::RefCell;

use crate::environment::Environment;
// use crate::ast_printer::AstPrinter;
use crate::error_handler::ErrorHandler;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::print_util::print_error;
use crate::lexer::Lexer;

use std::io::{ self, BufRead, Result, stdout, Write };
use std::process;

pub struct Viskum {
    environment: RefCell<Rc<RefCell<Environment>>>,
}

impl Viskum {
    pub fn new() -> Self {
        let environment = RefCell::new(Rc::new(RefCell::new(Environment::new())));
        Viskum { environment }
    }

    pub fn run_file(&self, path: &String) -> Result<()> {
        match std::fs::read_to_string(path) {
            Ok(str) => self.run(str.as_str()),
            Err(e) => {
                print_error(format!("There was an error while reading file: {}", e).as_str());
                process::exit(64);
            }
        }

        Ok(())
    }

    pub fn run_prompt(&self) {
        let stdin = io::stdin();
        print!("> ");
        let _ = stdout().flush();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    break;
                }
                if line == "@" {
                    println!("{:?}", self.environment);
                } else {
                    self.run(line.as_str());
                }
            } else {
                break;
            }
            print!("> ");
            let _ = stdout().flush();
        }
    }

    pub fn run(&self, source: &str) {
        let error_handler = RefCell::new(ErrorHandler::new());

        let mut lexer = Lexer::new(source.to_string(), &error_handler);

        let tokens = lexer.scan_tokens();

        if let Ok(tokens) = tokens {
            let mut parser = Parser::new(tokens, &error_handler);

            match parser.parse() {
                Ok(stmts) => {
                    let has_error = &error_handler.borrow().has_error();

                    if !*has_error {
                        let interpreter = Interpreter::new(&error_handler, &self.environment);
                        let _ = interpreter.interpret(stmts);
                        // AstPrinter.print(&expr);
                    } else {
                        error_handler.borrow_mut().print_errors();
                    }
                }
                Err(e) => {
                    error_handler.borrow_mut().report_error(e);
                    error_handler.borrow_mut().print_errors();
                }
            }
        }
    }
}
