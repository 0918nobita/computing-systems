use super::ast::{ExprAst, Identifier, Locatable, StmtAst, Token};
use super::term_color::red_bold;
use once_cell::sync::Lazy;

static SYNTAX_ERROR: Lazy<String> = Lazy::new(|| red_bold("Syntax error:"));

/// 1行分のトークン列を `StmtAst` に変換する再帰下降型パーサ
pub fn parse(tokens: &[Token]) -> Result<StmtAst, String> {
    if let Some(head) = tokens.first() {
        match head {
            Token::Ident(ident) if ident.name == "VAR" => match tokens.get(1) {
                Some(Token::Ident(var_ident)) => match tokens.get(2) {
                    Some(Token::Equal(_)) => {
                        let (expr, rest) = parse_expr(&tokens[3..])?;
                        if rest.is_empty() {
                            Ok(StmtAst::VarDecl(var_ident.clone(), expr))
                        } else {
                            Err(format!(
                                "{} ({}) Unexpected token",
                                SYNTAX_ERROR.as_str(),
                                rest[0].locate()
                            ))
                        }
                    }
                    Some(token) => Err(format!(
                        "{} ({}) Expected `=`",
                        SYNTAX_ERROR.as_str(),
                        token.locate()
                    )),
                    None => Err(format!(
                        "{} ({}) Unexpected end of line",
                        SYNTAX_ERROR.as_str(),
                        var_ident.locate().end
                    )),
                },
                Some(token) => Err(format!(
                    "{} ({}) Unexpected token",
                    SYNTAX_ERROR.as_str(),
                    token.locate()
                )),
                None => Err(format!(
                    "{} ({}) Unexpected end of line",
                    SYNTAX_ERROR.as_str(),
                    ident.locate().end
                )),
            },
            Token::Ident(ident) => {
                // 先頭のトークンが識別子なら、代入文と手続き呼び出しの2通りが想定される
                let (ast, rest) = parse_proc_call(ident, &tokens[1..])
                    .or_else(|_| parse_var_assign(ident, &tokens[1..]))?;
                if rest.is_empty() {
                    Ok(ast)
                } else {
                    Err(format!(
                        "{} ({}) Unexpected token",
                        SYNTAX_ERROR.as_str(),
                        rest[0].locate()
                    ))
                }
            }
            _ => Err(format!(
                "{} ({}) Expected identifier",
                SYNTAX_ERROR.as_str(),
                head.locate()
            )),
        }
    } else {
        Err(format!("{} No tokens found", SYNTAX_ERROR.as_str()))
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
