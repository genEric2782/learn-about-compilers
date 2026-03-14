use crate::rslexer::lexer::{TokenKind}; //Token,
// use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::io::{self, Write}; // BufRead

mod rslexer;

// #[derive(Serialize)]
// struct lexedOutput {
//     lexed_tokens: Vec<Token>,
// }

fn main() {
    let input = "7 + 5 + 4 + 3"; // Test string 
    // let mut stdout = io::stdout();

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

    let mut child = Command::new("dotnet")
        .arg("run")
        .current_dir("/home/generic/compiler_project/learn-about-compilers/learn-to-compile-c-sharp")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to start C# process");

    // println!("{:?}", serde_json::to_string(&tokens).unwrap());

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");

    // serde_json::to_writer_pretty(std::io::stdout(), &tokens).unwrap();
    serde_json::to_writer_pretty(stdin, &tokens).unwrap();
    child.wait().unwrap();
    // let _ = stdout.flush();
}
