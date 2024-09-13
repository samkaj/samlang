#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Let,
    Return,
    Fn,
    In,
    Of,
    While,
    For,
    If,
    Else,
    Is,
    Impl,
    Struct,
    Interface,
    Type(Primitive),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Not,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier(String),
    Number(i64),
    StrLiteral(String),
    Operator(Op),
    Keyword(Keyword),
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftCurly,
    RightCurly,
    Semicolon,
    Colon,
    Comma,
    Dot,
    Whitespace,
    Comment,
    Newline,
    RetArrow,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub pos: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: i64,
    pub col: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Int,
    Str,
    Double,
    Bool,
    Void,
}

impl Token {
    pub fn new(pos: Position, token_type: TokenType) -> Token {
        Token { token_type, pos }
    }
}
