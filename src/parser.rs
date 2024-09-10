use crate::types::{
    Keyword::{self, Type},
    Node,
    Op::*,
    Position, Primitive, Token, TokenType,
};

pub struct Parser {
    file: String,
    tokens: Vec<Token>,
    token_index: usize,
}

impl Parser {
    pub fn new(file: String, tokens: Vec<Token>) -> Parser {
        Parser {
            file,
            tokens,
            token_index: 0,
        }
    }

    // TODO:
    // Parse the expression `let x: int = 0;`
    // Let Ident Colon Type(int) Eq Number(0)

    pub fn parse(&mut self) -> Result<Node, String> {
        self.parse_top_level_declaration()
    }

    /// Parse a top level declaration
    ///
    /// # Examples
    /// ```txt
    /// - let x: int = 0; // GlobalVariableDeclaration
    /// - fn foo() -> int { ... } // FunctionDefinition
    /// - ...
    /// ```
    fn parse_top_level_declaration(&mut self) -> Result<Node, String> {
        if self.match_tokens(vec![
            TokenType::Whitespace,
            TokenType::Comment,
            TokenType::Newline,
        ]) {
            self.next_token();
            self.parse_top_level_declaration()
        } else if self.match_token(TokenType::Keyword(Keyword::Let)) {
            self.parse_variable_declaration()
        } else {
            let token = self.current_token().unwrap();
            let pos = &token.pos;
            Err(format!(
                "unexpected token at {}{}:{}: {:?}, expected a top level declaration",
                self.file, &pos.col, pos.line, token.token_type
            ))
        }
    }

    /// Parse a variable declaration
    /// ```txt
    /// let x: int = 0;
    /// let y;
    /// ```
    fn parse_variable_declaration(&mut self) -> Result<Node, String> {
        self.next_non_whitespace_token(); // We know we have a `let`

        if !self.match_token(TokenType::Identifier(String::new())) {
            let token = self.current_token().unwrap();
            let pos = &token.pos;
            return Err(format!(
                "Expected identifier at {}{}:{}: {:?}",
                self.file, pos.col, pos.line, token.token_type
            ));
        }
        self.next_non_whitespace_token();

        if !self.match_token(TokenType::Colon) {
            let token = self.current_token().unwrap();
            let pos = &token.pos;
            return Err(format!(
                "Expected ':' at {}:{}:{}: {:?}",
                self.file, pos.col, pos.line, token.token_type
            ));
        }
        self.next_non_whitespace_token();

        if !self.match_token(TokenType::Keyword(Keyword::Type(Primitive::Int))) {
            let token = self.current_token().unwrap();
            let pos = &token.pos;
            return Err(format!(
                "Expected type 'int' at {}:{}:{}: {:?}",
                self.file, pos.col, pos.line, token.token_type
            ));
        }
        self.next_non_whitespace_token();

        let value: Node;
        if self.match_token(TokenType::Operator(Eq)) {
            self.next_non_whitespace_token();

            match self.parse_expression() {
                Ok(node) => value = node,
                Err(e) => return Err(e),
            }
        } else {
            let token = self.current_token().unwrap();
            let pos = &token.pos;
            return Err(format!(
                "Expected '=' at {}:{}:{}: {:?}",
                self.file, pos.col, pos.line, token.token_type
            ));
        }

        self.next_non_whitespace_token();
        if !self.match_token(TokenType::Semicolon) {
            let token = self.current_token().unwrap();
            let pos = &token.pos;
            return Err(format!(
                "Expected ';' at {}:{}:{}: {:?}",
                self.file, pos.col, pos.line, token.token_type
            ));
        }

        Ok(value)
    }

    // FIXME: the node type has to change, think about how the AST should look
    fn parse_expression(&mut self) -> Result<Node, String> {
        dbg!("Parse expression");
        let curr = self.current_token().unwrap();
        return match curr.token_type.clone() {
            TokenType::Number(no) => {
                Ok(Node::new(curr.pos, vec![]))
            }
            _ => Err("".to_string()),
        };
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

    fn next_non_whitespace_token(&mut self) {
        while self.current_token().unwrap().token_type == TokenType::Whitespace {
            self.next_token();
        }
    }

    // Potential FIXME: token types do not compare enums with contained values correctly
    fn match_token(&mut self, token_type: TokenType) -> bool {
        if let Some(curr) = self.current_token() {
            return match (&curr.token_type, token_type) {
                (TokenType::Identifier(_), TokenType::Identifier(_)) => {
                    self.next_token();
                    true
                }
                (TokenType::Number(_), TokenType::Number(_)) => {
                    self.next_token();
                    true
                }
                (curr_type, expected_type) if *curr_type == expected_type => {
                    self.next_token();
                    true
                }
                _ => false,
            };
        }

        false
    }

    fn match_tokens(&mut self, token_types: Vec<TokenType>) -> bool {
        if let Some(curr) = self.current_token() {
            if token_types.iter().any(|t| *t == curr.token_type) {
                self.next_token();
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::{Position, Token, TokenType};

    #[test]
    fn match_token_types() {
        let pos = Position { line: 0, col: 0 };
        let str1 = Token {
            token_type: TokenType::StrLiteral("hello".to_string()),
            pos: pos.clone(),
        };

        let mut parser = Parser::new("".to_string(), vec![str1.clone()]);
        assert!(parser.match_token(TokenType::StrLiteral("hello".to_string())));
    }
}
