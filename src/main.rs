use std::io::{self, Write};

mod parser;
mod eval;
use eval::eval;

fn main() -> io::Result<()> {
    let mut buffer = String::new();


    let debugging = false;

    loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        io::stdout().flush().unwrap();

        let ast = parser::parse(&buffer);
        eval(&ast, debugging);

        io::stdout().flush().unwrap();

        if buffer.trim() == "quit" {
            break;
        }
    }
    Ok(())
}
