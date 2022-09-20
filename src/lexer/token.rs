#[derive(Clone)]
pub enum Token {
    If,
    Then,
    Else,
    Endif,
    Identifier(String),
    EOF,
}

pub fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric()
}

