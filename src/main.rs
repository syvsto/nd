use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};

mod builtins;
mod data;
mod errors;
mod eval;
mod parser;

use data::A;
use eval::eval;
use parser::Ast;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, terminal, Result,
};

fn main() -> Result<()> {
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
                            "d" => {
                                print_char_in_place(buffer, Some('▶'));
                            }
                            "p" => {
                                print_char_in_place(buffer, Some('◀'));
                            }
                            "#" => {
                                print_char_in_place(buffer, Some('◆'));
                            }
                            "!" => {
                                print_char_in_place(buffer, Some('▮'));
                            }
                            "?" => {
                                print_char_in_place(buffer, Some('▯'));
                            }
                            ">" => {
                                print_char_in_place(buffer, Some('→'));
                            }
                            "^" => {
                                print_char_in_place(buffer, Some('⋀'));
                            }
                            "v" => {
                                print_char_in_place(buffer, Some('⋁'));
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
