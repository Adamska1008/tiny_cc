use crate::token::{Token, TokenType};

trait Statement {
    fn token_type(&self) -> TokenType;
}

trait Expression {
    fn token_type(&self) -> TokenType;
}

struct Program {
    pub statements: Vec<Box<dyn Statement>>
}

impl Statement for Program {
    fn token_type(&self) -> TokenType {
        self.statements[0].token_type()
    }
}

struct BlockStatement {
    pub statements: Vec<Box<dyn Statement>>,
}

struct AssignStatement {
    pub name: Box<Identifier>,
    pub value: Box<dyn Expression>,
}

impl Statement for AssignStatement {
    fn token_type(&self) -> TokenType {
        TokenType::Assign
    }
}

struct ReadStatement {
    pub name: Box<Identifier>,
}

impl Statement for ReadStatement {
    fn token_type(&self) -> TokenType {
        TokenType::Read
    }
}

struct WriteStatement {
    pub name: Box<Identifier>,
}

impl Statement for WriteStatement {
    fn token_type(&self) -> TokenType {
        TokenType::Write
    }
}

struct IfStatement {
    pub cond: Box<dyn Expression>,
    pub consequence: Box<BlockStatement>,
}

impl Statement for IfStatement {
    fn token_type(&self) -> TokenType {
        TokenType::If
    }
}

struct RepeatStatement {
    pub cond: Box<dyn Expression>,
    pub consequence: Box<BlockStatement>,
}

impl Statement for RepeatStatement {
    fn token_type(&self) -> TokenType {
        TokenType::Repeat
    }
}

struct InfixExpression {
    pub op: Token,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for InfixExpression {
    fn token_type(&self) -> TokenType {
        self.op.token_type
    }
}

struct Identifier {
    pub value: String,
}

impl Expression for Identifier {
    fn token_type(&self) -> TokenType {
        TokenType::Ident
    }
}

struct Number {
    pub value: i32,
}

impl Expression for Number {
    fn token_type(&self) -> TokenType {
        TokenType::Number
    }
}
