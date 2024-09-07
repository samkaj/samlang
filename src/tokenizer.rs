use crate::common::{Keyword, Op, Position, Token, TokenType};
use std::iter::{self, from_fn};

pub struct Tokenizer {}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {}
    }

    pub fn print_tokens(&self, tokens: &Vec<Token>) {
        for token in tokens {
            println!("{:?}", token.token_type);
        }
    }

    /// Produce a sequence of tokens from a given input string.
    pub fn tokenize(&mut self, input: &str) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut iter = input.chars().peekable();
        let mut pos = Position { line: 0, col: 0 };
        while let Some(ch) = iter.next() {
            match ch {
                // Whitespace
                ' ' | '\t' => {
                    tokens.push(Token::new(pos.clone(), TokenType::Whitespace));
                }
                // Newline
                '\n' => {
                    pos.line += 1;
                    pos.col = 0;
                    tokens.push(Token::new(pos.clone(), TokenType::Newline));
                }
                // Numbers
                '1'..'9' => {
                    let n: i64 = iter::once(ch)
                        .chain(from_fn(|| {
                            iter.by_ref().next_if(|s| {
                                pos.col += 1;
                                s.is_ascii_digit()
                            })
                        }))
                        .collect::<String>()
                        .parse()
                        .expect("a digit");
                    tokens.push(Token::new(pos.clone(), TokenType::Number(n)));
                }
                // Keywords
                'a'..='z' | 'A'..='Z' => {
                    let keyword: String = iter::once(ch)
                        .chain(from_fn(|| {
                            iter.by_ref().next_if(|s| {
                                pos.col += 1;
                                s.is_ascii_alphabetic() || s == &'_' || s.is_ascii_digit()
                            })
                        }))
                        .collect();
                    match keyword.as_str() {
                        "let" => {
                            tokens.push(Token::new(pos.clone(), TokenType::Keyword(Keyword::Let)))
                        }
                        "return" => tokens
                            .push(Token::new(pos.clone(), TokenType::Keyword(Keyword::Return))),
                        "fn" => {
                            tokens.push(Token::new(pos.clone(), TokenType::Keyword(Keyword::Fn)))
                        }
                        "in" => {
                            tokens.push(Token::new(pos.clone(), TokenType::Keyword(Keyword::In)))
                        }
                        "of" => {
                            tokens.push(Token::new(pos.clone(), TokenType::Keyword(Keyword::Of)))
                        }
                        "while" => {
                            tokens.push(Token::new(pos.clone(), TokenType::Keyword(Keyword::While)))
                        }
                        "for" => {
                            tokens.push(Token::new(pos.clone(), TokenType::Keyword(Keyword::For)))
                        }
                        "int" => {
                            tokens.push(Token::new(pos.clone(), TokenType::Keyword(Keyword::Int)))
                        }
                        "str" => {
                            tokens.push(Token::new(pos.clone(), TokenType::Keyword(Keyword::Str)))
                        }
                        "double" => tokens
                            .push(Token::new(pos.clone(), TokenType::Keyword(Keyword::Double))),
                        "bool" => {
                            tokens.push(Token::new(pos.clone(), TokenType::Keyword(Keyword::Bool)))
                        }
                        "void" => {
                            tokens.push(Token::new(pos.clone(), TokenType::Keyword(Keyword::Void)))
                        }
                        "if" => {
                            tokens.push(Token::new(pos.clone(), TokenType::Keyword(Keyword::If)))
                        }
                        "else" => {
                            tokens.push(Token::new(pos.clone(), TokenType::Keyword(Keyword::Else)))
                        }
                        _ => tokens.push(Token::new(pos.clone(), TokenType::Identifier(keyword))),
                    }
                }
                // Operators
                '+' => tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Add))),
                '-' => tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Sub))),
                '*' => tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Mul))),
                '/' => tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Div))),
                '%' => tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Mod))),
                '&' => tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::And))),
                '|' => tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Or))),
                '=' => tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Eq))),
                '!' => {
                    if let Some(&'=') = iter.peek() {
                        iter.next();
                        tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Neq)));
                    } else {
                        tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Not)));
                    }
                }
                '<' => {
                    if let Some(&'=') = iter.peek() {
                        iter.next();
                        tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Lte)));
                    } else {
                        tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Lt)));
                    }
                }
                '>' => {
                    if let Some(&'=') = iter.peek() {
                        iter.next();
                        tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Gte)));
                    } else {
                        tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Gt)));
                    }
                }
                // String literals
                '"' => {
                    let string: String = from_fn(|| {
                        iter.by_ref().next_if(|s| {
                            pos.col += 1;
                            s != &'\"'
                        })
                    })
                    .collect();
                    tokens.push(Token::new(pos.clone(), TokenType::StrLiteral(string)));
                }
                '(' => tokens.push(Token::new(pos.clone(), TokenType::LeftParen)),
                ')' => tokens.push(Token::new(pos.clone(), TokenType::RightParen)),
                '[' => tokens.push(Token::new(pos.clone(), TokenType::LeftBracket)),
                ']' => tokens.push(Token::new(pos.clone(), TokenType::RightBracker)),
                '{' => tokens.push(Token::new(pos.clone(), TokenType::LeftCurly)),
                '}' => tokens.push(Token::new(pos.clone(), TokenType::RightCurly)),
                ';' => tokens.push(Token::new(pos.clone(), TokenType::Semicolon)),
                _ => {
                    return Err(format!(
                        "unexpected character at {}:{}: {}",
                        &pos.line, pos.col, ch
                    ));
                }
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_with_numbers() {
        let mut tokenizer = Tokenizer::new();
        let input = "int123";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(
            tokens[0].token_type,
            TokenType::Identifier("int123".to_string())
        );

        let input = "foo_123";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(
            tokens[0].token_type,
            TokenType::Identifier("foo_123".to_string())
        );
    }

    #[test]
    fn test_split_numbers() {
        let mut tokenizer = Tokenizer::new();
        let input = "123a1";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Number(123));
        assert_eq!(
            tokens[1].token_type,
            TokenType::Identifier("a1".to_string())
        );

        let input = "123 123";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Number(123));
        assert_eq!(tokens[1].token_type, TokenType::Whitespace);
        assert_eq!(tokens[2].token_type, TokenType::Number(123));
    }

    #[test]
    fn test_identifier() {
        let mut tokenizer = Tokenizer::new();
        let input = "foo";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(
            tokens[0].token_type,
            TokenType::Identifier("foo".to_string())
        );

        let input = "bar";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(
            tokens[0].token_type,
            TokenType::Identifier("bar".to_string())
        );

        let input = "int foo";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Keyword(Keyword::Int));
        assert_eq!(tokens[1].token_type, TokenType::Whitespace);
        assert_eq!(
            tokens[2].token_type,
            TokenType::Identifier("foo".to_string())
        );
    }

    #[test]
    fn test_operator() {
        let mut tokenizer = Tokenizer::new();
        let input = "%";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Operator(Op::Mod));

        let input = ">=";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Operator(Op::Gte));

        let input = ">=<";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Operator(Op::Gte));
        assert_eq!(tokens[1].token_type, TokenType::Operator(Op::Lt));
    }

    #[test]
    fn test_keyword() {
        let mut tokenizer = Tokenizer::new();
        let input = "let";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Keyword(Keyword::Let));

        let input = "return";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Keyword(Keyword::Return));

        let input = "fn";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Keyword(Keyword::Fn));

        let input = "in";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Keyword(Keyword::In));

        let input = "of";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Keyword(Keyword::Of));

        let input = "while";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Keyword(Keyword::While));

        let input = "for";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Keyword(Keyword::For));

        let input = "int";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Keyword(Keyword::Int));

        let input = "str";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Keyword(Keyword::Str));

        let input = "double";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Keyword(Keyword::Double));

        let input = "bool";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Keyword(Keyword::Bool));

        let input = "void";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Keyword(Keyword::Void));

        let input = "if";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Keyword(Keyword::If));

        let input = "else";
        let tokens = tokenizer.tokenize(input).unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Keyword(Keyword::Else));
    }
}
