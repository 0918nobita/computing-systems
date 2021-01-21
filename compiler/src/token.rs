use super::location::{Locatable, Location, Point};
use serde::Serialize;

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
    pub loc: Point,
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
    pub loc: Point,
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
    pub loc: Point,
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
