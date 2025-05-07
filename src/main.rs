mod lexer;
mod parser;
mod compiler;

use std::env;
use std::fs;
use std::io::{self, Write};

// use lexer::Lexer;
use fddl::lexer::Lexer;
use fddl::parser::Parser;

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
    println!("fddl repl");
    loop {
        print!("> ");
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

// runs source code
fn run(source: String) {
    println!("Source: {}", source.trim()); // prints source for debugging

    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    // old code begins
    // println!("Tokens:");
    // for token in &tokens { // Iterate by reference if you use tokens later
    //     println!("{:?}", token);
    // }
    // println!("---");
    // old code ends - delete if not needed

    let mut parser = Parser::new(tokens); // Create a new parser instance
    let ast_statements = parser.parse_program(); // NEW!

    println!("Parsed Statements (AST):");

    for stmt in ast_statements {
        println!("{:?}", stmt);
    }
    
    loop {

        if parser.is_at_end() { // Add is_at_end to Parser if not already public
            break;
        }

        match parser.parse_statement() { 
            Some(statement) => {
                println!("{:?}", statement); 
            }
            None => {
                println!("Parser returned None, might be an error or unhandled EOF by parse_statement.");
                break;
            }
        }

    }
}