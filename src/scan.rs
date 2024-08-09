/* © Copyright Topaz Foundation, 2024. All rights reserved © */

/* A scanner in a compiler is the initial phase that transforms source code
 * into a series of tokens, the units of Topaz's syntax. */

/* Imports */
use std::iter::Peekable;

/* Scanner Struct */
struct Scanner<'a> {
    stream: &'a str, /* Input file to be tokenized */
    iter: Peekable<std::str::Chars<'a>>,
    tokens: Vec<Type>,
}

impl<'a> Scanner<'a> {
    /* Initializes a scanner struct */
    pub fn new(stream: &'a str) -> Self {
        let iter = stream.chars().peekable();
        Scanner {
            stream,
            iter,
            tokens: Vec::new(),
        }
    }

    /* Processes characters sequentially to form and yield a string or numeric
     * sequence based on a specified criterion. */
    pub fn consume(&mut self, rune: char, base: impl Fn(char) -> bool) -> String {
        let mut result = rune.to_string();

        while let Some(&next_rune) = self.iter.peek() {
            if base(next_rune) {
                result.push(next_rune);
                self.iter.next();

                if rune == '"' && next_rune == '"' {
                    break;
                }
            } else {
                break;
            }
        }

        result
    }
}

/* A comprehensive token enumeration representing all different possible types */
#[derive(Debug, PartialEq)] // debug for printing, partialeq for testing.
pub enum Type {
    /* Literals */
    Identifier(String),

    Integer(f64),
    Boolean(bool),
    String(String),

    /* Operators */
    LParen,
    RParen,
    Plus,
    Dash,
    Star,
    Slash,
    Equals,

    /* Keywords */
    Struct,
    Extend,
    Func,
    If,
    Else,
    While,
    For,
    Import,
    Return,

    /* Miscelleneous */
    Error,
    Eof,
}

/* Function scans and tokenizes input string into separate tokens, returning a
 * vector of those tokens as strings, excluding empty tokens. */
pub fn scan(stream: &str) -> Vec<Type> {
    let mut scanner = Scanner::new(&stream);

    while let Some(rune) = scanner.iter.next() {
        match rune {
            rune if rune.is_ascii_whitespace() => continue,
            '0'..='9' => {
                let number = scanner
                    .consume(rune, |ch| ch.is_ascii_digit() || ch == '.')
                    .parse()
                    .expect("failer to consume integer value");
                scanner.tokens.push(Type::Integer(number))
            }
            '"' => {
                let string = scanner.consume(rune, |_| true);
                scanner.tokens.push(Type::String(string))
            }

            '(' => scanner.tokens.push(Type::LParen),
            ')' => scanner.tokens.push(Type::RParen),

            '+' => scanner.tokens.push(Type::Plus),
            '-' => scanner.tokens.push(Type::Dash),
            '*' => scanner.tokens.push(Type::Star),
            '/' => scanner.tokens.push(Type::Slash),

            '=' => scanner.tokens.push(Type::Equals),

            rune if rune.is_ascii_alphabetic() || rune == '_' => {
                let token = scanner.consume(rune, |ch| ch.is_ascii_alphabetic() || ch == '_');

                match token.as_str() {
                    "struct" => scanner.tokens.push(Type::Struct),
                    "extend" => scanner.tokens.push(Type::Extend),
                    "func" => scanner.tokens.push(Type::Func),
                    "if" => scanner.tokens.push(Type::If),
                    "else" => scanner.tokens.push(Type::Else),
                    "while" => scanner.tokens.push(Type::While),
                    "for" => scanner.tokens.push(Type::For),
                    "import" => scanner.tokens.push(Type::Import),
                    "return" => scanner.tokens.push(Type::Return),
                    "true" => scanner.tokens.push(Type::Boolean(true)),
                    "false" => scanner.tokens.push(Type::Boolean(false)),
                    _ => scanner.tokens.push(Type::Identifier(token)),
                }
            }

            _ => panic!("Unexpected syntax!"),
        }
    }
    scanner.tokens.push(Type::Eof);
    scanner.tokens
}
