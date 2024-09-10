use crate::types::{Keyword, Node, Token, TokenType};

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
        todo!("parser");
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

    // Potential FIXME: token types do not compare enums with contained values correctly
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

        let mut parser = Parser::new(vec![str1.clone()]);
        assert!(parser.match_token(TokenType::StrLiteral("hello".to_string())));
    }
}
