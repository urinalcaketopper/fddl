mod lexer;
mod parser;
mod compiler;

use std::env;
use std::fs;
use std::io::{self, Write};

use fddl::lexer::Lexer;
use fddl::parser::Parser;
use fddl::interpreter::evaluator::Evaluator;

fn main() {
    let args: Vec<String> = env::args().collect();

    // runs file or REPL
    if args.len() > 1 {
        run_file(&args[1]);
    } else {
        run_repl();
    }
}

// basic REPL
fn run_repl() {
    println!("FDDL REPL");
    println!("---------");
    loop {
        print!("fddl % ");
        io::stdout().flush().unwrap();

        let mut buffer =     String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        if buffer.trim().is_empty() {
            continue;
        }

        run(buffer.clone());
    }
}

// runs file
fn run_file(path: &str) {
    let source = fs::read_to_string(path).expect("Failed to read source file");
    run(source);
}

fn run(source: String) {
    println!("Source: {}", source.trim());

    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    println!("Tokens: {:?}", tokens);

    let mut parser = Parser::new(tokens);
    
    let program_ast = parser.parse_program(); 

    if !program_ast.is_empty() { 
        println!("Parsed Statements (AST):");
        for stmt in &program_ast {
            println!("{:?}", stmt);
        }
    } else {
        println!("No AST generated or parsing failed.");
    }


    if !program_ast.is_empty() {
        println!("Output:"); 
        let mut evaluator = Evaluator::new();
        match evaluator.evaluate_program(program_ast) {
            Ok(()) => { /* Program executed successfully */ }
            Err(e) => {
                eprintln!("Runtime Error: {:?}", e);
            }
        }
    } else {
        println!("Skipping execution.");
    }

    println!("---"); 
}