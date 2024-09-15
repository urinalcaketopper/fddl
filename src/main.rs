mod lexer;
mod parser;
mod interpreter;

use std::env;
use std::fs;
use std::io::{self, Write};

use lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // If a file is provided, run it
        run_file(&args[1]);
    } else {
        // Otherwise start the REPL
        run_repl();
    }
}

fn run_repl() {
    println!("fddl REPL");
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
fn run_file(path: &str) {
    let source = fs::read_to_string(path).expect("Failed to read source file");
    run(source);
}

fn run(source: String) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    // pass tokens to parser and interpreter
}