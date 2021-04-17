use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum LexemeType {
    Number(f64),
    Char(char),
    Word(String),
    Print,
    If,
    Forward,
    Plus,
    Equal,
    Minus,
    Multiply,
    Divide,
    Concat,
    Duplicate,
    Swap,
    Clear,
    WordStart,
    WordEnd,
    ArrayStart,
    ArrayEnd,
}

impl LexemeType {
    fn new(s: &str) -> LexemeType {
        if let Ok(w) = s.trim().parse::<f64>() {
            return LexemeType::Number(w);
        }

        let mut cs = s.chars();
        if cs.nth(0) == Some('\'') {
            return LexemeType::Char(cs.nth(0).expect("Couldn't parse character."));
        }

        match s {
            "=>" => LexemeType::Print,
            "?" => LexemeType::If,
            "->" => LexemeType::Forward,
            "+" => LexemeType::Plus,
            "=" => LexemeType::Equal,
            "-" => LexemeType::Minus,
            "*" => LexemeType::Multiply,
            "/" => LexemeType::Divide,
            "," => LexemeType::Concat,
            "<>" => LexemeType::Duplicate,
            "><" => LexemeType::Swap,
            "_" => LexemeType::Clear,
            ":" => LexemeType::WordStart,
            ";" => LexemeType::WordEnd,
            "[" => LexemeType::ArrayStart,
            "]" => LexemeType::ArrayEnd,
            _ => LexemeType::Word(s.to_string()),
        }
    }

    fn is_number(&self) -> bool {
        match self {
            LexemeType::Number(_) => true,
            _ => false,
        }
    }

    fn get_primitive_number(&self) -> Option<f64> {
        match self {
            LexemeType::Number(n) => Some(*n),
            _ => None,
        }
    }

    fn is_char(&self) -> bool {
        match self {
            LexemeType::Char(_) => true,
            _ => false,
        }
    }

    fn get_primitive_char(&self) -> Option<char> {
        match self {
            LexemeType::Char(c) => Some(*c),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Lexeme {
    string: String,
    ty: LexemeType,
}

impl Lexeme {
    fn new(s: &str) -> Self {
        Self {
            string: s.to_string(),
            ty: LexemeType::new(s),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TokenStack {
    pub tokens: Vec<Token>,
}

impl TokenStack {
    fn tokenize(lexemes: &[Lexeme]) -> Self {
        let mut i = 0;

        let mut tokens = Vec::with_capacity(lexemes.len());

        while i < lexemes.len() {
            match &lexemes[i].ty {
                LexemeType::Number(n) => tokens.push(Token::Data(DataType::Number(vec![*n]))),
                LexemeType::Char(c) => tokens.push(Token::Data(DataType::Char(vec![*c]))),
                LexemeType::Word(w) => tokens.push(Token::Word(w.to_string())),
                LexemeType::Print => tokens.push(Token::Builtin(Builtins::Print)),
                LexemeType::If => tokens.push(Token::Builtin(Builtins::If)),
                LexemeType::Forward => tokens.push(Token::Builtin(Builtins::Forward)),
                LexemeType::Plus => tokens.push(Token::Builtin(Builtins::Plus)),
                LexemeType::Equal => tokens.push(Token::Builtin(Builtins::Equal)),
                LexemeType::Minus => tokens.push(Token::Builtin(Builtins::Minus)),
                LexemeType::Multiply => tokens.push(Token::Builtin(Builtins::Multiply)),
                LexemeType::Divide => tokens.push(Token::Builtin(Builtins::Divide)),
                LexemeType::Concat => tokens.push(Token::Builtin(Builtins::Concat)),
                LexemeType::Duplicate => tokens.push(Token::Builtin(Builtins::Duplicate)),
                LexemeType::Swap => tokens.push(Token::Builtin(Builtins::Swap)),
                LexemeType::Clear => tokens.push(Token::Builtin(Builtins::Clear)),
                LexemeType::WordStart => tokens.push(Token::Definition(parse_word(&lexemes[i..]))),
                LexemeType::WordEnd => {}
                LexemeType::ArrayStart => {
                    let (next_i, token) = parse_data(&lexemes[i..]).expect("Couldn't parse data");
                    i += next_i;
                    tokens.push(token);
                }
                LexemeType::ArrayEnd => {}
            }

            i += 1;
        }

        Self { tokens }
    }
}

fn parse_data(lexemes: &[Lexeme]) -> Option<(usize, Token)> {
    let end = lexemes
        .iter()
        .position(|x| x.ty == LexemeType::ArrayEnd)
        .expect("Couldn't get array end delimiter.");

    let is_number = lexemes[1..end].iter().all(|x| x.ty.is_number());
    if is_number {
        let numbers = lexemes[1..end]
            .iter()
            .map(|x| x.ty.get_primitive_number().expect("Couldn't get numbers."))
            .collect();
        return Some((end, Token::Data(DataType::Number(numbers))));
    }

    let is_char = lexemes[1..end].iter().all(|x| x.ty.is_char());
    if is_char {
        let chars = lexemes[1..end]
            .iter()
            .map(|x| x.ty.get_primitive_char().expect("Couldn't get characters."))
            .collect();
        return Some((end, Token::Data(DataType::Char(chars))));
    }

    None
}

fn parse_word(lexemes: &[Lexeme]) -> (String, TokenStack) {
    let end = lexemes
        .iter()
        .position(|x| x.ty == LexemeType::WordEnd)
        .expect("Couldn't get word end delimiter");

    if let Some((name, definition)) = &lexemes[1..end].split_first() {
        if let LexemeType::Word(n) = &name.ty {
            return (n.clone(), TokenStack::tokenize(definition));
        }
    }

    panic!("Couldn't parse word definition.");
}

fn resolve_words(tokens: &[Token]) -> HashMap<String, TokenStack> {
    let mut words = HashMap::new();
    for token in tokens {
        match token {
            Token::Definition((name, ast)) => {
                words.insert(name.to_string(), ast.clone());
            }
            _ => (),
        }
    }
    words
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Number(Vec<f64>),
    Char(Vec<char>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Word(String),
    Data(DataType),
    Builtin(Builtins),
    Definition((String, TokenStack)),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Builtins {
    Print,
    Plus,
    Equal,
    Minus,
    Multiply,
    Divide,
    Concat,
    Duplicate,
    Swap,
    Clear,
    If,
    Forward,
    WordStart,
    WordEnd,
    ArrayStart,
    ArrayEnd,
}

fn lex(buf: &str) -> Vec<Lexeme> {
    buf.split(' ')
        .filter(|x| *x != "")
        .map(|x| Lexeme::new(x.trim()))
        .collect()
}

pub fn parse(buf: &str) -> (TokenStack, HashMap<String, TokenStack>) {
    let lexemes = lex(buf);
    let ast = TokenStack::tokenize(&lexemes);
    let words = resolve_words(&ast.tokens);
    (ast, words)
}
