use crate::rslexer::lexer::{TokenKind}; //Token,
// use serde::{Deserialize, Serialize};
use std::io::{self, Write}; // BufRead

mod rslexer;

// #[derive(Serialize)]
// struct lexedOutput {
//     lexed_tokens: Vec<Token>,
// }

fn main() {
    let input = "7 + 8 + 5"; // Test string 
    let mut stdout = io::stdout();

    let mut lexer = rslexer::lexer::Lexer::new(input);
    let mut tokens = Vec::new();
    let mut idx = 0;

    while let Some(token) = lexer.scan_for_token() {
        tokens.push(token);
        if tokens[idx].kind == TokenKind::EOF {
            break;
        }
        idx += 1;
    }

    // println!("{:?}", serde_json::to_string(&tokens).unwrap());

    serde_json::to_writer_pretty(std::io::stdout(), &tokens).unwrap();
    let _ = stdout.flush();
}
