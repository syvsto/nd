use std::collections::HashMap;
use std::io::{self, Write};

mod eval;
mod parser;
use eval::eval;

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    let debugging = false;

    let mut stack = Vec::new();
    let mut words = HashMap::new();

    loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        io::stdout().flush().unwrap();

        let line = buffer.clone();
        let (tokens, w) = parser::parse(&line);
        words.extend(w);
        eval(&tokens, &mut stack, &words, debugging);

        io::stdout().flush().unwrap();

        if buffer.trim() == "quit" {
            break;
        }
    }
    Ok(())
}
