use std::cell::RefCell;

#[derive(Clone, Copy, Debug)]
struct LocationEndpoint {
    line: i32,
    column: i32,
}

#[derive(Clone, Copy, Debug)]
struct Location {
    start: LocationEndpoint,
    end: LocationEndpoint,
}

trait Locatable {
    fn locate(&self) -> Location;
}

#[derive(Debug)]
struct Identifier {
    name: String,
    location: Location,
}

impl Locatable for Identifier {
    fn locate(&self) -> Location {
        self.location
    }
}

#[derive(Debug)]
struct StringLiteral {
    value: String,
    location: Location,
}

impl Locatable for StringLiteral {
    fn locate(&self) -> Location {
        self.location
    }
}

#[derive(Debug)]
pub enum Token {
    Ident(String),
    StrLit(String),
}

#[derive(Debug)]
pub enum ExprAst {
    StrLit(String),
}

#[derive(Debug)]
pub enum StmtAst {
    CallProc(String, Vec<ExprAst>),
}

pub fn tokenize(line: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut keyword_acc = String::new();

    let mut is_str_lit = false;
    let str_lit = RefCell::new(String::new());

    for c in line.chars() {
        if is_str_lit {
            if c == '"' {
                is_str_lit = false;
                tokens.push(Token::StrLit(str_lit.borrow().clone()));
                str_lit.borrow_mut().clear();
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
        tokens.push(Token::Ident(keyword_acc.clone()))
    }

    tokens
}

pub fn parse(tokens: Vec<Token>) -> Result<StmtAst, String> {
    if let Some(head) = tokens.first() {
        match head {
            Token::Ident(ident) => {
                let (expr_ast, rest) = parse_expr(&tokens[1..])?;
                if rest.is_empty() {
                    Ok(StmtAst::CallProc(ident.clone(), vec![expr_ast]))
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
            StmtAst::CallProc(proc, args) if proc.eq("PRINT") => {
                if args.len() > 1 {
                    return Err(String::from("Failed to compile: Too many arguments"));
                }
                if let Some(head) = args.first() {
                    match head {
                        ExprAst::StrLit(str_lit) => {
                            data_section.push_str(
                                format!("    dat{} db '{}', 10, 0\n", i, str_lit).as_str(),
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
