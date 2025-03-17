mod css_parser;
mod tests;

use std::fs;
use crate::css_parser::lexer::{Lexer, TokenType};
use crate::css_parser::parser::CssParser;

fn main() {
    let file_path = "src/style.css";
    match fs::read_to_string(file_path) {
        Ok(css) => {
            let mut parser = CssParser::new(css.clone());
            let mut lexer = Lexer::new(css);

            /*loop {
                let token = lexer.next_token();
                println!("{:?}", token);

                if token.token_type == TokenType::EOF {
                    break;
                }
            }*/

            match parser.parse_stylesheet() {
                Ok(stylesheet) => {
                    println!("Parsed CSS:");
                    println!("Regular rules: {}", stylesheet.rules.len());
                    for rule in &stylesheet.rules {
                        println!("{:?}", rule);
                    }

                    println!("At-rules: {}", stylesheet.at_rules.len());
                    for at_rule in &stylesheet.at_rules {
                        println!("{}", at_rule);
                    }
                },
                Err(err) => {
                    eprintln!("Error parsing CSS: {}", err);
                }
            }
        },
        Err(err) => {
            eprintln!("Failed to read CSS file: {}", err);
        }
    }
}
