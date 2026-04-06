use crate::rslexer::lexer::{TokenKind}; //Token,
use crate::r#extern::extern_cs::{call_parser};
use crate::ast::ast::{FlatASTNode, rebuild_tree};
// use serde::{Deserialize, Serialize};
// use std::process::{Command, Stdio}; used when running C# as a process 
// use std::io::{self, Write}; // BufRead
use std::ffi::CString;

mod rslexer;
mod r#extern;
mod ast;

// #[derive(Serialize)]
// struct lexedOutput {
//     lexed_tokens: Vec<Token>,
// }

fn main() {
    let input = "7 + 5"; // Test string 
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
 
 /*  Simple way to call C# code from Rust 
    let mut child = Command::new("dotnet")
        .arg("run")
        .current_dir("/home/generic/compiler_project/learn-about-compilers/learn-to-compile-parser-c#")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to start C# process");

   
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        serde_json::to_writer_pretty(stdin, &tokens).unwrap();
    child.wait().unwrap();
*/
    
    let lexed_json = serde_json::to_string(&tokens).expect("Serialization failed");

    let c_string = CString::new(lexed_json).expect("CString::new failed");
    let parsed_ast = call_parser(&c_string);
    let flat_ast: Vec<FlatASTNode> = serde_json::from_str(&parsed_ast).expect("Failed to deserialize JSON into flattened tree");
    

    let ast = rebuild_tree(&flat_ast, 0); 
    serde_json::to_writer_pretty(std::io::stdout(), &ast).unwrap();
    // Debugging
    println!();
    /*  Basic Debugging 
        // serde_json::to_writer_pretty(std::io::stdout(), &parsed_ast).unwrap();
        println!("{:?}", serde_json::to_string(&tokens).unwrap());
        serde_json::to_writer_pretty(std::io::stdout(), &tokens).unwrap();
        println!();
        let _ = stdout.flush();
    */
    
}
