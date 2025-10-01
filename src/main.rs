use rlox::token::{Token, TokenType};
use std::env;
use std::io;
use std::io::BufRead;
use std::process;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        n if n > 1 => {
            println!("Usage: rlox [script]");
            exit(64);
        }
        // Todo : runFile(args[1])
        2 => println!("run_file"),
        // Todo : runPrompt()
        _ => run_prompt(),
    }
}

fn run_prompt() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        if let Ok(content) = line {
            print!("> ");
            run(content);
        }
    }
}

fn fun_file(path: String) {}

fn run(source: String) {
    let mut tokens: Vec<Token>;
}

fn error(line: usize, message: String) {}

fn report(line: usize, location: String, message: String) {
    eprintln!("[line {} ] Error {} : {} ", line, location, message);
}
