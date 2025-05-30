mod lexer;
mod parser;
mod compiler;

use std::env;
use std::fs;
use std::io::{self, Write};

use fddl::lexer::Lexer;
use fddl::parser::Parser;
use fddl::interpreter::evaluator::Evaluator;
use fddl::parser::ast::{Statement, Expression};

fn main() {
    let args: Vec<String> = env::args().collect();

    // runs file or REPL
    if args.len() > 1 {
        run_file(&args[1]);
    } else {
        run_repl();
    }
}

fn run_repl() {
    println!("fddl REPL");
    println!("---------");
    let mut evaluator = Evaluator::new();

    loop {
        print!("fddl % "); 
        std::io::stdout().flush().unwrap();

        let mut buffer = String::new();
        if std::io::stdin().read_line(&mut buffer).is_err() || buffer.trim() == "exit" {
            break;
        }

        if buffer.trim().is_empty() {
            continue;
        }

        run_line(buffer, &mut evaluator); 
    }
}

fn run_line(source: String, evaluator: &mut Evaluator) { 
    println!("Source: {}", source.trim());

    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    let mut parser = Parser::new(tokens);
    let program_ast: Vec<Statement> = parser.parse_program();

    if !program_ast.is_empty() {

        println!("Output:");
        match evaluator.evaluate_program(program_ast) { 
            Ok(()) => { /* Statement executed successfully */ }
            Err(e) => {
                eprintln!("Runtime Error: {:?}", e);
            }
        }
    } else {
        println!("No AST generated or parsing failed for this line.");
    }
    println!("---");
}

fn run_file(path: &str) {
    println!("Running file: {}", path);
    match std::fs::read_to_string(path) {
        Ok(source) => {
            let mut file_evaluator = Evaluator::new();
            run_line(source, &mut file_evaluator);
        }
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
        }
    }
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