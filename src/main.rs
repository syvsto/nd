use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};

mod data;
mod errors;
mod eval;
mod parser;
use eval::{eval};
use parser::Ast;
use data::A;

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    let mut stack = Vec::new();
    let mut words = HashMap::new();

    let mut args = env::args();
    if let Some(f) = args.nth(1) {
        buffer = fs::read_to_string(f).expect("Invalid file name.");
        for line in buffer.lines() {
            run(&line, &mut stack, &mut words, false);
        }
    } else {
        repl(&mut buffer, &mut stack, &mut words);
    }
    Ok(())
}

fn repl(buffer: &mut String, stack: &mut Vec<A>, words: &mut HashMap<String, Ast>) -> io::Result<()> {
    let mut debugging = false;

    loop {
        buffer.clear();
        io::stdin().read_line(buffer)?;
        io::stdout().flush().unwrap();

        run(&buffer, stack, words, debugging);

        io::stdout().flush().unwrap();

        if buffer.trim() == ".debug" {
            debugging = !debugging;
        }
        if &buffer[..5] == ".load" {
            let file = fs::read_to_string(&buffer[6..].trim()).expect("Invalid file name.");
            for line in file.lines() {
                run(&line, stack, words, debugging);
            }
        }

        if buffer.trim() == ".quit" {
            break;
        }
    }
    Ok(())
}


fn run(buffer: &str, stack: &mut Vec<A>, words: &mut HashMap<String, Ast>, debugging: bool) {
    match parser::parse(buffer) {
        Ok((tokens, w)) => {
            words.extend(w);
            let _ = eval(&tokens, stack, words, debugging);
        }
        Err(err) => println!("Couldn't parse line: {:?}", err),
    }
}
