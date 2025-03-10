mod css_parser;
mod tests;

use std::fs;
use crate::css_parser::parser::CssParser;
use crate::css_parser::lexer::{Lexer, Token, TokenType};

fn main() {
    let file_path = "src/style.css";
    match fs::read_to_string(file_path) {
        Ok(css) => {
            let mut lexer = Lexer::new(css);

            loop {
                let token = lexer.next_token();
                println!("{:?}", token);
                if token.token_type == TokenType::EOF {
                    break;
                }
            }

            //println!("{:?}", lexer);

            /*let mut parser = CssParser::new(css);

            match parser.parse_stylesheet() {
                Ok(stylesheet) => {
                    println!("Parsed CSS:");
                    println!("{:?}", stylesheet);
                },
                Err(err) => {
                    eprintln!("Error parsing CSS: {}", err);
                }
            }*/
        },
        Err(err) => {
            eprintln!("Failed to read CSS file: {}", err);
        }
    }
}
