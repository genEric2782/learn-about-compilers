use crate::rslexer::lexer::TokenKind;

mod rslexer;

fn main() {
    let input = "7 + 8";

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

    println!("{:?}", serde_json::to_string(&tokens).unwrap());
}