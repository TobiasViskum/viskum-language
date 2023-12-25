use std::io::{ BufReader, Result, Read };
use std::fs::File;
use std::process;

use crate::print_util::print_error;

use super::run;

pub fn run_file(path: &String) -> Result<()> {
    let f = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            print_error(format!("Could not read file: {}", path).as_str());
            process::exit(64)
        }
    };
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();
    let _ = reader.read_to_end(&mut buf);

    run(&buf);
    Ok(())
}
