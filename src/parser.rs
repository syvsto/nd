use std::collections::HashMap;
use std::error::Error;

use crate::data::A;
use crate::errors::ErrorType;

#[derive(Debug, Clone, PartialEq)]
enum LexemeType {
    Number,
    Str,
    Array,
    Word,
    Print,
    If,
    Do,
    And,
    Or,
    Forward,
    Plus,
    Equal,
    Minus,
    Multiply,
    Divide,
    Concat,
    Len,
    Transmute,
    Duplicate,
    Swap,
    Rotate,
    Clear,
    ClearButOne,
    Pop,
    Definition,
}

#[derive(Debug, Clone, PartialEq)]
struct Lexeme {
    string: String,
    ty: LexemeType,
}

impl Lexeme {
    fn new(st: &str, ty: LexemeType) -> Self {
        Self {
            string: st.to_string(),
            ty,
        }
    }
}

pub type Ast = Vec<Token>;

fn parse_ast(lexemes: &[Lexeme]) -> Result<Ast, Box<dyn Error>> {
    let tokens: Result<Vec<_>, _> = lexemes.iter().map(|l| Token::parse(l)).collect();
    Ok(tokens?)
}

fn resolve_words(tokens: &[Token]) -> HashMap<String, Ast> {
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
pub enum Token {
    Word(String),
    Data(A),
    Builtin(Builtins),
    Definition((String, Ast)),
}

impl Token {
    fn parse(l: &Lexeme) -> Result<Self, Box<dyn Error>> {
        let t = match l.ty {
            LexemeType::Number => {
                let n = l.string.parse::<f32>()?;
                Token::Data(A::F(vec![n]))
            }
            LexemeType::Str => Token::Data(A::C(l.string.chars().collect::<Vec<_>>())),
            LexemeType::Array => {
                let ws = l.string.split_whitespace();
                let ns: Result<Vec<_>, _> = ws.map(|w| w.parse::<f32>()).into_iter().collect();
                let ns = ns?;
                Token::Data(A::F(ns))
            }
            LexemeType::Definition => {
                let ws = lex(&l.string)?;
                if let Some((name, definition)) = ws.split_first() {
                    Token::Definition((name.string.clone(), parse_ast(definition)?))
                } else {
                    return Err(Box::new(ErrorType::Parse));
                }
            }
            LexemeType::Word => Token::Word(l.string.clone()),
            LexemeType::Print => Token::Builtin(Builtins::Print),
            LexemeType::If => Token::Builtin(Builtins::If),
            LexemeType::Do => Token::Builtin(Builtins::Do),
            LexemeType::And => Token::Builtin(Builtins::And),
            LexemeType::Or => Token::Builtin(Builtins::Or),
            LexemeType::Forward => Token::Builtin(Builtins::Forward),
            LexemeType::Plus => Token::Builtin(Builtins::Plus),
            LexemeType::Equal => Token::Builtin(Builtins::Equal),
            LexemeType::Minus => Token::Builtin(Builtins::Minus),
            LexemeType::Multiply => Token::Builtin(Builtins::Multiply),
            LexemeType::Divide => Token::Builtin(Builtins::Divide),
            LexemeType::Concat => Token::Builtin(Builtins::Concat),
            LexemeType::Len => Token::Builtin(Builtins::Len),
            LexemeType::Transmute => Token::Builtin(Builtins::Transmute),
            LexemeType::Duplicate => Token::Builtin(Builtins::Duplicate),
            LexemeType::Swap => Token::Builtin(Builtins::Swap),
            LexemeType::Rotate => Token::Builtin(Builtins::Rotate),
            LexemeType::Clear => Token::Builtin(Builtins::Clear),
            LexemeType::ClearButOne => Token::Builtin(Builtins::ClearButOne),
            LexemeType::Pop => Token::Builtin(Builtins::Pop),
        };

        Ok(t)
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
    Concat,
    Len,
    Transmute,
    Duplicate,
    Swap,
    Rotate,
    Clear,
    ClearButOne,
    Pop,
    If,
    Do,
    And,
    Or,
    Forward,
}

fn lex(buf: &str) -> Result<Vec<Lexeme>, ErrorType> {
    use LexemeType::*;

    let mut cs = buf.chars().peekable();
    let mut res = vec![];
    while let Some(c) = cs.peek() {
        match c {
            '#' => {
                while let Some(cm) = cs.next() {
                    if cm == '\n' {
                        break;
                    }
                }
            }
            '"' => {
                cs.next();
                let p = cs.clone().position(|c| c == '"').ok_or(ErrorType::Parse)?;
                let l: String = cs.clone().collect::<Vec<_>>()[..p].into_iter().collect();
                while let Some(cm) = cs.next() {
                    if cm == '"' {
                        break;
                    }
                }
                res.push(Lexeme::new(l.trim(), Str));
            }
            '[' => {
                let p = cs.clone().position(|c| c == ']').ok_or(ErrorType::Parse)?;
                let l: String = cs.clone().collect::<Vec<_>>()[1..p - 1]
                    .into_iter()
                    .collect();
                while let Some(cm) = cs.next() {
                    if cm == ']' {
                        break;
                    }
                }
                res.push(Lexeme::new(l.trim(), Array));
            }
            ':' => {
                let p = cs.clone().position(|c| c == ';').ok_or(ErrorType::Parse)?;
                let l: String = cs.clone().collect::<Vec<_>>()[1..p - 1]
                    .into_iter()
                    .collect();
                while let Some(cm) = cs.next() {
                    if cm == ';' {
                        break;
                    }
                }
                res.push(Lexeme::new(l.trim(), Definition));
            }
            a if a.is_digit(10) => {
                let p = cs.clone().position(|c| c.is_whitespace()).unwrap_or(1);
                let l: String = cs.clone().collect::<Vec<_>>()[..p].into_iter().collect();
                while let Some(cm) = cs.peek() {
                    if !cm.is_whitespace() {
                        cs.next();
                    } else {
                        break;
                    }
                }
                res.push(Lexeme::new(l.trim(), Number));
            }
            a if !a.is_whitespace() => {
                let mut s = String::new();
                while let Some(cm) = cs.peek() {
                    if !cm.is_whitespace() {
                        s.push(cs.next().ok_or(ErrorType::Parse)?);
                    } else {
                        break;
                    }
                }
                match s.as_ref() {
                    "and" => res.push(Lexeme::new("and", And)),
                    "or" => res.push(Lexeme::new("or", Or)),
                    "+" => res.push(Lexeme::new("+", Plus)),
                    "-" => res.push(Lexeme::new("-", Minus)),
                    "*" => res.push(Lexeme::new("*", Multiply)),
                    "/" => res.push(Lexeme::new("/", Divide)),
                    "cat" => res.push(Lexeme::new("cat", Concat)),
                    "len" => res.push(Lexeme::new("len", Len)),
                    "trm" => res.push(Lexeme::new("trm", Transmute)),
                    "dup" => res.push(Lexeme::new("dup", Duplicate)),
                    "pop" => res.push(Lexeme::new("pop", Pop)),
                    "swp" => res.push(Lexeme::new("swp", Swap)),
                    "rot" => res.push(Lexeme::new("rot", Rotate)),
                    "clr" => res.push(Lexeme::new("clr", Clear)),
                    "clr1" => res.push(Lexeme::new("clr1", ClearButOne)),
                    "eql" => res.push(Lexeme::new("eql", Equal)),
                    "do" => res.push(Lexeme::new("do", Do)),
                    "if" => res.push(Lexeme::new("if", If)),
                    "then" => res.push(Lexeme::new("then", Forward)),
                    "_" => res.push(Lexeme::new("_", Print)),
                    _ => res.push(Lexeme::new(s.trim(), Word)),
                }
            }
            _ => {}
        }
        cs.next();
    }

    Ok(res)
}

pub fn parse(buf: &str) -> Result<(Ast, HashMap<String, Ast>), Box<dyn Error>> {
    let lexemes = lex(&buf);
    let ast = parse_ast(&lexemes?)?;
    let words = resolve_words(&ast);
    Ok((ast, words))
}
