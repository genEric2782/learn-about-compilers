use serde::Serialize;
use std::{iter::Peekable, str::Chars};

#[derive(Debug, Serialize)]
pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        TextSpan {
            start,
            end,
            literal,
        }
    }

    // pub fn length(&self) -> usize {
    //     self.end - self.start
    // }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum TokenKind {
    Integer(i64),
    // Uinterger(u64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
    Whitespace,
    EOF,
    Bad,
}

#[derive(Debug, Serialize)]
pub struct Token {
    pub(crate) kind: TokenKind,
    span: TextSpan,
}

// Tokens are the individual pieces of input
// things like letter, numbers speical chars, etc. i.e. 1, a, =, +, etc....
impl Token {
    // Create Instance
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

// First Step in Building a compiler is Lexing, this is the act of breaking down the given input
// "the code" into meaningfull pieces called Tokens
pub struct Lexer<'a> {
    // Chars - iter for char on a str slice
    // 'd - a lifetime to be decided during compile time
    // Peekable a special iter that allows the next item to be viewed without consuming it
    input: Peekable<Chars<'a>>,
    source: &'a str,
    idx: usize,
}

impl<'a> Lexer<'a> {
    // Create Instance
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            source: input,
            idx: 0,
        }
    }

    pub fn scan_for_token(&mut self) -> Option<Token> {
        let mut kind = TokenKind::Bad;
        let start_pos = self.idx.clone(); // TODO

        // End of input stream
        if self.peek().is_none() {
            self.idx += 1;
            return Some(Token::new(
                TokenKind::EOF,
                TextSpan::new(0, 0, '\0'.to_string()), // \0 null terminator
            ));
        }

        if let Some(c) = self.peek() {
            if Self::is_valid_number(&c) {
                let number = self.tokenize_number();
                kind = TokenKind::Integer(number);
            } else if Self::is_valid_punctuation(&c) {
                let punctuation = self.tokenize_punctuation();
                kind = punctuation
            } else if Self::is_whitespace(&c) {
                self.input.next();
                kind = TokenKind::Whitespace;
                self.idx += 1;
            } else {
                // For now every tokenize method should consume the token
                // so for the case of a "bad" token we need to consume it somehwere
                self.input.next(); // Make sure to consume the token before proceeding
            }
        }

        // self.idx += 1;
        let literal = self.source[start_pos..self.idx].to_string(); // input.by_ref().skip(start_pos).take(self.idx - start_pos).collect();
        let span: TextSpan = TextSpan::new(start_pos, self.idx, literal);

        Some(Token::new(kind, span))
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn next(&mut self) -> Option<char> {
        self.input.next()
    }

    fn is_valid_number(c: &char) -> bool {
        c.is_digit(10)
    }

    fn is_valid_punctuation(c: &char) -> bool {
        match c {
            '+' | '-' | '*' | '/' | '=' => true,
            _ => false,
        }
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    fn tokenize_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(&c) = self.peek() {
            if c.is_digit(10) {
                let n = self.next().unwrap();
                number = number * 10 + n.to_digit(10).unwrap() as i64;
                self.idx += 1;
            } else {
                break;
            }
        }
        number
    }

    fn tokenize_punctuation(&mut self) -> TokenKind {
        let mut ptoken = TokenKind::Bad;
        while let Some(&c) = self.peek() {
            if Self::is_valid_punctuation(&c) {
                let p = self.next().unwrap();
                match p {
                    '+' => {
                        ptoken = TokenKind::Plus;
                        self.idx += 1;
                    }
                    '-' => {
                        ptoken = TokenKind::Minus;
                        self.idx += 1;
                    }
                    '*' => {
                        ptoken = TokenKind::Multiply;
                        self.idx += 1;
                    }
                    '/' => {
                        ptoken = TokenKind::Divide;
                        self.idx += 1;
                    }
                    '=' => {
                        ptoken = TokenKind::Equals;
                        self.idx += 1;
                    }
                    _ => ptoken = TokenKind::Bad,
                };
            } else {
                break;
            }
        }
        ptoken
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mirrors the loop in main.rs: scan until (and including) EOF.
    fn kinds_of(input: &str) -> Vec<TokenKind> {
        let mut lexer = Lexer::new(input);
        let mut kinds = Vec::new();
        while let Some(token) = lexer.scan_for_token() {
            let is_eof = token.kind == TokenKind::EOF;
            kinds.push(token.kind);
            if is_eof {
                break;
            }
        }
        kinds
    }

    #[test]
    fn tokenizes_single_digit_number() {
        assert_eq!(kinds_of("7"), vec![TokenKind::Integer(7), TokenKind::EOF]);
    }

    #[test]
    fn tokenizes_multi_digit_number() {
        assert_eq!(kinds_of("42"), vec![TokenKind::Integer(42), TokenKind::EOF]);
    }

    #[test]
    fn tokenizes_each_punctuation_kind() {
        assert_eq!(kinds_of("+"), vec![TokenKind::Plus, TokenKind::EOF]);
        assert_eq!(kinds_of("-"), vec![TokenKind::Minus, TokenKind::EOF]);
        assert_eq!(kinds_of("*"), vec![TokenKind::Multiply, TokenKind::EOF]);
        assert_eq!(kinds_of("/"), vec![TokenKind::Divide, TokenKind::EOF]);
        assert_eq!(kinds_of("="), vec![TokenKind::Equals, TokenKind::EOF]);
    }

    #[test]
    fn tokenizes_addition_expression_with_whitespace() {
        assert_eq!(
            kinds_of("7 + 5"),
            vec![
                TokenKind::Integer(7),
                TokenKind::Whitespace,
                TokenKind::Plus,
                TokenKind::Whitespace,
                TokenKind::Integer(5),
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn tokenizes_subtraction_expression_without_whitespace() {
        assert_eq!(
            kinds_of("4-1"),
            vec![
                TokenKind::Integer(4),
                TokenKind::Minus,
                TokenKind::Integer(1),
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn tokenizes_multiplication_expression() {
        assert_eq!(
            kinds_of("2*2"),
            vec![
                TokenKind::Integer(2),
                TokenKind::Multiply,
                TokenKind::Integer(2),
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn flags_unrecognized_characters_as_bad() {
        assert_eq!(kinds_of("@"), vec![TokenKind::Bad, TokenKind::EOF]);
    }

    #[test]
    fn empty_input_returns_only_eof() {
        assert_eq!(kinds_of(""), vec![TokenKind::EOF]);
    }

    #[test]
    fn token_span_covers_the_matched_literal() {
        let mut lexer = Lexer::new("42");
        let token = lexer.scan_for_token().unwrap();
        assert_eq!(token.span.start, 0);
        assert_eq!(token.span.end, 2);
        assert_eq!(token.span.literal, "42");
    }
}
