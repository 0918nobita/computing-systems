use super::location::{Location, Point};
use super::term_color::red_bold;
use super::token::{Comma, Equal, Identifier, LineBreak, StringLiteral, Token};
use once_cell::sync::Lazy;

#[derive(PartialEq, Eq)]
enum TokenizerState {
    Ready,
    Identifier(IdentState),
    StringLiteral(StrLitState),
}

#[derive(PartialEq, Eq)]
struct IdentState {
    start: i32,
    acc: String,
}

#[derive(PartialEq, Eq)]
struct StrLitState {
    start: i32,
    acc: String,
}

static TOKENIZATION_ERROR: Lazy<String> = Lazy::new(|| red_bold("Tokenization error:"));

pub fn tokenize(src: &str) -> Result<Vec<Token>, String> {
    let len = src.len();

    let mut tokens = Vec::<Token>::new();
    let mut state = TokenizerState::Ready;
    let mut line_number = 0;
    let mut column_number = 0;

    for (i, c) in src.chars().enumerate() {
        if c == '\n' {
            try_tokenizing_ident(&mut tokens, &mut state, line_number, column_number - 1);

            if let TokenizerState::StringLiteral(_) = state {
                return Err(format!(
                    "{} ({}:{}) Unexpected [EOL] in string literal",
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

        if let TokenizerState::StringLiteral(StrLitState { start, ref acc }) = state {
            if c == '"' {
                let location = Location {
                    start: Point::new(line_number, start),
                    end: Point::new(line_number, column_number),
                };
                tokens.push(Token::StrLit(StringLiteral {
                    value: acc.clone(),
                    location,
                }));
                column_number += 1;
                state = TokenizerState::Ready;
                continue;
            }

            state = TokenizerState::StringLiteral(StrLitState {
                start,
                acc: format!("{}{}", acc, c),
            });
            column_number += 1;
            continue;
        }

        if c.is_whitespace() {
            try_tokenizing_ident(&mut tokens, &mut state, line_number, column_number - 1)
        } else if c == '"' {
            try_tokenizing_ident(&mut tokens, &mut state, line_number, column_number - 1);

            state = TokenizerState::StringLiteral(StrLitState {
                start: column_number,
                acc: String::new(),
            });
        } else if c == ',' {
            try_tokenizing_ident(&mut tokens, &mut state, line_number, column_number - 1);

            tokens.push(Token::Comma(Comma {
                loc: Point::new(line_number, column_number),
            }));
        } else if c == '=' {
            try_tokenizing_ident(&mut tokens, &mut state, line_number, column_number - 1);

            tokens.push(Token::Equal(Equal {
                loc: Point::new(line_number, column_number),
            }));
        } else if let TokenizerState::Identifier(IdentState { start, ref acc }) = state {
            let acc = format!("{}{}", acc, c);
            state = TokenizerState::Identifier(IdentState { start, acc })
        } else {
            let acc = c.to_string();
            state = TokenizerState::Identifier(IdentState {
                start: column_number,
                acc,
            })
        }

        if i == len - 1 {
            try_tokenizing_ident(&mut tokens, &mut state, line_number, column_number);

            if let TokenizerState::StringLiteral(_) = state {
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

fn try_tokenizing_ident(
    tokens: &mut Vec<Token>,
    state: &mut TokenizerState,
    line: i32,
    column: i32,
) {
    if let TokenizerState::Identifier(IdentState { start, ref acc }) = state {
        let location = Location {
            start: Point::new(line, *start),
            end: Point::new(line, column),
        };
        tokens.push(Token::Ident(Identifier {
            name: acc.clone(),
            location,
        }));
        *state = TokenizerState::Ready;
    }
}
