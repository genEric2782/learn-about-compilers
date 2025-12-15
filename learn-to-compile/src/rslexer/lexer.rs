use std::{iter::Peekable, str::Chars};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        TextSpan { start, end, literal }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }

}

#[derive(Debug, PartialEq, Serialize)]
pub enum TokenKind {
    Integer(i64),
    // Uinterger(u64),
    Plus,
    Minus,
    Equals,
    Whitespace,
    EOF,
    Bad,
}

#[derive(Debug, Serialize)]
pub struct Token 
{
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

impl <'a> Lexer<'a> {

    // Create Instance 
    pub fn new(input: &'a str) -> Self {
        Self { input: input.chars().peekable(), source: input ,idx: 0 }
    }

    pub fn scan_for_token(&mut self) -> Option<Token> {  

        let mut kind = TokenKind::Bad;
        let start_pos = self.idx.clone(); // TODO

        // End of input stream 
        if self.peek().is_none() 
        {
            self.idx += 1;
            return Some(Token::new(
                TokenKind::EOF, 
                TextSpan::new(0, 0, '\0'.to_string()) // \0 null terminator
            ));
        }

        if let Some(c) = self.peek() {
            if Self::is_valid_number(&c) {
                let number = self.tokenize_number();
                kind = TokenKind::Integer(number);
                
            } 
            else if Self::is_valid_punctuation(&c) {
                let punctuation = self.tokenize_punctuation();
                kind = punctuation
            } 
            else if Self::is_whitespace(&c) {
                self.input.next();
                kind = TokenKind::Whitespace;
            }
            else {
                // For npw every tokenize method should consume the token 
                // so for the case of a "bad" token we need to consume it somehwere
                self.input.next(); // Make sure to consume the token before proceeding
            }
        } 

        self.idx += 1;
        let literal = self.source[start_pos..self.idx].to_string(); // input.by_ref().skip(start_pos).take(self.idx - start_pos).collect();
        let span : TextSpan = TextSpan::new(start_pos, self.idx, literal);

        Some(Token::new(kind, span)) 
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn next(&mut self) -> Option<char> {
        self.input.next()
    }

    fn is_valid_number(c: &char) -> bool {
        c.is_digit( 10)
    }

    fn is_valid_punctuation(c: &char) -> bool {
        match c {
            '+' | '-' | '=' => true,
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
                    '+' => ptoken = TokenKind::Plus,
                    '-' => ptoken = TokenKind::Minus,
                    '=' => ptoken = TokenKind::Equals,
                    _ => ptoken = TokenKind::Bad,
                };
            } else {
                break;
            }
        }
        ptoken
    }

}