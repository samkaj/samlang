// https://petermalmgren.com/three-rust-parsers/

use crate::common::{Keyword, Node, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    token_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            token_index: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Node, String> {
        self.parse_program()
    }

    /// A program is the top-level structure of a source file.
    /// It can contain statements, functions, and other declarations.
    fn parse_program(&mut self) -> Result<Node, String> {
        let mut children: Vec<Node> = vec![];
        while !self.end_of_tokens() {
            match self.parse_declaration() {
                Ok(n) => children.push(n),
                Err(e) => return Err(e),
            }
        }

        Ok(Node {
            token: Token::new(
                self.tokens[self.token_index].pos.clone(),
                TokenType::Program,
            ),
            children,
        })
    }

    /// A declaration is a statement that defines a variable or a function.
    fn parse_declaration(&mut self) -> Result<Node, String> {
        todo!();
    }

    /// A statement is a sequence of tokens that ends with a semicolon or is a block of statements.
    fn parse_statement(&mut self) -> Result<Node, String> {
        todo!();
    }

    /// A block is a sequence of statements enclosed in curly braces.
    fn parse_block(&mut self) -> Result<Node, String> {
        todo!();
    }

    /// An expression is a sequence of tokens that evaluates to a value.
    fn parse_expression(&mut self) -> Result<Node, String> {
        todo!();
    }

    fn end_of_tokens(&mut self) -> bool {
        self.token_index >= self.tokens.len()
    }

    fn current_token(&mut self) -> Option<Token> {
        if !self.end_of_tokens() {
            Some(self.tokens[self.token_index].clone())
        } else {
            None
        }
    }

    fn next_token(&mut self) {
        if !self.end_of_tokens() {
            self.token_index += 1;
        }
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if let Some(curr) = self.current_token() {
            if curr.token_type == token_type {
                self.next_token();
                return true;
            }

            return false;
        }

        false
    }
}
