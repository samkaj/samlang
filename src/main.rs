pub mod common;
pub mod tokenizer;

fn main() {
    // read file from command line
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = std::fs::read_to_string(filename).expect("Failed to read file");

    let mut tokenizer = tokenizer::Tokenizer::new();
    let tokens = tokenizer.tokenize(&source).unwrap();
    tokenizer.print_tokens(&tokens);
}
