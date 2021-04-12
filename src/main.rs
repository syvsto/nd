use std::io::{self, Write};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Lexemes {
    words: Vec<String>,
}

#[derive(Debug)]
struct AST {
    tokens: Vec<Token>,
}
 
#[derive(Debug)]
enum Control {
    ContinueToThen,
    DefineWord,
}

#[derive(Debug)]
enum Val {
    Float(f64),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
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
            "if" => Builtin(If),
            "then" => Builtin(Then),
            "+" => Builtin(Plus),
            "=" => Builtin(Equal),
            "-" => Builtin(Minus),
            "*" => Builtin(Multiply),
            "/" => Builtin(Divide),
            _ => Word(word.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Builtins {
    Print,
    Plus,
    Equal,
    Minus,
    Multiply,
    Divide,
    If,
    Then,
    WordStart,
    WordEnd,
}

impl Builtins {
    fn eval(&self, stack: &mut Vec<Val>) -> Option<Control> {
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

            Then => {}

            If => {
                let comparison = stack.pop().expect("Couldn't pop from stack");
                match comparison {
                    Val::Bool(cmp) => {
                        if !cmp {
                            return Some(Control::ContinueToThen);
                        }
                    }
                    _ => panic!("Wrong type in comparison or index."),
                }
            }

            WordStart => {
                return Some(Control::DefineWord);
            }

            WordEnd => {}
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

fn next_of_type(ty: Builtins, tokens: &[Token]) -> Option<usize> {
   tokens.iter()
       .position(|token| match token {
           Token::Builtin(t) => *t == ty,
           _ => false
       })
}

fn peek_word(tokens: &[Token]) -> Option<String> {
    match &tokens[0] {
        Token::Word(name) => Some(name.clone()),
        _ => None,
    }
}

fn eval(ast: &AST, stack: &mut Vec<Val>, words: Rc<RefCell<HashMap<String, AST>>>, debug: bool) {
    use Token::*;

    let mut i = 0;
    while i < ast.tokens.len() {
        match &ast.tokens[i] {
            Word(name) => {
                if let Some(word) = words.borrow().get(name) {
                    eval(word, stack, words.clone(), debug);
                }
            }
            Number(n) => stack.push(Val::Float(*n)),
            Builtin(func) => {
                if let Some(ctrl) = func.eval(stack) {
                    match ctrl {
                        Control::ContinueToThen => {
                            if let Some(v) = next_of_type(Builtins::Then, &ast.tokens) {
                                i = v;
                            }
                        }
                        Control::DefineWord => {
                            let mut word = AST { tokens:  vec![] };
                            let name = peek_word(&ast.tokens[i..]).expect("First token in function definition wasn't a word.");
                            while ast.tokens[i] != Builtin(Builtins::WordEnd) {
                                word.tokens.push(ast.tokens[i].clone())
                            }
                            words.borrow_mut().insert(name, word);
                        }
                    }
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
    let mut words = Rc::new(RefCell::new(HashMap::new()));

    let debugging = false;

    loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        io::stdout().flush().unwrap();

        let lexemes = lex(&buffer);
        let ast = parse(lexemes);
        eval(&ast, &mut stack, words.clone(), debugging);

        io::stdout().flush().unwrap();

        if buffer.trim() == "quit" {
            break;
        }
    }
    Ok(())
}
