#[derive(Debug)]
pub enum Token {
    Ident(String),
    StrLit(String),
}
