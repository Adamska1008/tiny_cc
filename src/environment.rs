use std::collections::HashMap;

pub struct RegisterGroup {}

impl RegisterGroup {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct SymbolTable {
    table: HashMap<String, i32>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { table: HashMap::new() }
    }

    // 返回名字为name的变量的内存地址
    // 若不存在，返回-1
    pub fn look_up(&self, name: &str) -> i32 {
        if self.table.contains_key(name) {
            *self.table.get(name).unwrap()
        } else {
            -1
        }
    }

    pub fn insert(&mut self, name: &str) -> i32 {
        if self.table.contains_key(name) {
            -1
        } else {
            let size = self.table.len();
            self.table.insert(name.to_string(), size as i32);
            size as i32
        }
    }
}
