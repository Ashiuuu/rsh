use std::str::Chars;

pub const EOF: char = '\0';

pub struct Input<'a> {
    chars: Chars<'a>,
}

impl<'a> Input<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars()
        }
    }

    pub fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF)
    }

    pub fn advance(&mut self) -> char {
        self.chars.next().unwrap_or(EOF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_test() {
        let input = Input::new("12345");

        assert_eq!(input.peek(), '1');
    }

    #[test]
    fn advance_test() {
        let mut input = Input::new("12345");

        assert_eq!(input.advance(), '1');
        assert_eq!(input.peek(), '2');
    }

    #[test]
    fn eof_test() {
        let mut input = Input::new("1");

        assert_eq!(input.advance(), '1');
        assert_eq!(input.advance(), EOF);
        assert_eq!(input.advance(), EOF);
        assert_eq!(input.peek(), EOF);
    }
}
