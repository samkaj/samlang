#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Let,
    Return,
    Fn,
    In,
    Of,
    While,
    For,
    Int,
    Str,
    Double,
    Bool,
    Void,
    If,
    Else,
    Struct,
    Interface,
    Is,
    Impl,
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
    RightBracker,
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

pub struct Token {
    pub token_type: TokenType,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub line: i64,
    pub col: i64,
}

impl Token {
    pub fn new(pos: Position, token_type: TokenType) -> Token {
        Token { token_type, pos }
    }
}
