use crate::ast::{
    AssignStatement, BlockStatement, Expression, Identifier, IfStatement, InfixExpression, Number, Program,
    ReadStatement, RepeatStatement, Statement, WriteStatement,
};
use crate::lexer::Lexer;
use crate::token::TokenType::Until;
use crate::token::{Token, TokenType};

pub struct Parser {
    lexer: Lexer,
    peek: Token,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let peek = lexer.next_token();
        Self { lexer, peek }
    }

    fn peek_token(&self) -> Token {
        self.peek.clone()
    }

    fn next_token(&mut self) -> Token {
        let cur = self.peek_token();
        self.peek = self.lexer.next_token();
        cur
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        while self.peek_token().token_type != TokenType::Eof {
            let stmt = self.parse_statement();
            program.statements.push(stmt);
        }
        program
    }

    fn parse_statement(&mut self) -> Box<dyn Statement> {
        match self.peek_token().token_type {
            TokenType::Ident => Box::new(self.parse_assign_statement()),
            TokenType::If => Box::new(self.parse_if_statement()),
            TokenType::Repeat => Box::new(self.parse_repeat_statement()),
            TokenType::Read => Box::new(self.parse_read_statement()),
            TokenType::Write => Box::new(self.parse_write_statement()),
            _ => panic!(
                "the token type represents no statement:{:?}",
                self.peek_token().token_type
            ),
        }
    }

    fn parse_assign_statement(&mut self) -> AssignStatement {
        let ident = self.next_token(); // 一定是 TokenType::Ident
        if self.peek_token().token_type != TokenType::Assign {
            panic!("expected TokenType::Assign, found: {:?}", self.peek_token().token_type);
        }
        self.next_token(); // pass :=
        let right_exp = self.parse_expression();
        self.next_token(); // pass ;
        AssignStatement {
            name: Identifier { value: ident.literal },
            value: right_exp,
        }
    }

    fn parse_if_statement(&mut self) -> IfStatement {
        self.next_token(); // pass If
        let cond = self.parse_expression();
        if self.peek_token().token_type != TokenType::Then {
            panic!("expected TokenType::Then, found: {:?}", self.peek_token().token_type);
        }
        self.next_token(); // pass then
        let consequence = self.parse_block_statement();
        self.next_token(); // pass end
        IfStatement { cond, consequence }
    }

    fn parse_repeat_statement(&mut self) -> RepeatStatement {
        self.next_token(); // pass repeat
        let consequence = self.parse_block_statement();
        self.next_token(); // pass until
        let cond = self.parse_expression();
        self.next_token(); // pass ;
        RepeatStatement { cond, consequence }
    }

    fn parse_read_statement(&mut self) -> ReadStatement {
        self.next_token(); // pass read
        let ident = self.next_token();
        self.next_token(); // pass ;
        ReadStatement {
            name: Identifier { value: ident.literal },
        }
    }

    fn parse_write_statement(&mut self) -> WriteStatement {
        self.next_token(); // pass read
        let ident = self.next_token();
        self.next_token(); // pass ;
        WriteStatement {
            name: Identifier { value: ident.literal },
        }
    }

    // 解析到End或Until为止；并且不会消耗这两个token
    fn parse_block_statement(&mut self) -> BlockStatement {
        let mut block = BlockStatement { statements: vec![] };
        while self.peek_token().token_type != TokenType::End && self.peek_token().token_type != Until {
            let stmt = self.parse_statement();
            block.statements.push(stmt);
        }
        block
    }

    // 注意到标准代码中只出现了两种表达式：单元、双元，没有复合表达式，故暂不考虑
    fn parse_expression(&mut self) -> Box<dyn Expression> {
        let left = self.parse_prefix_expression();
        if self.peek_token().token_type != TokenType::SemiColon && self.peek_token().token_type != TokenType::Then {
            let op = self.next_token();
            Box::new(self.parse_infix_expression(op, left))
        } else {
            left
        }
    }

    fn parse_prefix_expression(&mut self) -> Box<dyn Expression> {
        match self.peek_token().token_type {
            TokenType::Ident => Box::new(self.parse_ident()),
            TokenType::Number => Box::new(self.parse_number()),
            _ => panic!(
                "token type: {:?} is not prefix expression",
                self.peek_token().token_type
            ),
        }
    }

    fn parse_infix_expression(&mut self, op: Token, left: Box<dyn Expression>) -> InfixExpression {
        InfixExpression {
            op,
            left,
            right: self.parse_prefix_expression(),
        }
    }

    fn parse_ident(&mut self) -> Identifier {
        Identifier {
            value: self.next_token().literal,
        }
    }

    fn parse_number(&mut self) -> Number {
        Number {
            value: self.next_token().literal.parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_read_statement() {
        let input = "read x;";
        let mut parser = Parser::new(input);
        println!("{:?}", parser.parse_program());
    }

    #[test]
    fn test_write_statement() {
        let input = "write x;";
        let mut parser = Parser::new(input);
        println!("{:?}", parser.parse_program());
    }

    #[test]
    fn test_assign_statement() {
        let input = "x := 5; y := x + 3";
        let mut parser = Parser::new(input);
        println!("{:?}", parser.parse_program());
    }

    #[test]
    fn test_if_statement() {
        let input = "if a < b then x := 3; end";
        let mut parser = Parser::new(input);
        println!("{:?}", parser.parse_program());
    }

    #[test]
    fn test_repeat_statement() {
        let input = "
if 0 < x then
    fact := 1;
    repeat
        fact := fact * x;
        x := x - 1;
    until x = 0;
    write fact; { output factorial of x }
end";
        let mut parser = Parser::new(input);
        println!("{:?}", parser.parse_program());
    }

    #[test]
    fn unit_test() {
        let input = "
{ Sample program in TINY language - computes factorial}
read x; { input an integer }
if 0 < x then { don't compute if x <= 0 }
    fact := 1;
    repeat
        fact := fact * x;
        x := x - 1;
    until x = 0;
    write fact; { output factorial of x }
end";
        let mut parser = Parser::new(input);
        println!("{:?}", parser.parse_program());
    }
}
