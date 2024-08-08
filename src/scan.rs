use std::iter::Peekable;

/* Tests for lexer/scanner */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_expression() {
        assert_eq!(
            scan("15.2 + 6 - (2 * 7)"),
            [
                Type::Integer(15.2),
                Type::Plus,
                Type::Integer(6.0),
                Type::Dash,
                Type::LParen,
                Type::Integer(2.0),
                Type::Star,
                Type::Integer(7.0),
                Type::RParen,
            ]
        )
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

/* Processes characters sequentially to form and yield a string or numeric sequence
 * based on a specified criterion. */
pub fn consume(
    rune: char,
    iter: &mut Peekable<impl Iterator<Item = char>>,
    mut base: impl FnMut(char) -> bool,
) -> String {
    let mut result = rune.to_string();

    while let Some(&next_rune) = iter.peek() {
        if base(next_rune) {
            result.push(next_rune);
            iter.next();
        } else {
            break;
        }
    }

    result
}

/* Function scans and tokenizes input string into separate tokens, returning a
 * vector of those tokens as strings, excluding empty tokens. */
pub fn scan(stream: &str) -> Vec<Type> {
    let mut tokens: Vec<Type> = vec![];
    let mut iter = stream.chars().peekable();

    while let Some(rune) = iter.next() {
        match rune {
            rune if rune.is_ascii_whitespace() => continue,
            '0'..='9' => tokens.push(Type::Integer(
                consume(rune, &mut iter, |ch| ch.is_ascii_digit() || ch == '.')
                    .parse()
                    .expect("failer to consume integer value"),
            )),
            '(' => tokens.push(Type::LParen),
            ')' => tokens.push(Type::RParen),

            '+' => tokens.push(Type::Plus),
            '-' => tokens.push(Type::Dash),
            '*' => tokens.push(Type::Star),
            '/' => tokens.push(Type::Slash),
            _ => panic!("Unexpected syntax!"),
        }
    }

    tokens
}
