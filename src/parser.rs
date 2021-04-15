use std::collections::HashMap;

#[derive(Debug)]
struct Lexemes {
    words: Vec<String>,
}

#[derive(Debug)]
pub struct TokenStack {
    pub tokens: Vec<Token>,
}

impl TokenStack {
    pub fn new_from_tokens(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_vec(),
        }
    }

    fn tokenize(lexemes: Lexemes) -> Self {
        let v = lexemes
            .words
            .iter()
            .map(|word| Token::parse(word))
            .collect();

        Self { tokens: v }
    }

    fn resolve_words(&self) -> HashMap<String, TokenStack> {
        let token_start: Vec<_> = self.tokens.iter().enumerate().filter(|(_, token)| {
            match_token(token, Builtins::WordStart)
        }).collect();
        let token_end: Vec<_> = self.tokens.iter().enumerate().filter(|(_, token)| {
             match_token(token, Builtins::WordEnd)
        }).collect();

        let valid_syntax = token_start.len() == token_end.len();

        if !valid_syntax {
            panic!("Couldn't parse word definitions, not all word starts and ends match.");
        }

        let mut words = HashMap::new();
        for ((start, _), (end, _)) in token_start.iter().zip(token_end.iter()) {
            if let Some((name, definition)) = &self.tokens[start+1..*end].split_first() {
                if let Token::Word(n) = name {
                    words.insert(n.clone(), TokenStack::new_from_tokens(definition));
                }
            }
        }

        words
    }
}

fn match_token(token: &Token, ty: Builtins) -> bool {
    if let Token::Builtin(b) = token {
        *b == ty
    } else {
        false
    }
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
            "dup" => Builtin(Duplicate),
            ":" => Builtin(WordStart),
            ";" => Builtin(WordEnd),
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
    Duplicate,
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

pub fn parse(buf: &str) -> (TokenStack, HashMap<String, TokenStack>) {
    let lexemes = lex(buf);
    let global_stack = TokenStack::tokenize(lexemes);
    let words = global_stack.resolve_words();
    (global_stack, words)
}
