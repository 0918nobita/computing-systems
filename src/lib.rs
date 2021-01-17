use std::cell::RefCell;
use serde::{Serialize};

#[derive(Clone, Copy, Debug, Serialize)]
struct LocationEndpoint {
    line: i32,
    column: i32,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct Location {
    start: LocationEndpoint,
    end: LocationEndpoint,
}

trait Locatable {
    fn locate(&self) -> Location;
}

#[derive(Clone, Debug, Serialize)]
pub struct Identifier {
    name: String,
    location: Location,
}

impl Locatable for Identifier {
    fn locate(&self) -> Location {
        self.location
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct StringLiteral {
    value: String,
    location: Location,
}

impl Locatable for StringLiteral {
    fn locate(&self) -> Location {
        self.location
    }
}

#[derive(Debug)]
pub struct Comma {
    loc: LocationEndpoint,
}

impl Locatable for Comma {
    fn locate(&self) -> Location {
        Location { start: self.loc, end: self.loc }
    }
}

#[derive(Debug)]
pub enum Token {
    Ident(Identifier),
    StrLit(StringLiteral),
    Comma(Comma),
}

#[derive(Debug, Serialize)]
pub enum ExprAst {
    StrLit(StringLiteral),
}

#[derive(Debug, Serialize)]
pub enum StmtAst {
    CallProc(Identifier, Vec<ExprAst>),
}

#[derive(PartialEq, Eq)]
enum TokenizerState {
    Ready,
    Identifier,
    StringLiteral,
}

pub fn tokenize(line: String, line_number: i32) -> Result<Vec<Token>, String> {
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
                    start: LocationEndpoint {
                        line: line_number,
                        column: str_lit_start,
                    },
                    end: LocationEndpoint {
                        line: line_number,
                        column: i as i32,
                    },
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
                    start: LocationEndpoint {
                        line: line_number,
                        column: ident_start,
                    },
                    end: LocationEndpoint {
                        line: line_number,
                        column: i as i32 - 1,
                    },
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
                    start: LocationEndpoint {
                        line: line_number,
                        column: ident_start,
                    },
                    end: LocationEndpoint {
                        line: line_number,
                        column: i as i32 - 1,
                    },
                };
                tokens.push(Token::Ident(Identifier { name: ident_acc.clone(), location }));
            }
            tokens.push(Token::Comma(Comma { loc: LocationEndpoint { line: line_number, column: i as i32 } }));
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
            start: LocationEndpoint {
                line: line_number,
                column: ident_start,
            },
            end: LocationEndpoint {
                line: line_number,
                column: line.len() as i32 - 1,
            },
        };
        tokens.push(Token::Ident(Identifier {
            name: ident_acc.clone(),
            location,
        }))
    }

    if state == TokenizerState::StringLiteral {
        Err(String::from("Syntax error: Expected end of string literal"))
    } else {
        Ok(tokens)
    }
}

pub fn parse(tokens: &[Token]) -> Result<StmtAst, String> {
    if let Some(head) = tokens.first() {
        match head {
            Token::Ident(ident) => {
                let (args, rest) = parse_argument_list(&tokens[1..])?;
                if rest.is_empty() {
                    Ok(StmtAst::CallProc(ident.clone(), args))
                } else {
                    Err(format!("Syntax error: Unexpected tokens\n  {:?}", rest))
                }
            }
            _ => Err(String::from("Syntax error: Expected identifier")),
        }
    } else {
        Err(String::from("Syntax error: No tokens found"))
    }
}

pub fn parse_argument_list(tokens: &[Token]) -> Result<(Vec<ExprAst>, &[Token]), String> {
    let mut args = Vec::<ExprAst>::new();
    let (first_arg, mut remaining_tokens) = parse_expr(tokens)?;
    args.push(first_arg);
    loop {
        if let Some(Token::Comma { .. }) = remaining_tokens.first() {
            remaining_tokens = &remaining_tokens[1..];
        } else {
            break;
        }
        match parse_expr(remaining_tokens) {
            Ok((arg, rest)) => {
                args.push(arg);
                remaining_tokens = rest;
            },
            Err(_) => {
                break;
            },
        }
    }
    Ok((args, remaining_tokens))
}

pub fn parse_expr(tokens: &[Token]) -> Result<(ExprAst, &[Token]), String> {
    if let Some(head) = tokens.first() {
        match head {
            Token::StrLit(str_lit) => Ok((ExprAst::StrLit(str_lit.clone()), &tokens[1..])),
            _ => Err(format!("Syntax error: Expected expression\n  {:?}", head)),
        }
    } else {
        Err(String::from("Syntax error: Unexpected end of statement"))
    }
}

pub type Asm = String;

pub fn compile(stmts: &[StmtAst]) -> Result<Asm, String> {
    let mut data_section = String::from("section .data\n");
    let mut text_section = String::from("\nsection .text\n_start:\n");

    for (i, stmt) in stmts.into_iter().enumerate() {
        match stmt {
            StmtAst::CallProc(proc, args) if proc.name.eq("PRINT") => {
                if args.len() > 1 {
                    return Err(String::from("Failed to compile: Too many arguments"));
                }
                if let Some(head) = args.first() {
                    match head {
                        ExprAst::StrLit(str_lit) => {
                            data_section.push_str(
                                format!("    dat{} db '{}', 10, 0\n", i, str_lit.value).as_str(),
                            );
                            text_section.push_str(
                                format!("    mov rdi, dat{}\n    call printString\n", i).as_str(),
                            );
                        }
                    }
                } else {
                    return Err(String::from("Failed to compile: Too few arguments"));
                }
            }
            _ => {
                return Err(String::from("Failed to compile: Unknown procedure"));
            }
        }
    }

    text_section.push_str("    mov rax, 60\n"); // sys_exit
    text_section.push_str("    xor rdi, rdi\n");
    text_section.push_str("    syscall\n\n");
    text_section.push_str("printString:\n");
    text_section.push_str("    call stringLength\n");
    text_section.push_str("    mov rdx, rax\n");
    text_section.push_str("    mov rax, 1\n");
    text_section.push_str("    mov rsi, rdi\n");
    text_section.push_str("    mov rdi, 1\n");
    text_section.push_str("    syscall\n");
    text_section.push_str("    ret\n\n");
    text_section.push_str("stringLength:\n");
    text_section.push_str("    xor rax, rax\n");
    text_section.push_str(".loop:\n");
    text_section.push_str("    cmp byte[rdi+rax], 0\n");
    text_section.push_str("    je .end\n");
    text_section.push_str("    inc rax\n");
    text_section.push_str("    jmp .loop\n");
    text_section.push_str(".end:\n");
    text_section.push_str("    ret\n");

    Ok(format!(
        "bits 64\nglobal _start\n{}{}",
        data_section, text_section
    ))
}
