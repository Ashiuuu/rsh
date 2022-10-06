use lexer::{input::Input, token::Token};

mod ast;
mod lexer;
mod parser;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();

    match args.get(0) {
        Some(s) => match s.as_str() {
            "-c" => {
                let mut input =
                    Input::new(args.get(1).expect("Expected commands in second argument"));

                loop {
                    let token = input.next_token();
                    println!("{:?}", token);
                    if token == Token::EOF {
                        break;
                    }
                }
            }
            _ => todo!(),
        },
        None => todo!(),
    }
}
