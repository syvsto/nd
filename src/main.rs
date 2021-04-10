use std::io::{self, Write};

#[derive(Debug)]
struct Lexemes {
    words: Vec<String>,
}

#[derive(Debug)]
struct AST {
    tokens: Vec<Token>,
}

#[derive(Debug)]
enum Token {
    Word(String),
    Number(f64),
    Builtin(Builtins),
}

impl Token {
    fn parse(word: &str) -> Self {
        use Builtins::*;
        use Token::*;

        if let Ok(w) = word.trim().parse::<f64>() {
            return Number(w);
        }

        match word {
            "print" => Builtin(Print),
            "+" => Builtin(Plus),
            "-" => Builtin(Minus),
            "*" => Builtin(Multiply),
            "/" => Builtin(Divide),
            _ => Word(word.to_string()),
        }
    }
}

#[derive(Debug)]
enum Builtins {
    Print,
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Builtins {
    fn eval(&self, stack: &mut Vec<f64>) {
        use Builtins::*;
        match self {
            Print => {
                if let Some((last, _)) = stack.split_last() {
                    println!("{}", last);
                }
            }
            Plus => {
                let n1 = stack.pop().expect("Couldn't pop from stack");
                let n2 = stack.pop().expect("Couldn't pop from stack");
                stack.push(n1 + n2);
            }
            Minus => {
                let n1 = stack.pop().expect("Couldn't pop from stack");
                let n2 = stack.pop().expect("Couldn't pop from stack");
                stack.push(n1 - n2);
            }
            Multiply => {
                let n1 = stack.pop().expect("Couldn't pop from stack");
                let n2 = stack.pop().expect("Couldn't pop from stack");
                stack.push(n1 * n2);
            }
            Divide => {
                let n1 = stack.pop().expect("Couldn't pop from stack");
                let n2 = stack.pop().expect("Couldn't pop from stack");
                stack.push(n1 / n2);
            }
        }
    }
}

fn lex(buf: &str) -> Lexemes {
    Lexemes {
        words: buf
            .split(' ')
            .filter(|x| *x != "")
            .map(|x| x.trim().to_string())
            .collect(),
    }
}

fn parse(lexemes: Lexemes) -> AST {
    let mut v = vec![];
    for word in &lexemes.words {
        v.push(Token::parse(word));
    }
    AST { tokens: v }
}

fn eval(ast: AST, stack: &mut Vec<f64>) {
    use Token::*;

    for node in &ast.tokens {
        match node {
            Word(_) => continue,
            Number(n) => stack.push(*n),
            Builtin(func) => func.eval(stack),
        }
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    let mut stack = Vec::new();

    loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        io::stdout().flush().unwrap();

        let lexemes = lex(&buffer);
        let ast = parse(lexemes);
        eval(ast, &mut stack);

        io::stdout().flush().unwrap();

        if buffer.trim() == "quit" {
            break;
        }
    }
    Ok(())
}
