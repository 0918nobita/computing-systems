use super::location::{Locatable, Location};
use super::token::{Identifier, StringLiteral};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ExprAst {
    Ident(Identifier),
    StrLit(StringLiteral),
}

impl Locatable for ExprAst {
    fn locate(&self) -> Location {
        match self {
            ExprAst::Ident(ident) => ident.locate(),
            ExprAst::StrLit(str_lit) => str_lit.locate(),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum StmtAst {
    VarDecl(Identifier, ExprAst),
    VarAssign(Identifier, ExprAst),
    ProcCall(Identifier, Vec<ExprAst>),
}
