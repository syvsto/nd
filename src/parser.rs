use std::collections::HashMap;
use std::error::Error;

use crate::data::{vf_to_u8, Ty, A};
use crate::errors::ErrorType;

#[derive(Debug, Clone, PartialEq)]
enum LexemeType {
    Number,
    Str,
    Array,
    Word,
    Print,
    If,
    And,
    Or,
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
                Token::Data(A::new_f(n))
            }
            LexemeType::Str => {
                let len = l.string.len();
                Token::Data(A::new(
                    Ty::C,
                    1,
                    len,
                    vec![len],
                    l.string.as_bytes().to_vec(),
                ))
            }
            LexemeType::Array => {
                let ws = l.string.split_whitespace();
                let ns: Result<Vec<_>, _> = ws.map(|w| w.parse::<f32>()).into_iter().collect();
                let ns = ns?;
                let len = ns.len();
                Token::Data(A::new(
                    Ty::F,
                    1,
                    len,
                    vec![len],
                    vf_to_u8(ns.as_slice()).to_vec(),
                ))
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
            LexemeType::And => Token::Builtin(Builtins::And),
            LexemeType::Or => Token::Builtin(Builtins::Or),
            LexemeType::Forward => Token::Builtin(Builtins::Forward),
            LexemeType::Plus => Token::Builtin(Builtins::Plus),
            LexemeType::Equal => Token::Builtin(Builtins::Equal),
            LexemeType::Minus => Token::Builtin(Builtins::Minus),
            LexemeType::Multiply => Token::Builtin(Builtins::Multiply),
            LexemeType::Divide => Token::Builtin(Builtins::Divide),
            LexemeType::Concat => Token::Builtin(Builtins::Concat),
            LexemeType::Duplicate => Token::Builtin(Builtins::Duplicate),
            LexemeType::Swap => Token::Builtin(Builtins::Swap),
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
    Duplicate,
    Swap,
    Clear,
    ClearButOne,
    Pop,
    If,
    And,
    Or,
    Forward,
    WordStart,
    WordEnd,
    ArrayStart,
    ArrayEnd,
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
            '⋀' => res.push(Lexeme::new("⋀", And)),
            '⋁' => res.push(Lexeme::new("⋁", Or)),
            '+' => res.push(Lexeme::new("+", Plus)),
            '-' => res.push(Lexeme::new("-", Minus)),
            '*' => res.push(Lexeme::new("*", Multiply)),
            '/' => res.push(Lexeme::new("/", Divide)),
            ',' => res.push(Lexeme::new(",", Concat)),
            '▶' => res.push(Lexeme::new("▶", Duplicate)),
            '◀' => res.push(Lexeme::new("◀", Pop)),
            '◆' => res.push(Lexeme::new("◆", Swap)),
            '▮' => res.push(Lexeme::new("▮", Clear)),
            '▯' => res.push(Lexeme::new("▯", ClearButOne)),
            '=' => res.push(Lexeme::new("=", Equal)),
            '?' => res.push(Lexeme::new("?", If)),
            '→' => res.push(Lexeme::new("→", Forward)),
            '_' => res.push(Lexeme::new("_", Print)),
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
                let p = cs.clone().position(|c| c.is_whitespace()).unwrap_or(1);
                let l: String = cs.clone().collect::<Vec<_>>()[..p].into_iter().collect();
                while let Some(cm) = cs.peek() {
                    if !cm.is_whitespace() {
                        cs.next();
                    } else {
                        break;
                    }
                }
                res.push(Lexeme::new(l.trim(), Word));
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
