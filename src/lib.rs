use std::cell::RefCell;

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
    CallProc(String, ExprAst),
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
                if let Some(Token::StrLit(str_lit)) = tokens.get(1) {
                    Ok(StmtAst::CallProc(
                        ident.clone(),
                        ExprAst::StrLit(str_lit.clone()),
                    ))
                } else {
                    Err(String::from("Syntax Error"))
                }
            }
            _ => Err(String::from("Syntax Error")),
        }
    } else {
        Err(String::from("Syntax Error"))
    }
}
