use serde::Serialize;
use std::fmt;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct LocationEndpoint {
    pub line: i32,
    pub column: i32,
}

impl fmt::Display for LocationEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.column + 1)
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Location {
    pub start: LocationEndpoint,
    pub end: LocationEndpoint,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
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
pub struct LineBreak {
    pub loc: LocationEndpoint,
}

impl Locatable for LineBreak {
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
    LineBreak(LineBreak),
}

impl Locatable for Token {
    fn locate(&self) -> Location {
        match self {
            Token::Ident(ident) => ident.locate(),
            Token::StrLit(str_lit) => str_lit.locate(),
            Token::Comma(comma) => comma.locate(),
            Token::Equal(equal) => equal.locate(),
            Token::LineBreak(line_break) => line_break.locate(),
        }
    }
}

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
