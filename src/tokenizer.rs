use crate::types::{Keyword, Op, Position, Token, TokenType, Primitive};
use std::iter::{self, from_fn};

pub struct Tokenizer {}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {}
    }

    pub fn print_tokens(&self, tokens: &Vec<Token>) {
        for token in tokens {
            println!(
                "[{},{}] {:?}",
                &token.pos.line, &token.pos.col, token.token_type
            );
        }
    }

    /// Produce a sequence of tokens from a given input string.
    pub fn tokenize(&mut self, input: &str) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut iter = input.chars().peekable();
        let mut pos = Position { line: 1, col: 1 };
        let mut in_comment = false;
        while let Some(ch) = iter.next() {
            if in_comment {
                if ch == '\n' {
                    in_comment = false;
                } else {
                    pos.col += 1;
                    continue;
                }
            }

            match ch {
                // Whitespace
                ' ' | '\t' => {
                    // multiple whitespaces are treated as one
                    while let Some(&c) = iter.peek() {
                        if c == ' ' || c == '\t' {
                            pos.col += 1;
                            iter.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::new(pos.clone(), TokenType::Whitespace));
                }
                // Newline
                '\n' => {
                    pos.line += 1;
                    pos.col = 1;
                    tokens.push(Token::new(pos.clone(), TokenType::Newline));
                }
                '0' => {
                    tokens.push(Token::new(pos.clone(), TokenType::Number(0)));
                }
                // Numbers
                '1'..='9' => {
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
                        "is" => {
                            tokens.push(Token::new(pos.clone(), TokenType::Keyword(Keyword::Is)))
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
                        "int" => tokens.push(Token::new(
                            pos.clone(),
                            TokenType::Keyword(Keyword::Type(Primitive::Int)),
                        )),
                        "str" => tokens.push(Token::new(
                            pos.clone(),
                            TokenType::Keyword(Keyword::Type(Primitive::Str)),
                        )),
                        "double" => tokens.push(Token::new(
                            pos.clone(),
                            TokenType::Keyword(Keyword::Type(Primitive::Double)),
                        )),
                        "bool" => tokens.push(Token::new(
                            pos.clone(),
                            TokenType::Keyword(Keyword::Type(Primitive::Bool)),
                        )),
                        "void" => tokens.push(Token::new(
                            pos.clone(),
                            TokenType::Keyword(Keyword::Type(Primitive::Void)),
                        )),
                        "if" => {
                            tokens.push(Token::new(pos.clone(), TokenType::Keyword(Keyword::If)))
                        }
                        "else" => {
                            tokens.push(Token::new(pos.clone(), TokenType::Keyword(Keyword::Else)))
                        }
                        "struct" => tokens
                            .push(Token::new(pos.clone(), TokenType::Keyword(Keyword::Struct))),
                        "interface" => tokens.push(Token::new(
                            pos.clone(),
                            TokenType::Keyword(Keyword::Interface),
                        )),
                        "impl" => {
                            tokens.push(Token::new(pos.clone(), TokenType::Keyword(Keyword::Impl)))
                        }
                        _ => tokens.push(Token::new(pos.clone(), TokenType::Identifier(keyword))),
                    }
                }
                // Operators
                '+' => tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Add))),
                '-' => {
                    if let Some(&'>') = iter.peek() {
                        iter.next();
                        tokens.push(Token::new(pos.clone(), TokenType::RetArrow));
                    } else {
                        tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Sub)))
                    }
                }
                '*' => tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Mul))),
                '.' => tokens.push(Token::new(pos.clone(), TokenType::Dot)),
                '/' => {
                    if let Some(&'/') = iter.peek() {
                        in_comment = true;
                        iter.next();
                        tokens.push(Token::new(pos.clone(), TokenType::Comment));
                    } else {
                        tokens.push(Token::new(pos.clone(), TokenType::Operator(Op::Div)))
                    }
                }
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
                    iter.next();
                }
                '(' => tokens.push(Token::new(pos.clone(), TokenType::LeftParen)),
                ')' => tokens.push(Token::new(pos.clone(), TokenType::RightParen)),
                '[' => tokens.push(Token::new(pos.clone(), TokenType::LeftBracket)),
                ']' => tokens.push(Token::new(pos.clone(), TokenType::RightBracket)),
                '{' => tokens.push(Token::new(pos.clone(), TokenType::LeftCurly)),
                '}' => tokens.push(Token::new(pos.clone(), TokenType::RightCurly)),
                ':' => tokens.push(Token::new(pos.clone(), TokenType::Colon)),
                ';' => tokens.push(Token::new(pos.clone(), TokenType::Semicolon)),
                ',' => tokens.push(Token::new(pos.clone(), TokenType::Comma)),
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
