use crate::lexer::{input::Input, token::Token};

struct Parser<'a> {
    input: Input<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        let mut input = Input::new(s);
        Self {
            current_token: input.next_token(),
            input,
        }
    }

    pub fn peek(&self) -> Token {
        self.current_token.clone()
    }

    pub fn advance(&mut self) -> Token {
        self.current_token = self.input.next_token();
        self.current_token.clone()
    }

    pub fn parse_input(&mut self) -> Result<(), ()> {
        let token = self.peek();

        if token == Token::EOF {
            return Ok(());
        }

        self.advance();

        self.parse_if()
    }

    pub fn parse_if(&mut self) -> Result<(), ()> {
        todo!()
    }
}
