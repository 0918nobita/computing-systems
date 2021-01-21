use serde::Serialize;
use std::fmt;

#[derive(Clone, Copy, Serialize)]
pub struct Point {
    line: i32,
    column: i32,
}

impl Point {
    pub fn new(line: i32, column: i32) -> Self {
        Point { line, column }
    }

    fn stringify(&self) -> String {
        format!("{}:{}", self.line + 1, self.column + 1)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.stringify())
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.stringify())
    }
}

#[derive(Clone, Copy, Serialize)]
pub struct Location {
    pub start: Point,
    pub end: Point,
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start.stringify(), self.end.stringify())
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start.stringify(), self.end.stringify())
    }
}

pub trait Locatable {
    fn locate(&self) -> Location;
}
