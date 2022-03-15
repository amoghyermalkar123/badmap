#[derive(Debug)]
pub enum Token {
    Number(f64),
    Char(char),
    String(String),
    Ident(String),
    EOF,
}
