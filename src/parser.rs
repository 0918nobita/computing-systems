use super::ast::{ExprAst, Identifier, StmtAst, Token};

pub fn parse(tokens: &[Token]) -> Result<StmtAst, String> {
    if let Some(head) = tokens.first() {
        match head {
            Token::Ident(ident) if ident.name == "VAR" => {
                if let Some(Token::Ident(var_ident)) = tokens.get(1) {
                    if let Some(Token::Equal(_)) = tokens.get(2) {
                        let (expr, rest) = parse_expr(&tokens[3..])?;
                        if rest.is_empty() {
                            Ok(StmtAst::VarDecl(var_ident.clone(), expr))
                        } else {
                            Err(format!("Unexpected tokens\n  {:?}", rest))
                        }
                    } else {
                        Err(String::from("Syntax error: Expected equal"))
                    }
                } else {
                    Err(String::from("Syntax error: Expected identifier"))
                }
            }
            Token::Ident(ident) => {
                // 先頭のトークンが識別子なら、代入文と手続き呼び出しの2通りが想定される
                let (ast, rest) = parse_proc_call(ident, tokens)?;
                if rest.is_empty() {
                    Ok(ast)
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

fn parse_proc_call<'a>(
    ident: &Identifier,
    tokens: &'a [Token],
) -> Result<(StmtAst, &'a [Token]), String> {
    let (args, rest) = parse_argument_list(&tokens[1..])?;
    Ok((StmtAst::ProcCall((*ident).clone(), args), rest))
}

fn parse_argument_list(tokens: &[Token]) -> Result<(Vec<ExprAst>, &[Token]), String> {
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
            }
            Err(_) => {
                break;
            }
        }
    }
    Ok((args, remaining_tokens))
}

fn parse_expr(tokens: &[Token]) -> Result<(ExprAst, &[Token]), String> {
    if let Some(head) = tokens.first() {
        match head {
            Token::StrLit(str_lit) => Ok((ExprAst::StrLit(str_lit.clone()), &tokens[1..])),
            Token::Ident(ident) => Ok((ExprAst::Ident(ident.clone()), &tokens[1..])),
            _ => Err(format!("Syntax error: Expected expression\n  {:?}", head)),
        }
    } else {
        Err(String::from("Syntax error: Unexpected end of line"))
    }
}
