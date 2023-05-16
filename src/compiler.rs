use crate::ast::{AssignStatement, BlockStatement, Identifier, IfStatement, InfixExpression, Node, NodeType, Number, Program, ReadStatement, WriteStatement};
use crate::code::OpCode::{ADD, DIV, IN, JEQ, JLT, LD, LDA, LDC, MUL, OUT, ST, SUB};
use crate::code::RegisterCode::{AC, AC1, GP, MP, PC};
use crate::code::{OpCode, RegisterCode};
use crate::environment::{RegisterGroup, SymbolTable};
use crate::token::TokenType;

pub struct Compiler {
    pub intermedia: Vec<String>,
    pub registers: RegisterGroup,
    pub symbol_table: SymbolTable,
    pub tmp_offset: i32,
    pub emit_loc: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            intermedia: vec![],
            registers: RegisterGroup::new(),
            symbol_table: SymbolTable::new(),
            tmp_offset: 0,
            emit_loc: 0usize,
        }
    }

    pub fn compile(&mut self, node: &dyn Node) {
        match node.node_type() {
            NodeType::Program => {
                let program: &Program = node.as_any().downcast_ref().expect(""); // Rust中的向下转型语法
                for s in &program.statements {
                    self.compile(&**s)
                }
            }
            NodeType::BlockStatement => {
                let block: &BlockStatement = node.as_any().downcast_ref().expect("");
                for s in &block.statements {
                    self.compile(&**s);
                }
            }
            NodeType::ReadStatement => {
                let read: &ReadStatement = node.as_any().downcast_ref().expect("");
                self.emit_r0(IN, AC, 0usize, 0usize);
                let mut loc = self.symbol_table.look_up(&read.name.value);
                if loc == -1 {
                    loc = self.symbol_table.insert(&read.name.value);
                }
                self.emit_rm(ST, AC, loc as usize, GP);
            }
            NodeType::WriteStatement => {
                let write: &WriteStatement = node.as_any().downcast_ref().expect("");
                self.compile(&write.name);
                self.emit_r0(OUT, AC, 0usize, 0usize);
            }
            NodeType::AssignStatement => {
                let assign: &AssignStatement = node.as_any().downcast_ref().expect("");
                self.compile(&*assign.value);
                let mut loc = self.symbol_table.look_up(&assign.name.value);
                if loc == -1 {
                    loc = self.symbol_table.insert(&assign.name.value);
                }
                self.emit_rm(ST, AC, loc as usize, GP);
            }
            NodeType::IfStatement => {
                let if_stmt: &IfStatement = node.as_any().downcast_ref().expect("");
                // 编译条件
                self.compile(&*if_stmt.cond);
                // 条件地址
                let after_cond = self.emit_skip(1usize);
                // 编译then序列
                self.compile(&if_stmt.consequence);
                let after_seq = self.emit_skip(1usize);
                let current_loc = self.emit_skip(0usize);
                self.emit_backup(after_cond);
                self.emit_rm_abs(JEQ, AC, current_loc);
                self.emit_restore();
                let current_loc = self.emit_skip(0usize);
                self.emit_backup(after_seq);
                self.emit_rm_abs(LDA, PC, current_loc);
                self.emit_restore();
            }
            NodeType::InfixExpression => {
                let infix: &InfixExpression = node.as_any().downcast_ref().expect("");
                self.compile(&*infix.left);
                // 保存左操作数
                self.emit_rm(ST, AC, self.tmp_offset as usize, MP);
                self.tmp_offset -= 1;
                self.compile(&*infix.right);
                self.tmp_offset += 1;
                self.emit_rm(LD, AC1, self.tmp_offset as usize, MP);
                match infix.op.token_type {
                    TokenType::Add => self.emit_r0(ADD, AC, AC1, AC),
                    TokenType::Minus => self.emit_r0(SUB, AC, AC1, AC),
                    TokenType::Mul => self.emit_r0(MUL, AC, AC1, AC),
                    TokenType::Divide => self.emit_r0(DIV, AC, AC1, AC),
                    TokenType::LessThan => {
                        self.emit_r0(SUB, AC, AC1, AC);
                        self.emit_rm(JLT, AC, 2usize, PC);
                        self.emit_rm(LDC, AC, 0usize, AC);
                        self.emit_rm(LDA, PC, 1usize, PC);
                        self.emit_rm(LDC, AC, 1usize, AC);
                    }
                    TokenType::Equal => {
                        self.emit_r0(SUB, AC, AC1, AC);
                        self.emit_rm(JEQ, AC, 2usize, PC);
                        self.emit_rm(LDC, AC, 0usize, AC);
                        self.emit_rm(LDA, PC, 1usize, PC);
                        self.emit_rm(LDC, AC, 1usize, AC);
                    }
                    _ => panic!("token type {:?} is not infix operator", infix.op.token_type),
                }
            }
            NodeType::Identifier => {
                let ident: &Identifier = node.as_any().downcast_ref().expect("");
                let loc = self.symbol_table.look_up(&ident.value);
                self.emit_rm(LD, AC, loc as usize, GP);
            }
            NodeType::Number => {
                let number: &Number = node.as_any().downcast_ref().expect("");
                self.emit_rm(LDC, AC, number.value as usize, 0usize);
            }
            _ => {}
        }
    }

    pub fn to_intermedia_code(&self) -> String {
        let mut output = String::new();
        for line in &self.intermedia {
            output.push_str(&format!("{}\n", line));
        }
        output
    }

    fn emit_code(&mut self, code: String) {
        if self.emit_loc == self.intermedia.len() {
            self.intermedia.push(code);
        } else {
            self.intermedia[self.emit_loc] = code;
        }
        self.emit_loc += 1;
    }

    // 产生一个寄存器到内存的指令
    fn emit_rm(&mut self, op: OpCode, target: impl Into<usize>, offset: impl Into<usize>, base: impl Into<usize>) {
        let code = format!(
            "{:>3}:  {:>5}  {},{}({})",
            self.intermedia.len(),
            op,
            target.into(),
            offset.into(),
            base.into()
        );
        self.emit_code(code);
    }

    // 产生一个寄存器的指令
    fn emit_r0(&mut self, op: OpCode, target: impl Into<usize>, first: impl Into<usize>, second: impl Into<usize>) {
        let code = format!(
            "{:>3}:  {:>5}  {},{},{}",
            self.intermedia.len(),
            op,
            target.into(),
            first.into(),
            second.into()
        );
        self.emit_code(code);
    }

    // 跳过中间段的指令
    // 返回跳过之前的指令地址
    fn emit_skip(&mut self, skip: usize) -> usize {
        let loc = self.intermedia.len();
        for _ in 0..skip {
            self.intermedia.push("".to_string());
        }
        self.emit_loc += skip;
        loc
    }

    //
    fn emit_backup(&mut self, loc: usize) {
        self.emit_loc = loc;
    }

    //
    fn emit_restore(&mut self) {
        self.emit_loc = self.intermedia.len();
    }

    fn emit_rm_abs(&mut self, op: OpCode, target: impl Into<usize>, absolute: impl Into<usize>) {
        let code = format!(
            "{:>3}:  {:>5} {},{}({})",
            self.emit_loc,
            op,
            target.into(),
            absolute.into() - (self.emit_loc + 1),
            PC
        );
        self.emit_code(code);
    }
}

#[cfg(test)]
mod test {
    use crate::compiler::Compiler;
    use crate::parser::Parser;

    #[test]
    fn test_read() {
        let input = "read x;";
        let mut parser = Parser::new(input);
        let mut compiler = Compiler::new();
        compiler.compile(&parser.parse_program());
        println!("{:?}", compiler.intermedia);
    }

    #[test]
    fn test_write() {
        let input = "read x;write x;";
        let mut parser = Parser::new(input);
        let mut compiler = Compiler::new();
        compiler.compile(&parser.parse_program());
        println!("{:?}", compiler.intermedia);
    }

    #[test]
    fn test_assign() {
        let input = "x := 5;";
        let mut parser = Parser::new(input);
        let mut compiler = Compiler::new();
        compiler.compile(&parser.parse_program());
        println!("{:?}", compiler.intermedia);
    }

    #[test]
    fn test_infix() {
        let input = "
x := 5 + 3;
y := x * 4;
z := x < y;";
        let mut parser = Parser::new(input);
        let mut compiler = Compiler::new();
        compiler.compile(&parser.parse_program());
        println!("{}", compiler.to_intermedia_code());
    }

    #[test]
    fn test_unit() {
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
        let mut compiler = Compiler::new();
        compiler.compile(&parser.parse_program());
        println!("{}", compiler.to_intermedia_code());
    }
}
