mod ast;
mod interpreter;
mod lexer;
mod parser;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: d1e_lang <source_file.d1e>");
        process::exit(1);
    }

    let filename = &args[1];
    let source_code = match fs::read_to_string(filename) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source_code);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer Error: {}", e);
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();
    if let Err(e) = interpreter.eval(program) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
