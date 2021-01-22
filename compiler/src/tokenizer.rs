use super::location::{Location, Point};
use super::term_color::red_bold;
use super::token::{Comma, Equal, Identifier, LineBreak, StringLiteral, Token};
use once_cell::sync::Lazy;

#[derive(PartialEq, Eq)]
enum TokenizerState {
    Ready,
    Identifier,
    StringLiteral,
}

static TOKENIZATION_ERROR: Lazy<String> = Lazy::new(|| red_bold("Tokenization error:"));

pub fn tokenize(line: &str) -> Result<Vec<Token>, String> {
    let len = line.len();

    let mut tokens = Vec::<Token>::new();
    let mut state = TokenizerState::Ready;
    let mut ident_start = 0;
    let mut ident_acc = String::new();
    let mut str_lit_start = 0;
    let mut str_lit = String::new();
    let mut line_number = 0;
    let mut column_number = 0;

    for (i, c) in line.chars().enumerate() {
        if c == '\n' {
            if state == TokenizerState::Identifier {
                let location = Location {
                    start: Point::new(line_number, ident_start),
                    end: Point::new(line_number, column_number - 1),
                };
                tokens.push(Token::Ident(Identifier {
                    name: ident_acc.clone(),
                    location,
                }));
                ident_acc.clear();
                state = TokenizerState::Ready;
            } else if state == TokenizerState::StringLiteral {
                return Err(format!(
                    "{} ({}:{}) Unexpected end of line",
                    TOKENIZATION_ERROR.as_str(),
                    line_number,
                    column_number
                ));
            }

            tokens.push(Token::LineBreak(LineBreak {
                loc: Point::new(line_number, column_number),
            }));

            line_number += 1;
            column_number = 0;
            continue;
        }

        if state == TokenizerState::StringLiteral {
            if c == '"' {
                state = TokenizerState::Ready;
                let location = Location {
                    start: Point::new(line_number, str_lit_start),
                    end: Point::new(line_number, column_number),
                };
                tokens.push(Token::StrLit(StringLiteral {
                    value: str_lit.clone(),
                    location,
                }));
                str_lit.clear();
                column_number += 1;
                continue;
            }

            str_lit.push(c);
            column_number += 1;
            continue;
        }

        if c.is_whitespace() {
            if state == TokenizerState::Identifier {
                let location = Location {
                    start: Point::new(line_number, ident_start),
                    end: Point::new(line_number, column_number - 1),
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
            str_lit_start = column_number;
        } else if c == ',' {
            if state == TokenizerState::Identifier {
                let location = Location {
                    start: Point::new(line_number, ident_start),
                    end: Point::new(line_number, column_number - 1),
                };
                tokens.push(Token::Ident(Identifier {
                    name: ident_acc.clone(),
                    location,
                }));
                ident_acc.clear();
                state = TokenizerState::Ready;
            }

            tokens.push(Token::Comma(Comma {
                loc: Point::new(line_number, column_number),
            }));
        } else if c == '=' {
            if state == TokenizerState::Identifier {
                let location = Location {
                    start: Point::new(line_number, ident_start),
                    end: Point::new(line_number, column_number - 1),
                };
                tokens.push(Token::Ident(Identifier {
                    name: ident_acc.clone(),
                    location,
                }));
                ident_acc.clear();
                state = TokenizerState::Ready;
            }
            tokens.push(Token::Equal(Equal {
                loc: Point::new(line_number, column_number),
            }));
        } else {
            if state != TokenizerState::Identifier {
                state = TokenizerState::Identifier;
                ident_start = column_number;
            }
            ident_acc.push(c);
        }

        if i == len - 1 {
            if state == TokenizerState::Identifier {
                let location = Location {
                    start: Point::new(line_number, ident_start),
                    end: Point::new(line_number, column_number),
                };
                tokens.push(Token::Ident(Identifier {
                    name: ident_acc.clone(),
                    location,
                }));
            } else if state == TokenizerState::StringLiteral {
                return Err(format!(
                    "{} ({}:{}) Unexpected end of line",
                    TOKENIZATION_ERROR.as_str(),
                    line_number,
                    column_number
                ));
            }
        }

        column_number += 1;
    }

    Ok(tokens)
}
