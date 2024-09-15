use crate::types::{Definition, Keyword, Primitive, Statement, Token, TokenType, Type, AST};

#[derive(Debug, Clone, PartialEq)]
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

    /// Build the syntax tree and report potential errors.
    pub fn parse(&mut self) -> Result<AST, String> {
        let mut ast = AST::new();
        loop {
            self.next_non_whitespace_token();
            match self.parse_definition() {
                Ok(def) => ast.definitions.push(def),
                Err(err) => {
                    if self.end_of_tokens() {
                        break;
                    }
                    return Err(err);
                }
            }

        }

        Ok(ast)
    }

    /// Parse a definition. It can be a function definition, global variable or a struct.
    /// Any unexpected tokens result in an error.
    fn parse_definition(&mut self) -> Result<Definition, String> {
        if self.match_token(TokenType::Keyword(Keyword::Fn)).is_ok() {
            self.parse_fn_def()
        } else if self.match_token(TokenType::Keyword(Keyword::Let)).is_ok() {
            self.parse_global()
        } else if self
            .match_token(TokenType::Keyword(Keyword::Struct))
            .is_ok()
        {
            self.parse_struct_def()
        } else {
            return Err(self.error("expected a top level definition".to_string()));
        }
    }

    /// Parse a function.
    /// # Example:
    /// ```sk
    /// fn greet(name: string) -> void {
    ///     // [statement]
    /// }
    /// ```
    fn parse_fn_def(&mut self) -> Result<Definition, String> {
        self.match_token(TokenType::Keyword(Keyword::Fn))?; // fail here

        self.next_non_whitespace_token();
        self.match_token(TokenType::Identifier("".to_string()))?; // fail here
                                                                  //
        let token = self.current_token()?;
        let function_name = match token.token_type.clone() {
            TokenType::Identifier(name) => name,
            _ => {
                return Err(self.error(format!(
                    "expected identifier but got {:?}",
                    token.token_type
                )))
            }
        };

        self.next_non_whitespace_token();
        self.match_token(TokenType::LeftParen)?;
        let mut params: Vec<(String, Type)> = vec![];
        loop {
            self.next_non_whitespace_token();
            if self.match_token(TokenType::RightParen).is_ok() {
                // no params
                break;
            }

            let token = self.current_token()?;
            let param_name = match token.token_type {
                TokenType::Identifier(name) => name,
                _ => {
                    return Err(self.error(format!(
                        "expected parameter identifier but got {:?}",
                        token.token_type
                    )))
                }
            };

            self.next_non_whitespace_token();
            self.match_token(TokenType::Colon)?;

            self.next_non_whitespace_token();
            let token = self.current_token()?;
            let param_type = match token.token_type {
                TokenType::Keyword(t) => match t {
                    Keyword::Type(t) => match t {
                        Primitive::Int => Type::Int(None),
                        Primitive::Str => Type::String(None),
                        Primitive::Bool => Type::Bool(None),
                        Primitive::Double => Type::Double(None),
                        _ => return Err(self.error("expected a non-void type".to_string())),
                    },
                    // FIXME: tokenizer won't know if this is a struct, match on identifier instead
                    Keyword::Struct => {
                        self.next_non_whitespace_token();

                        self.match_token(TokenType::Identifier(String::default()))?;
                        let token = self.current_token()?;
                        let name = match token.clone().token_type {
                            TokenType::Identifier(s) => s,
                            _ => return Err(self.error("expected a struct identifier".to_string())),
                        };

                        Type::Struct(name)
                    }
                    t => return Err(self.error(format!("expected a type, got {:?}", t))),
                },
                t => return Err(self.error(format!("expected a type, got {:?}", t))),
            };

            params.push((param_name, param_type));

            self.next_non_whitespace_token();

            if self.match_token(TokenType::Comma).is_err() {
                break;
            }
        }

        self.match_token(TokenType::RightParen)?;
        self.next_non_whitespace_token();

        let mut return_type: Option<Type> = None;
        if self.match_token(TokenType::RetArrow).is_ok() {
            self.next_non_whitespace_token();
            self.match_token(TokenType::Keyword(Keyword::Any))?;
            let t = match self.current_token()?.token_type {
                TokenType::Keyword(t) => {
                    if self.keyword_is_type(t.clone()) {
                        match t {
                            Keyword::Type(t) => match t {
                                Primitive::Int => Type::Int(None),
                                Primitive::Str => Type::String(None),
                                Primitive::Bool => Type::Bool(None),
                                Primitive::Double => Type::Double(None),
                                _ => return Err("expected a non-void type".to_string()),
                            },
                            Keyword::Struct => {
                                self.next_non_whitespace_token();

                                self.match_token(TokenType::Identifier(String::default()))?;
                                let token = self.current_token()?;
                                let name = match token.clone().token_type {
                                    TokenType::Identifier(s) => s,
                                    _ => return Err("expected a struct identifier".to_string()),
                                };

                                Type::Struct(name)
                            }
                            _ => return Err(self.error("expected a type".to_string())),
                        }
                    } else {
                        return Err(self.error("expected a type".to_string()));
                    }
                }
                _ => return Err(self.error("expected a type".to_string())),
            };

            self.next_non_whitespace_token();
            return_type = Some(t);
        }
        let block = self.parse_block()?;

        Ok(Definition::FnDef(function_name, params, return_type, block))
    }

    fn parse_global(&mut self) -> Result<Definition, String> {
        todo!("global function definition")
    }

    fn parse_struct_def(&mut self) -> Result<Definition, String> {
        todo!("struct definition")
    }

    fn parse_block(&mut self) -> Result<Statement, String> {
        self.match_token(TokenType::LeftCurly)?;
        self.next_non_whitespace_token();
        self.match_token(TokenType::RightCurly)?;
        Ok(Statement::EmptyStmt)
    }

    fn end_of_tokens(&mut self) -> bool {
        self.token_index >= self.tokens.len()
    }

    fn current_token(&mut self) -> Result<Token, String> {
        if !self.end_of_tokens() {
            Ok(self.tokens[self.token_index].clone())
        } else {
            Err("Unexpected end of file".to_string())
        }
    }

    fn next_token(&mut self) {
        if !self.end_of_tokens() {
            self.token_index += 1;
        }
    }

    fn next_non_whitespace_token(&mut self) {
        self.next_token();
        let whitespaces = vec![
            TokenType::Whitespace,
            TokenType::Newline,
            TokenType::Comment,
        ];
        while whitespaces.iter().any(|t| {
            if self.end_of_tokens() {
                return false;
            }
            *t == self.current_token().unwrap().token_type
        }) {
            self.next_token();
        }
    }

    fn match_token(&mut self, token_type: TokenType) -> Result<(), String> {
        if let Ok(curr) = self.current_token() {
            return match (&curr.token_type, token_type.clone()) {
                (TokenType::Identifier(_), TokenType::Identifier(_))
                | (TokenType::Number(_), TokenType::Number(_))
                | (TokenType::StrLiteral(_), TokenType::StrLiteral(_))
                | (TokenType::Keyword(_), TokenType::Keyword(_)) => Ok(()),
                (curr_type, expected_type) if *curr_type == expected_type => Ok(()),
                _ => {
                    let msg = format!(
                        "expected `{:?}`, but got `{:?}`",
                        token_type, &curr.token_type
                    );
                    Err(self.error(msg))
                }
            };
        }

        Ok(())
    }

    fn keyword_is_type(&self, keyword: Keyword) -> bool {
        return match keyword {
            Keyword::Struct | Keyword::Type(_) => true,
            _ => false,
        };
    }

    fn error(&mut self, msg: String) -> String {
        if let Ok(token) = self.current_token() {
            format!(
                "Syntax error: {} at {}:{:?}:{:?}",
                msg, self.file, &token.pos.line, token.pos.col
            )
        } else {
            format!("Syntax error: {}", msg)
        }
    }
}

#[cfg(test)]
mod test {}
