use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};

mod errors;
mod eval;
mod parser;
use eval::{eval, Val};
use parser::Ast;

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    let mut debugging = false;

    let mut stack = Vec::new();
    let mut words = HashMap::new();

    let mut args = env::args();
    if let Some(f) = args.nth(1) {
        buffer = fs::read_to_string(f).expect("Invalid file name.");
        for line in buffer.lines() {
            repl(&line, &mut stack, &mut words, debugging);
        }
    } else {
        loop {
            buffer.clear();
            io::stdin().read_line(&mut buffer)?;
            io::stdout().flush().unwrap();

            repl(&buffer, &mut stack, &mut words, debugging);

            io::stdout().flush().unwrap();

            if buffer.trim() == ".debug" {
                debugging = !debugging;
            }
            if buffer.trim() == ".quit" {
                break;
            }
        }
    }
    Ok(())
}


fn repl(buffer: &str, stack: &mut Vec<Val>, words: &mut HashMap<String, Ast>, debugging: bool) {
    match parser::parse(buffer) {
        Ok((tokens, w)) => {
            words.extend(w);
            eval(&tokens, stack, words, debugging);
        }
        Err(err) => println!("Couldn't parse line: {:?}", err),
    }
}
