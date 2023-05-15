use std::fmt::{Debug, Formatter};
use crate::token::{Token, TokenType};

pub trait Statement: Debug {
    fn token_type(&self) -> TokenType;
}

pub trait Expression: Debug {
    fn token_type(&self) -> TokenType;
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: vec![]
        }
    }
}

impl Debug for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Program [")?;
        for stmt in &self.statements {
            writeln!(f, "{:?},", stmt)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl Statement for Program {
    fn token_type(&self) -> TokenType {
        self.statements[0].token_type()
    }
}

// tiny语言中块语句的结束标志为TokenType::End或TokenType::Until
pub struct BlockStatement {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Debug for BlockStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "BlockStatement [")?;
        for stmt in &self.statements {
            writeln!(f, "{:?},", stmt)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl Statement for BlockStatement {
    fn token_type(&self) -> TokenType {
        self.statements[0].token_type()
    }
}

#[derive(Debug)]
pub struct AssignStatement {
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Statement for AssignStatement {
    fn token_type(&self) -> TokenType {
        TokenType::Assign
    }
}

#[derive(Debug)]
pub struct ReadStatement {
    pub name: Identifier,
}

impl Statement for ReadStatement {
    fn token_type(&self) -> TokenType {
        TokenType::Read
    }
}

#[derive(Debug)]
pub struct WriteStatement {
    pub name: Identifier,
}

impl Statement for WriteStatement {
    fn token_type(&self) -> TokenType {
        TokenType::Write
    }
}

pub struct IfStatement {
    pub cond: Box<dyn Expression>,
    pub consequence: BlockStatement,
}

impl Debug for IfStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "IfStatement {{")?;
        writeln!(f, "cond: {:?}", self.cond)?;
        write!(f, "consequence:\n{:?}}}", self.consequence)?;
        Ok(())
    }
}

impl Statement for IfStatement {
    fn token_type(&self) -> TokenType {
        TokenType::If
    }
}

pub struct RepeatStatement {
    pub cond: Box<dyn Expression>,
    pub consequence: BlockStatement,
}

impl Debug for RepeatStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "RepeatStatement {{")?;
        writeln!(f, "cond: {:?}", self.cond)?;
        write!(f, "consequence:\n{:?}}}", self.consequence)?;
        Ok(())
    }
}

impl Statement for RepeatStatement {
    fn token_type(&self) -> TokenType {
        TokenType::Repeat
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    pub op: Token,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for InfixExpression {
    fn token_type(&self) -> TokenType {
        self.op.token_type
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Identifier {
    pub value: String,
}


impl Expression for Identifier {
    fn token_type(&self) -> TokenType {
        TokenType::Ident
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Number {
    pub value: i32,
}

impl Expression for Number {
    fn token_type(&self) -> TokenType {
        TokenType::Number
    }
}
