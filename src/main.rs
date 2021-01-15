extern crate basic;

use std::{cell::RefCell, fs};
use basic::Token;

fn main() {
    let content = fs::read_to_string("./hello.bas").expect("Failed to load source file");
    for line in content.split("\n").into_iter() {
        println!("{}", line);
        println!("  {:?}", tokenize(String::from(line)));
    }
}

fn tokenize(line: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut keyword_acc = String::new();

    let mut is_str_lit = false;
    let str_lit = RefCell::new(String::new());

    for c in line.chars() {
        if is_str_lit {
            if c == '"' {
                is_str_lit = false;
                tokens.push(Token::StrLit(str_lit.borrow().clone()));
                continue;
            }

            str_lit.borrow_mut().push(c);
            continue;
        }

        if c.is_whitespace() {
            if !keyword_acc.is_empty() {
                tokens.push(Token::Ident(keyword_acc.clone()));
                keyword_acc.clear();
            }
        } else if c == '"' {
            is_str_lit = true;
        } else {
            keyword_acc.push(c);
        }
    }
    if !keyword_acc.is_empty() {
        println!("  {}", keyword_acc);
    }

    tokens
}
