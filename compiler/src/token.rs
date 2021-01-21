use super::location::{Locatable, Location, Point};
use serde::Serialize;
use std::fmt;

#[derive(Clone, Serialize)]
pub struct Identifier {
    pub name: String,
    pub location: Location,
}

impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{:?}", self.name, self.location)
    }
}

impl Locatable for Identifier {
    fn locate(&self) -> Location {
        self.location
    }
}

#[derive(Clone, Serialize)]
pub struct StringLiteral {
    pub value: String,
    pub location: Location,
}

impl fmt::Debug for StringLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"@{:?}", self.value, self.location)
    }
}

impl Locatable for StringLiteral {
    fn locate(&self) -> Location {
        self.location
    }
}

pub struct Comma {
    pub loc: Point,
}

impl fmt::Debug for Comma {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`,`@{:?}", self.loc)
    }
}

impl Locatable for Comma {
    fn locate(&self) -> Location {
        Location {
            start: self.loc,
            end: self.loc,
        }
    }
}

pub struct Equal {
    pub loc: Point,
}

impl fmt::Debug for Equal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`=`@{:?}", self.loc)
    }
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
