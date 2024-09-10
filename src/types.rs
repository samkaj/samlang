#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    name: String,
    value: Option<Node>,
    var_type: Type,
    pos: Position,
}

impl Variable {
    pub fn new(name: String, value: Option<Node>, var_type: Type, pos: Position) -> Self {
        Variable {
            name,
            value,
            var_type,
            pos,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Primitive(Primitive),
    Struct(String),
    Interface(String),
    Pointer(Box<Type>),
}

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
    AnyExpression,
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
pub struct Node {
    pub pos: Position,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(pos: Position, children: Vec<Node>) -> Self {
        Node { pos, children }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node::new(Position { line: 0, col: 0 }, vec![])
    }
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
