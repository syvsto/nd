#[derive(Debug)]
struct Lexemes {
    words: Vec<String>,
}

#[derive(Debug)]
pub struct TokenStack {
    pub tokens: Vec<Token>,
}
 
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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
pub enum Builtins {
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

fn lex(buf: &str) -> Lexemes {
    Lexemes {
        words: buf
            .split(' ')
            .filter(|x| *x != "")
            .map(|x| x.trim().to_string())
            .collect(),
    }
}

fn tokenize(lexemes: Lexemes) -> TokenStack {
    let v = lexemes.words.iter()
        .map(|word| Token::parse(word))
        .collect();

    TokenStack { tokens: v }
}

pub fn parse(buf: &str) -> TokenStack {
    tokenize(lex(buf))
}
