use std::io::{self, Write};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

mod eval;
mod parser;
use eval::eval;

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    let debugging = true;

    let mut stack = Vec::new();
    let words = Rc::new(RefCell::new(HashMap::new()));

    loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        io::stdout().flush().unwrap();

        let ast = parser::parse(&buffer);
        eval(&ast, &mut stack, words.clone(), debugging);

        io::stdout().flush().unwrap();

        if buffer.trim() == "quit" {
            break;
        }
    }
    Ok(())
}
