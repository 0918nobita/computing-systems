use super::location::{Location, Point};
use super::term_color::red_bold;
use super::token::{Comma, Equal, Identifier, StringLiteral, Token};
use once_cell::sync::Lazy;
use std::cell::RefCell;

#[derive(PartialEq, Eq)]
enum TokenizerState {
    Ready,
    Identifier,
    StringLiteral,
}

static TOKENIZATION_ERROR: Lazy<String> = Lazy::new(|| red_bold("Tokenization error:"));

pub fn tokenize<L: Into<String>>(line: L, line_index: i32) -> Result<Vec<Token>, String> {
    let line: String = line.into();
    let mut tokens = Vec::new();

    let mut state = TokenizerState::Ready;

    let mut ident_start: i32 = 0;
    let mut ident_acc = String::new();

    let mut str_lit_start = 0;
    let str_lit = RefCell::new(String::new());

    for (i, c) in line.chars().enumerate() {
        if state == TokenizerState::StringLiteral {
            if c == '"' {
                state = TokenizerState::Ready;
                let location = Location {
                    start: Point::new(line_index, str_lit_start),
                    end: Point::new(line_index, i as i32),
                };
                tokens.push(Token::StrLit(StringLiteral {
                    value: str_lit.borrow().clone(),
                    location,
                }));
                str_lit.borrow_mut().clear();
                continue;
            }

            str_lit.borrow_mut().push(c);
            continue;
        }

        if c.is_whitespace() {
            if !ident_acc.is_empty() {
                let location = Location {
                    start: Point::new(line_index, ident_start),
                    end: Point::new(line_index, i as i32 - 1),
                };
                tokens.push(Token::Ident(Identifier {
                    name: ident_acc.clone(),
                    location,
                }));
                ident_acc.clear();
                state = TokenizerState::Ready;
            }
        } else if c == '"' {
            state = TokenizerState::StringLiteral;
            str_lit_start = i as i32;
        } else if c == ',' {
            if state == TokenizerState::Identifier {
                let location = Location {
                    start: Point::new(line_index, ident_start),
                    end: Point::new(line_index, i as i32 - 1),
                };
                tokens.push(Token::Ident(Identifier {
                    name: ident_acc.clone(),
                    location,
                }));
            }
            tokens.push(Token::Comma(Comma {
                loc: Point::new(line_index, i as i32),
            }));
        } else if c == '=' {
            if state == TokenizerState::Identifier {
                let location = Location {
                    start: Point::new(line_index, ident_start),
                    end: Point::new(line_index, i as i32 - 1),
                };
                tokens.push(Token::Ident(Identifier {
                    name: ident_acc.clone(),
                    location,
                }));
            }
            tokens.push(Token::Equal(Equal {
                loc: Point::new(line_index, i as i32),
            }));
        } else if state != TokenizerState::Identifier {
            state = TokenizerState::Identifier;
            ident_start = i as i32;
            ident_acc.push(c);
        } else {
            ident_acc.push(c);
        }
    }

    if !ident_acc.is_empty() {
        let location = Location {
            start: Point::new(line_index, ident_start),
            end: Point::new(line_index, line.len() as i32 - 1),
        };
        tokens.push(Token::Ident(Identifier {
            name: ident_acc.clone(),
            location,
        }))
    }

    if state == TokenizerState::StringLiteral {
        Err(format!(
            "{} ({}:{}) Expected end of string literal",
            TOKENIZATION_ERROR.as_str(),
            line_index + 1,
            line.len()
        ))
    } else {
        Ok(tokens)
    }
}
