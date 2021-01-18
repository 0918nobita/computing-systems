use serde::Serialize;
use std::fmt;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct LocationEndpoint {
    pub line: i32,
    pub column: i32,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Location {
    pub start: LocationEndpoint,
    pub end: LocationEndpoint,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}-{}:{}",
            self.start.line, self.start.column, self.end.line, self.end.column
        )
    }
}

pub trait Locatable {
    fn locate(&self) -> Location;
}

#[derive(Clone, Debug, Serialize)]
pub struct Identifier {
    pub name: String,
    pub location: Location,
}

impl Locatable for Identifier {
    fn locate(&self) -> Location {
        self.location
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct StringLiteral {
    pub value: String,
    pub location: Location,
}

impl Locatable for StringLiteral {
    fn locate(&self) -> Location {
        self.location
    }
}

#[derive(Debug)]
pub struct Comma {
    pub loc: LocationEndpoint,
}

impl Locatable for Comma {
    fn locate(&self) -> Location {
        Location {
            start: self.loc,
            end: self.loc,
        }
    }
}

#[derive(Debug)]
pub struct Equal {
    pub loc: LocationEndpoint,
}

impl Locatable for Equal {
    fn locate(&self) -> Location {
        Location {
            start: self.loc,
            end: self.loc,
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Ident(Identifier),
    StrLit(StringLiteral),
    Comma(Comma),
    Equal(Equal),
}

#[derive(Debug, Serialize)]
pub enum ExprAst {
    Ident(Identifier),
    StrLit(StringLiteral),
}

#[derive(Debug, Serialize)]
pub enum StmtAst {
    VarDecl(Identifier, ExprAst),
    ProcCall(Identifier, Vec<ExprAst>),
}
