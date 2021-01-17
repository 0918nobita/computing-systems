use serde::Serialize;

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

trait Locatable {
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
    StrLit(StringLiteral),
}

#[derive(Debug, Serialize)]
pub enum StmtAst {
    VarDecl(Identifier, ExprAst),
    CallProc(Identifier, Vec<ExprAst>),
}
