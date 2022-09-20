use super::{input::Input, token::{Token, is_identifier_char}};

impl<'a> Input<'a> {
    pub fn next_token(&mut self) {
       match self.peek() {
           c if is_identifier_char(c) => self.tokenize_identifier(),
           _ => todo!(),
       } 
    }

    fn tokenize_identifier(&mut self) {
        todo!()
    }
}

