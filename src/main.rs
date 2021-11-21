use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};

mod builtins;
mod array;
mod errors;
mod eval;
mod parser;

use array::A;
use eval::eval;
use parser::Ast;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, terminal, Result,
};

fn main() -> Result<()> {
    let mut buffer = String::new();

    let mut stack: Vec<A> = Vec::new();
    let mut words = HashMap::new();

    let mut args = env::args();
    if let Some(f) = args.nth(1) {
        buffer = fs::read_to_string(f).expect("Invalid file name.");
        for line in buffer.lines() {
            run(&line, &mut stack, &mut words, false);
        }
    } else {
        repl(&mut buffer, &mut stack, &mut words)?;
    }
    Ok(())
}

fn repl(buffer: &mut String, stack: &mut Vec<A>, words: &mut HashMap<String, Ast>) -> Result<()> {
    let mut debugging = false;

    loop {
        match event::read()? {
            Event::Key(k) => match k {
                KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                } => {
                    print!("{}", c);
                    buffer.push(c);
                    io::stdout().flush().unwrap();
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    if buffer.trim() == ".debug" {
                        debugging = !debugging;
                    }
                    if let Some(i) = buffer.char_indices().map(|(i, _)| i).nth(5) {
                        if buffer.len() > 5 && &buffer[..i] == ".load" {
                            let file = fs::read_to_string(&buffer[i..].trim())
                                .expect("Invalid file name.");
                            println!("");
                            for line in file.lines() {
                                run(&line, stack, words, debugging);
                            }
                        }
                    }

                    if buffer.trim() == ".quit" {
                        break;
                    }

                    println!("");
                    run(&buffer, stack, words, debugging);
                    io::stdout().flush().unwrap();

                    buffer.clear();
                }
                KeyEvent {
                    code: KeyCode::Tab, ..
                } => {
                    let l = buffer.len();
                    if l >= 1 {
                        match &buffer[l - 1..] {
                            "?" => {
                                print_help();
                                buffer.clear();
                            }
                            _ => {}
                        }
                    }
                }
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } => print_char_in_place(buffer, None),
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}

fn print_help() {
    println!("Builtin functions");
    println!("=================");
    println!("");
    println!("IO");
    println!("--");
    println!("");
    println!("_ Print the top stack value.");
    println!("");
    println!("Control flow");
    println!("------------");
    println!("");
    println!("if Conditionally evaluate the next code based on the truthiness of the top stack element. If falsy, continue from next then.");
    println!("then Marker to identify branch end.");
    println!("do Repeat the following word n times, where n is the value of the top stack element.");
    println!("");
    println!("Equality");
    println!("--------");
    println!("");
    println!("eql Test value-wise equality between top two stack elements.");
    println!("");
    println!("Arithmetic");
    println!("----------");
    println!("");
    println!("+ Value-wise addition between top two stack elements.");
    println!("- Value-wise subtraction between top two stack elements.");
    println!("* Value-wise multiplication between top two stack elements.");
    println!("/ Value-wise division between top two stack elements.");
    println!("");
    println!("All arithmetic operators repeat the top stack element, so [ 1 2 3 ] 1 + evaluates to [ 2 3 4 ].");
    println!("");
    println!("Array manipulation");
    println!("------------------");
    println!("");
    println!("cat Concatenate top stack element to the following stack element.");
    println!("trm Transmute the top stack element into individual elements.");
    println!("len Push the length of the top stack element onto the stack.");
    println!("");
    println!("Stack manipulation");
    println!("------------------");
    println!("");
    println!("dup Duplicate top stack element.");
    println!("pop Pop top stack element.");
    println!("swp Swap the top two stack elements.");
    println!("rot Move the bottom stack element to the top.");
    println!("clr Clear the stack.");
    println!("clr1 Clear all but the top stack element.");
    println!("");
    println!("Definitions");
    println!("-----------");
    println!("");
    println!(": Start word definition.");
    println!("; End word definition.");
    println!("[ Start element definition. If element only contains a single value, the brackets can be omitted.");
    println!("] End element definition.");
}

fn print_char_in_place(buffer: &mut String, c: Option<char>) {
    let _ = buffer.pop();
    let _ = execute!(
        io::stdout(),
        cursor::MoveLeft(1),
        terminal::Clear(terminal::ClearType::UntilNewLine)
    );
    if let Some(s) = c {
        buffer.push(s);
        print!("{}", s);
    }
    io::stdout().flush().unwrap();
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
