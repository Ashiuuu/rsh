#[derive(Clone, PartialEq, Eq, Debug)]
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

pub fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}
