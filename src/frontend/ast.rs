#[derive(debug)]
pub enum Literal {
    Bool(bool),
    Int(i64),
    String(String),
    Char(char),
    Null,
}
