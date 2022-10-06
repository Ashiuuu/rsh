use super::{
    input::{Input, EOF},
    token::{is_identifier_char, is_whitespace, Token},
};

impl<'a> Input<'a> {
    pub fn next_token(&mut self) -> Token {
        self.advance_while(is_whitespace);

        match self.peek() {
            EOF => Token::EOF,
            c if is_identifier_char(c) => self.tokenize_identifier(),
            _ => todo!(),
        }
    }

    fn tokenize_identifier(&mut self) -> Token {
        let ident = self.take_while(is_identifier_char);
        match ident.as_ref() {
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            "endif" => Token::Endif,
            _ => Token::Identifier(ident),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_identifier() {
        let mut input = Input::new("test");

        assert_eq!(input.next_token(), Token::Identifier("test".to_string()))
    }

    #[test]
    fn multiple_tokens() {
        let mut input = Input::new("first second");

        assert_eq!(input.next_token(), Token::Identifier("first".to_string()));
        assert_eq!(input.next_token(), Token::Identifier("second".to_string()))
    }

    #[test]
    fn eof_token() {
        let mut input = Input::new("");

        assert_eq!(input.next_token(), Token::EOF)
    }

    #[test]
    fn condition_tokens() {
        let mut input = Input::new("if else  then   endif");

        assert_eq!(input.next_token(), Token::If)
    }
}
