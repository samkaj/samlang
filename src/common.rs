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

// TBA: think about how to represent the AST. should declarations, statements, expressions, etc. be enums? e.g.: Declaration(DeclarationType)
// This would keep the AST more organized and easier to traverse, possibly.
pub enum NodeType {
    Program,
    FunctionDeclaration,
    VariableDeclaration,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub token: Token, // FIXME: tokens are semi colons and stuff, we care about the type of the node. Therefore, we should have a NodeType enum
    pub children: Vec<Node>,
}

pub struct Program {
    pub functions: Vec<Function>,
}

pub struct Function {
    pub name: String,
    pub params: Vec<Declaration>,
    pub body: Vec<Node>,
}

pub struct Declaration {
    pub name: String,
    pub value: Node,
}

pub enum Statement {
    Declaration(Declaration),
    Expression(Node),
    Return(Node),
    If(Node, Vec<Statement>, Vec<Statement>),
    While(Node, Vec<Statement>),
    For(Node, Node, Node, Vec<Statement>),
    ForIn(Node, Node, Vec<Statement>),
}

pub enum Expression {
    BinaryOp(Op, Box<Expression>, Box<Expression>),
    UnaryOp(Op, Box<Expression>),
    Literal(Token),
    Identifier(Token),
    Call(Token, Vec<Expression>),
}

pub enum Type {
    Int,
    Str,
    Double,
    Bool,
    Void,
    Struct(String),
    Interface(String),
}

impl Token {
    pub fn new(pos: Position, token_type: TokenType) -> Token {
        Token { token_type, pos }
    }
}
