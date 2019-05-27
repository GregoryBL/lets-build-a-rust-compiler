use std::io::{self, Read, Write};
// use ::utils::*;

pub struct Cradle {
    buff: char
}

impl Cradle {

    fn get_char(&mut self) {
        let mut b: [u8; 1] = [0];
        io::stdin().read(&mut b[..]).unwrap();
        self.buff = b[0] as char;
    }

    fn init(&mut self) {
        self.get_char();
    }

    pub fn match_char(&mut self, c: char) {
        if c == self.buff {
            self.get_char();
        } else {
            expected(format!("\n\n  {}  \n\n", c));
        }
    }
    
    pub fn get_num(&mut self) -> char {
        if !self.buff.is_numeric() {
            expected("Integer".to_string());
            "a".chars().next().unwrap()
        } else {
            let tmp = self.buff;
            self.get_char();
            tmp
        }
    }

    pub fn get_name(&mut self) -> char {
        if !self.buff.is_alphabetic() {
            expected("Name".to_string());
            "a".chars().next().unwrap()
        } else {
            let tmp = self.buff;
            self.get_char();
            tmp
        }
    }

    pub fn term(&mut self) {
        emit_ln(format!("MOVE #{},D0", self.get_num()))
    }

    pub fn expression(&mut self) {
        self.term();
        while (vec!['+', '-']).into_iter().find(|&x| x == self.buff).is_some() {
            emit_ln("MOVE D0,D1".to_string());
            match self.buff {
                '+' => self.add(),
                '-' => self.subtract(),
                _ => expected("Addop".to_string()),
            };
        };
    }
    
    fn add(&mut self) {
        self.match_char('+');
        self.term();
        emit_ln("ADD D1,D0".to_string());
    }

    fn subtract(&mut self) {
        self.match_char('-');
        self.term();
        emit_ln("SUB D1,D0".to_string());
        emit_ln("NEG D0".to_string());
    }
}

/// Some error helpers

pub fn error(s: String) {
    println!("");
    println!("Error: {}", s);
}

pub fn abort(s: String) {
    error(s);
    std::process::exit(1);
}

pub fn expected(s: String) {
    abort(format!("{} expected.", s));
}

/// Writing
pub fn emit(s: String) {
    if io::stdout().write_all(format!("\t{}", s).as_bytes()).is_err() {
        abort("Didn't emit correctly.".to_string());
    };
}

pub fn emit_ln(s: String) {
    let tmp = emit(s);
    println!("");
    tmp
}


pub fn main() {
    println!("starting cradle");
    let mut c: Cradle = Cradle { buff: ' ' };
    c.init();
    c.expression();
}
