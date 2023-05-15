use crate::token::{self, Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    pos: i32,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: Self::remove_comment(input).chars().collect(),
            pos: -1,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.consume_spaces();
        let ch = self.next_char();
        match ch {
            ';' => Token::new(TokenType::SemiColon, ";"),
            '<' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::EqualLessThan, "<=")
                } else {
                    Token::new(TokenType::LessThan, "<")
                }
            }
            '=' => Token::new(TokenType::Equal, "="),
            ':' => {
                self.next_char();
                Token::new(TokenType::Assign, ":=")
            }
            '*' => Token::new(TokenType::Mul, "*"),
            '-' => Token::new(TokenType::Minus, "-"),
            '+' => Token::new(TokenType::Add, "+"),
            '/' => Token::new(TokenType::Divide, "/"),
            '"' => {
                let literal = self.read_string();
                Token::new(TokenType::String, &literal)
            }
            '\0' => Token::new(TokenType::Eof, ""),
            _ => {
                if Self::is_letter(ch) {
                    let literal = self.read_identifier();
                    Token::new(token::look_up_keywords(&literal), &literal)
                } else if Self::is_digit(ch) {
                    let literal = self.read_number();
                    Token::new(TokenType::Number, &literal)
                } else {
                    Token::new(TokenType::Illegal, "")
                }
            }
        }
    }

    pub fn peek_char(&self) -> char {
        if self.pos as usize == self.input.len() - 1 {
            0 as char
        } else {
            self.input[(self.pos + 1) as usize]
        }
    }

    pub fn next_char(&mut self) -> char {
        let next = self.peek_char();
        if next != 0 as char {
            self.pos += 1;
        }
        return next;
    }

    pub fn consume_spaces(&mut self) {
        loop {
            let ch = self.peek_char();
            if ch != '\n' && ch != '\r' && ch != '\t' && ch != ' ' {
                break;
            }
            self.next_char();
        }
    }

    fn is_letter(ch: char) -> bool {
        ch >= 'a' && ch <= 'z' || ch >= 'A' && ch <= 'Z'
    }

    fn is_digit(ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn read_identifier(&mut self) -> String {
        self.pos -= 1;
        let mut output = String::new();
        loop {
            let ch = self.peek_char();
            if Self::is_letter(ch) {
                output.push(ch);
                self.next_char();
            } else {
                break;
            }
        }
        output
    }

    fn read_number(&mut self) -> String {
        self.pos -= 1;
        let mut output = String::new();
        loop {
            let ch = self.peek_char();
            if Self::is_digit(ch) {
                output.push(ch);
                self.next_char();
            } else {
                break;
            }
        }
        output
    }

    pub fn read_string(&mut self) -> String {
        let mut output = String::new();
        loop {
            let ch = self.next_char();
            if ch == '"' {
                break;
            }
            output.push(ch)
        }
        output
    }

    // 去除输入中的注释部分
    fn remove_comment(input: &str) -> String {
        let mut output = String::new();
        let mut in_comment = false;
        for ch in input.chars() {
            if ch == '{' {
                in_comment = true;
            } else if ch == '}' {
                in_comment = false;
            } else if !in_comment {
                output.push(ch);
            }
        }
        return output;
    }
}

#[cfg(test)]
mod test {
    use crate::token::TokenType;

    use super::Lexer;

    #[test]
    pub fn unit_test() {
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
        let mut l = Lexer::new(input);
        loop {
            let token = l.next_token();
            println!("{:?}", token);
            if token.token_type == TokenType::Eof {
                break;
            }
        }
    }
}
