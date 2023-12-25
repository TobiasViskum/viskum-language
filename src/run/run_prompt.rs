use std::io::{ self, BufRead };

use super::run;

pub fn run_prompt() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        print!("> ");
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            run(&line.as_bytes());
        } else {
            break;
        }
    }
}
