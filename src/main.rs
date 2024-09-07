pub mod common;
pub mod tokenizer;

fn main() {
    let mut tokenizer = tokenizer::Tokenizer::new();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let tokens = tokenizer.tokenize(&input);
    tokenizer.print_tokens(&tokens.unwrap());
}
