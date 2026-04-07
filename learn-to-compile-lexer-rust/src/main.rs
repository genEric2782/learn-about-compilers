use crate::r#extern::extern_hs::{hs_exit, hs_init, readAST};
use crate::r#extern::extern_py::evaluate_ast_with_python;
use crate::r#extern::extern_sc::ScalaAstProcessor;
use crate::ir::ir_tac::TACInstruction;
use crate::rslexer::lexer::{TokenKind}; //Token,
use crate::r#extern::extern_cs::{call_parser};
use crate::ast::ast::{FlatASTNode, rebuild_tree};
// use serde::{Deserialize, Serialize};
// use std::process::{Command, Stdio}; used when running C# as a process 
// use std::io::{self, Write}; // BufRead
use std::ffi::CString;
// use pyo3::prelude::*;
// use pyo3::types::PyModule;

mod rslexer;
mod r#extern;
mod ast;
mod ir;

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
    let serialized_ast = serde_json::to_string(&ast).unwrap();
    let serialized_ast_clone = &serialized_ast.clone();
    let c_ast_input = CString::new(serialized_ast).unwrap();


    unsafe { hs_init(std::ptr::null_mut(), std::ptr::null_mut()); };
    let c_is_valid_ast_typecheck_output = unsafe { readAST(c_ast_input.as_ptr()) };
    unsafe { hs_exit(); };
    if c_is_valid_ast_typecheck_output == 1 
    {
        // TODO Run this async? 
        println!("Yay its valid");
        match evaluate_ast_with_python(&serialized_ast_clone) {
            Ok(result) => {
                println!("The result of the evaluated AST {}", result)
            }
            Err(e) => {
                eprintln!("Error in the Python Code: {}", e)
            }
        }
    } 
    else // == 0  
    {
        panic!("Invalid Tree Gasp");
    }

    let tac_generation = ScalaAstProcessor::new().expect("Failed to initialise GrallVM isolate ");
    match tac_generation.process(&serialized_ast_clone) {
        Ok(result) => {
            // println!("Raw JSON from Scala:\n{result}");  // add this
            let tac_instructions: Vec<TACInstruction> = serde_json::from_str(&result)
                .expect("Faild To deserialize");
            // Sanity check
            for instr in &tac_instructions {
                println!("{:?}", instr);
            }
        }
        Err(e) => eprint!("Error: {e}"),
    }
    // let c_is_valid_ast_return = unsafe { };

    // serde_json::to_writer_pretty(std::io::stdout(), &ast).unwrap();
    // Debugging
    // println!();
    /*  Basic Debugging 
        // serde_json::to_writer_pretty(std::io::stdout(), &parsed_ast).unwrap();
        println!("{:?}", serde_json::to_string(&tokens).unwrap());
        serde_json::to_writer_pretty(std::io::stdout(), &tokens).unwrap();
        println!();
        let _ = stdout.flush();
    */
    
}
