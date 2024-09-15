use std::process;

use parser::Parser;

pub mod parser;
pub mod tokenizer;
pub mod types;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = std::fs::read_to_string(&filename).expect("Failed to read file");

    let mut tokenizer = tokenizer::Tokenizer::new(filename.to_string());
    let tokens = match tokenizer.tokenize(&source) {
        Ok(tokens) => tokens,
        Err(msg) => {
            println!("{}", msg);
            process::exit(1);
        }
    };

    //tokenizer.print_tokens(&tokens);

    let mut parser = Parser::new(filename.to_string(), tokens);
    if let Err(err) = parser.parse() {
        println!("{}", err);
        process::exit(1);
    }
    process::exit(0);
}
