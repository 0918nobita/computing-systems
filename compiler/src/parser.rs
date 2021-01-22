use super::ast::{ExprAst, StmtAst};
use super::location::Locatable;
use super::term_color::red_bold;
use super::token::{Identifier, Token};
use once_cell::sync::Lazy;

static SYNTAX_ERROR: Lazy<String> = Lazy::new(|| red_bold("Syntax error:"));

pub fn parse(tokens: &[Token]) -> Result<Vec<StmtAst>, String> {
    let mut tokens = tokens;
    let mut stmts = Vec::<StmtAst>::new();

    loop {
        loop {
            match tokens.first() {
                Some(Token::LineBreak(_)) => {
                    tokens = &tokens[1..];
                    continue;
                }
                Some(_) => {
                    break;
                }
                None => {
                    return Ok(stmts);
                }
            }
        }

        match tokens.first() {
            Some(Token::Ident(ident)) if ident.name == "VAR" => match tokens.get(1) {
                Some(Token::Ident(var_ident)) => match tokens.get(2) {
                    Some(Token::Equal(_)) => {
                        let (expr, rest) = parse_expr(&tokens[3..])?;
                        stmts.push(StmtAst::VarDecl(var_ident.clone(), expr));
                        expect_eol(&rest)?;
                        tokens = rest;
                    }
                    Some(token) => {
                        return Err(format!(
                            "{} ({}) `=` expected but {:?} found",
                            SYNTAX_ERROR.as_str(),
                            token.locate(),
                            token
                        ));
                    }
                    None => {
                        return Err(format!(
                            "{} `=` expected but [EOF] found",
                            SYNTAX_ERROR.as_str()
                        ));
                    }
                },
                Some(Token::LineBreak(line_break)) => {
                    return Err(format!(
                        "{} ({}) Identifier expected but [EOL] found",
                        SYNTAX_ERROR.as_str(),
                        line_break.locate()
                    ));
                }
                Some(token) => {
                    return Err(format!(
                        "{} ({}) Unexpected token",
                        SYNTAX_ERROR.as_str(),
                        token.locate()
                    ));
                }
                None => {
                    return Err(format!("{} Unexpected end of file", SYNTAX_ERROR.as_str()));
                }
            },
            Some(Token::Ident(ident)) => {
                // 先頭のトークンが識別子なら、代入文と手続き呼び出しの2通りが想定される
                let (ast, rest) = parse_proc_call(ident, &tokens[1..])
                    .or_else(|_| parse_var_assign(ident, &tokens[1..]))?;
                stmts.push(ast);
                expect_eol(&rest)?;
                tokens = rest;
            }
            Some(head) => {
                println!("{:?}", head);
                return Err(format!(
                    "{} ({}) Expected identifier",
                    SYNTAX_ERROR.as_str(),
                    head.locate()
                ));
            }
            None => {
                unreachable!();
            }
        }
    }
}

fn expect_eol(tokens: &[Token]) -> Result<(), String> {
    if let Some(head) = tokens.first() {
        match head {
            Token::LineBreak(_) => Ok(()),
            _ => Err(format!(
                "{} ({}) Unexpected token",
                SYNTAX_ERROR.as_str(),
                head.locate()
            )),
        }
    } else {
        Ok(())
    }
}

fn parse_var_assign<'a>(
    ident: &Identifier,
    tokens: &'a [Token],
) -> Result<(StmtAst, &'a [Token]), String> {
    if let Some(head) = tokens.first() {
        match head {
            Token::Equal(_) => {
                let (expr_ast, rest) = parse_expr(&tokens[1..])?;
                Ok((StmtAst::VarAssign(ident.clone(), expr_ast), rest))
            }
            _ => Err(format!(
                "{} ({}) Expected `=`",
                SYNTAX_ERROR.as_str(),
                head.locate()
            )),
        }
    } else {
        Err(format!(
            "{} ({}) Unexpected end of line",
            SYNTAX_ERROR.as_str(),
            ident.locate().end
        ))
    }
}

fn parse_proc_call<'a>(
    ident: &Identifier,
    tokens: &'a [Token],
) -> Result<(StmtAst, &'a [Token]), String> {
    let (args, rest) = parse_argument_list(tokens)?;
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
            _ => Err(format!(
                "{} ({}) Expected expression",
                SYNTAX_ERROR.as_str(),
                head.locate()
            )),
        }
    } else {
        Err(format!("{} Unexpected end of line", SYNTAX_ERROR.as_str()))
    }
}
