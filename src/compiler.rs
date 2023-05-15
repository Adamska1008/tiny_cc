use crate::ast::{Node, NodeType, Program, ReadStatement};
use crate::code::{OpCode, RegisterCode};
use crate::environment::{RegisterGroup, SymbolTable};
use crate::token::TokenType;

pub struct Compiler {
    pub intermedia: Vec<String>,
    pub registers: RegisterGroup,
    pub symbol_table: SymbolTable,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            intermedia: vec![],
            registers: RegisterGroup::new(),
            symbol_table: SymbolTable::new(),
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
            NodeType::ReadStatement => {
                let read: &ReadStatement = node.as_any().downcast_ref().expect("");
                self.emit_r0(OpCode::IN, RegisterCode::AC, 0usize, 0usize);
                let mut loc = self.symbol_table.look_up(&read.name.value);
                if loc == -1 {
                    loc = self.symbol_table.insert(&read.name.value);
                }
                self.emit_rm(OpCode::ST, RegisterCode::AC, loc as usize, RegisterCode::GP);
            }
            _ => {}
        }
    }

    //
    fn emit_rm(
        &mut self,
        op: OpCode,
        target: impl Into<usize>,
        offset: impl Into<usize>,
        base: impl Into<usize>,
    ) {
        let target = target.into();
        let offset = offset.into();
        let base = base.into();
        self.intermedia.push(format!(
            "{:>3}:  {:>5}  {},{},{}",
            self.intermedia.len(),
            op,
            target,
            offset,
            base
        ));
    }

    //
    fn emit_r0(
        &mut self,
        op: OpCode,
        target: impl Into<usize>,
        first: impl Into<usize>,
        second: impl Into<usize>,
    ) {
        let target = target.into();
        let first = first.into();
        let second = second.into();
        self.intermedia.push(format!(
            "{:>3}:  {:>5}  {},{},{}",
            self.intermedia.len(),
            op,
            target,
            first,
            second
        ));
    }
}

#[cfg(test)]
mod test {
    use crate::compiler::Compiler;
    use crate::parser::Parser;

    #[test]
    fn test_read() {
        let input = "read x";
        let mut parser = Parser::new(input);
        let mut compiler = Compiler::new();
        compiler.compile(&parser.parse_program());
        println!("{:?}", compiler.intermedia);
    }
}
