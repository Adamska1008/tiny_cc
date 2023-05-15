use crate::token::{Token, TokenType};
use std::any::Any;
use std::fmt::{Debug, Formatter};

pub enum NodeType {
    Program,

    AssignStatement,
    IfStatement,
    RepeatStatement,
    BlockStatement,
    ReadStatement,
    WriteStatement,

    InfixExpression,
    Identifier,
    Number,
}

pub trait Node {
    fn token_type(&self) -> TokenType;
    fn node_type(&self) -> NodeType;
    fn as_any(&self) -> &dyn Any;
}

pub trait Statement: Debug + Node {}

pub trait Expression: Debug + Node {}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Self {
        Self { statements: vec![] }
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

impl Node for Program {
    fn token_type(&self) -> TokenType {
        self.statements[0].token_type()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Program
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for Program {}

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

impl Node for BlockStatement {
    fn token_type(&self) -> TokenType {
        self.statements[0].token_type()
    }

    fn node_type(&self) -> NodeType {
        NodeType::BlockStatement
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for BlockStatement {}

#[derive(Debug)]
pub struct AssignStatement {
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Node for AssignStatement {
    fn token_type(&self) -> TokenType {
        TokenType::Assign
    }

    fn node_type(&self) -> NodeType {
        NodeType::AssignStatement
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for AssignStatement {}

#[derive(Debug)]
pub struct ReadStatement {
    pub name: Identifier,
}

impl Node for ReadStatement {
    fn token_type(&self) -> TokenType {
        TokenType::Read
    }

    fn node_type(&self) -> NodeType {
        NodeType::ReadStatement
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for ReadStatement {}

#[derive(Debug)]
pub struct WriteStatement {
    pub name: Identifier,
}

impl Node for WriteStatement {
    fn token_type(&self) -> TokenType {
        TokenType::Write
    }

    fn node_type(&self) -> NodeType {
        NodeType::WriteStatement
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for WriteStatement {}

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

impl Node for IfStatement {
    fn token_type(&self) -> TokenType {
        TokenType::If
    }

    fn node_type(&self) -> NodeType {
        NodeType::IfStatement
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for IfStatement {}

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

impl Node for RepeatStatement {
    fn token_type(&self) -> TokenType {
        TokenType::Repeat
    }

    fn node_type(&self) -> NodeType {
        NodeType::RepeatStatement
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for RepeatStatement {}

#[derive(Debug)]
pub struct InfixExpression {
    pub op: Token,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Node for InfixExpression {
    fn token_type(&self) -> TokenType {
        self.op.token_type
    }

    fn node_type(&self) -> NodeType {
        NodeType::InfixExpression
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for InfixExpression {}

#[derive(Eq, PartialEq, Debug)]
pub struct Identifier {
    pub value: String,
}

impl Node for Identifier {
    fn token_type(&self) -> TokenType {
        TokenType::Ident
    }

    fn node_type(&self) -> NodeType {
        NodeType::Identifier
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for Identifier {}

#[derive(Eq, PartialEq, Debug)]
pub struct Number {
    pub value: i32,
}

impl Node for Number {
    fn token_type(&self) -> TokenType {
        TokenType::Number
    }

    fn node_type(&self) -> NodeType {
        NodeType::Number
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for Number {}

#[cfg(test)]
mod test {
    use crate::ast::{Node, Program, Statement};

    #[test]
    fn test_downcast() {
        let program = Program::new();
        let node: &dyn Node = &program;
        let down: &Program = node.as_any().downcast_ref().expect("");
    }
}
