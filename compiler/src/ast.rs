use super::token::{Identifier, StringLiteral};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ExprAst {
    Ident(Identifier),
    StrLit(StringLiteral),
}

#[derive(Debug, Serialize)]
pub enum StmtAst {
    VarDecl(Identifier, ExprAst),
    VarAssign(Identifier, ExprAst),
    ProcCall(Identifier, Vec<ExprAst>),
}
