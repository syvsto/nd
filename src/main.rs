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
enum Val {
    Float(f64),
    Bool(bool),
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
            "branch" => Builtin(Branch),
            "branch?" => Builtin(Cond),
            "+" => Builtin(Plus),
            "=" => Builtin(Equal),
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
    Equal,
    Minus,
    Multiply,
    Divide,
    Branch,
    Cond,
}

impl Builtins {
    fn eval(&self, stack: &mut Vec<Val>) -> Option<usize> {
        use Builtins::*;
        match self {
            Print => {
                if let Some((last, _)) = stack.split_last() {
                    println!("{:?}", last);
                }
            }

            Plus => {
                let n1 = stack.pop().expect("Couldn't pop from stack");
                let n2 = stack.pop().expect("Couldn't pop from stack");
                match (n1, n2) {
                    (Val::Float(a), Val::Float(b)) => stack.push(Val::Float(a + b)),
                    _ => panic!("Couldn't add values, not all values were numbers."),
                }
            }

            Equal => {
                let n1 = stack.pop().expect("Couldn't pop from stack");
                let n2 = stack.pop().expect("Couldn't pop from stack");
                match (n1, n2) {
                    (Val::Float(a), Val::Float(b)) => {
                        if a == b {
                            stack.push(Val::Bool(true))
                        } else {
                            stack.push(Val::Bool(false))
                        }
                    }
                    _ => panic!("Couldn't compare values, not all values were numbers."),
                }
            }

            Minus => {
                let n1 = stack.pop().expect("Couldn't pop from stack");
                let n2 = stack.pop().expect("Couldn't pop from stack");
                match (n1, n2) {
                    (Val::Float(a), Val::Float(b)) => stack.push(Val::Float(a - b)),
                    _ => panic!("Couldn't subtract values, not all values were numbers."),
                }
            }

            Multiply => {
                let n1 = stack.pop().expect("Couldn't pop from stack");
                let n2 = stack.pop().expect("Couldn't pop from stack");
                match (n1, n2) {
                    (Val::Float(a), Val::Float(b)) => stack.push(Val::Float(a * b)),
                    _ => panic!("Couldn't multiply values, not all values were numbers."),
                }
            }

            Divide => {
                let n1 = stack.pop().expect("Couldn't pop from stack");
                let n2 = stack.pop().expect("Couldn't pop from stack");
                match (n1, n2) {
                    (Val::Float(a), Val::Float(b)) => stack.push(Val::Float(a / b)),
                    _ => panic!("Couldn't divide values, not all values were numbers."),
                }
            }

            Branch => {
                let idx = stack.pop().expect("Couldn't pop from stack");
                match idx {
                    Val::Float(a) => return Some(a as usize),
                    _ => panic!("Couldn't get index to stack location, wrong type."),
                }
            }
            Cond => {
                let idx = stack.pop().expect("Couldn't pop from stack");
                let comparison = stack.pop().expect("Couldn't pop from stack");
                match (idx, comparison) {
                    (Val::Float(i), Val::Bool(cmp)) => {
                        if cmp {
                            return Some(i as usize);
                        }
                    }
                    _ => panic!("Wrong type in comparison or index."),
                }
            }
        }
        None
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

fn eval(ast: AST, stack: &mut Vec<Val>, debug: bool) {
    use Token::*;

    let mut i = 0;
    while i < ast.tokens.len() {
        match &ast.tokens[i] {
            Word(_) => (),
            Number(n) => stack.push(Val::Float(*n)),
            Builtin(func) => {
                if let Some(idx) = func.eval(stack) {
                    i = idx;
                }
            }
        }
        if debug {
            println!("{:?}", stack);
        }
        i += 1;
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    let mut stack = Vec::new();

    let debugging = true;

    loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        io::stdout().flush().unwrap();

        let lexemes = lex(&buffer);
        let ast = parse(lexemes);
        eval(ast, &mut stack, debugging);

        io::stdout().flush().unwrap();

        if buffer.trim() == "quit" {
            break;
        }
    }
    Ok(())
}
