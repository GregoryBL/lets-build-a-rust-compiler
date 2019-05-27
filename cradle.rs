use std::io::{self, Read, Write};
// use ::utils::*;

pub struct Cradle {
    buff: char
}

impl Cradle {

    fn get_char(&mut self) {
        println!("get_char");
        let mut b: [u8; 1] = [0];
        io::stdin().read(&mut b[..]).unwrap();
        self.buff = b[0] as char;
    }

    fn init(&mut self) {
        self.get_char();
    }

    pub fn is_match(&mut self, c: char) {
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

    pub fn expression(&mut self) {
        if emit_ln("expression".to_string()).is_err() {
            abort("Didn't write expression".to_string())
        };
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
pub fn emit(s: String) -> io::Result<()> {
    io::stdout().write_all(format!("\t{}", s).as_bytes())?;
    Ok(())
}

pub fn emit_ln(s: String) -> io::Result<()> {
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
