use std::fmt::{Display, Formatter};

// 操作码定义
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    LDC, // load constant: LDC a,b(c) 表示将b+c地址处的值存储到寄存器a中
    LD,  // load: LD a,b(c) 表示将b+寄存器c地址处的值存储到寄存器a中
    ST,  // store: ST a,b(c) 表示寄存器a存储到b+寄存器c中
    IN,

    ADD, // add: ADD a,b,c 表示将寄存器b+寄存器c存储到寄存器a中，其中a、b、c恒定为累加器1、2、1
    SUB, // sub: SUB a,b,c
    MUL, // multiply: MUL a,b,c
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::LDC => write!(f, "LDC"),
            OpCode::LD => write!(f, "LD"),
            OpCode::ST => write!(f, "ST"),
            OpCode::IN => write!(f, "IN"),
            OpCode::ADD => write!(f, "ADD"),
            OpCode::SUB => write!(f, "SUB"),
            OpCode::MUL => write!(f, "MUL"),
        }
    }
}

// 寄存器的操作数定义
#[derive(Debug, Copy, Clone)]
pub enum RegisterCode {
    AC,  // 累加器1
    AC1, // 累加器2
    GP,  // Global Pointer，全局指针，指向全局变量存储的底端
    MP,  // Memory Pointer，指向
    PC,  // 程序计数器
}

impl From<usize> for RegisterCode {
    fn from(value: usize) -> Self {
        match value {
            0 => RegisterCode::AC,
            1 => RegisterCode::AC1,
            5 => RegisterCode::GP,
            6 => RegisterCode::MP,
            7 => RegisterCode::PC,
            _ => panic!("the code {:?} has not responding register", value),
        }
    }
}

impl Into<usize> for RegisterCode {
    fn into(self) -> usize {
        match self {
            RegisterCode::AC => 0,
            RegisterCode::AC1 => 1,
            RegisterCode::GP => 5,
            RegisterCode::MP => 6,
            RegisterCode::PC => 7,
        }
    }
}
