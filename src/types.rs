use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct AST {
    pub definitions: Vec<Definition>,
}

impl AST {
    pub fn new() -> Self {
        AST {
            definitions: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    DeclStmt,
    ForStmt,
    ExprStmt,
    AssignStmt,
    IfStmt,
    ElseStmt,
    Block(Box<Vec<Statement>>),
    RetStmt(Expression),
    EmptyStmt,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Definition {
    FnDef(String, Vec<(String, Type)>, Option<Type>, Statement),
    GlobalDef,
    StructDef(Struct),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int(Option<i64>),
    Double(Option<f64>),
    String(Option<String>),
    Bool(Option<bool>),
    Void,
    Struct(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    name: String,
    fields: HashMap<String, Box<Type>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    VarDecl(String),
    VarDeclInit(String),
    VarAccess(String),
    AnonFnDef(HashMap<String, Type>),
    FnCall(String),
    VarAssign(Box<Expression>, Box<Expression>),
    BinaryOp(Op, Box<Expression>, Box<Expression>),
    UnaryOp(Box<Expression>),
    Literal(Type),
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
    Any,
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
    Pipe,
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
