/* © Copyright Topaz Foundation, 2024. All rights reserved © */

/* A scanner in a compiler is the initial phase that transforms raw source code
 * into a series of tokens, the units of a programming language's syntax. */

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

    /* Processes characters sequentially to form and yield a string or numeric sequence
     * based on a specified criterion. */
    pub fn consume(&mut self, rune: char, base: impl Fn(char) -> bool) -> String {
        let mut result = rune.to_string();

        while let Some(&next_rune) = self.iter.peek() {
            if base(next_rune) {
                result.push(next_rune);
                self.iter.next();
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
    /* Data Types */
    Integer(f64),

    /* Operators */
    LParen,
    RParen,
    Plus,
    Dash,
    Star,
    Slash,
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
            '(' => scanner.tokens.push(Type::LParen),
            ')' => scanner.tokens.push(Type::RParen),

            '+' => scanner.tokens.push(Type::Plus),
            '-' => scanner.tokens.push(Type::Dash),
            '*' => scanner.tokens.push(Type::Star),
            '/' => scanner.tokens.push(Type::Slash),
            '"' => continue,
            _ => panic!("Unexpected syntax!"),
        }
    }

    scanner.tokens
}
