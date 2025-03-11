use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Punctuation
    OpenBrace,       // {
    CloseBrace,      // }
    OpenParen,       // (
    CloseParen,      // )
    OpenBracket,     // [
    CloseBracket,    // ]
    Semicolon,       // ;
    Colon,           // :
    Comma,           // ,
    Hash,            // #
    Dot,             // .
    Asterisk,        // *

    // Math operators
    Plus,            // +
    Minus,           // -
    Slash,           // /

    // CSS Selectors
    GreaterThan,     // > (child selector)
    Tilde,           // ~ (general sibling)
    // Plus is already defined above (adjacent sibling)

    // At-rules
    AtSymbol,        // @

    // Literals
    Identifier(String),      // property names, element names, etc.
    Number(f64),             // numeric values (without unit)
    String(String),          // quoted strings

    // Special CSS values
    Unit(String),            // px, em, %, etc.
    HexColor(String),        // #fff, #123456

    // End of file
    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::OpenBrace => write!(f, "{{"),
            TokenType::CloseBrace => write!(f, "}}"),
            TokenType::OpenParen => write!(f, "("),
            TokenType::CloseParen => write!(f, ")"),
            TokenType::OpenBracket => write!(f, "["),
            TokenType::CloseBracket => write!(f, "]"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Comma => write!(f, ","),
            TokenType::Hash => write!(f, "#"),
            TokenType::Dot => write!(f, "."),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Slash => write!(f, "/"),
            TokenType::GreaterThan => write!(f, ">"),
            TokenType::Tilde => write!(f, "~"),
            TokenType::AtSymbol => write!(f, "@"),
            TokenType::Identifier(val) => write!(f, "Identifier({})", val),
            TokenType::Number(val) => write!(f, "Number({})", val),
            TokenType::String(val) => write!(f, "String(\"{}\")", val),
            TokenType::Unit(val) => write!(f, "Unit({})", val),
            TokenType::HexColor(val) => write!(f, "HexColor(#{})", val),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize, length: usize) -> Self {
        Token {
            token_type,
            line,
            column,
            length,
        }
    }
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
    line: usize,
    column: usize,
    next_token_cache: Option<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
            line: 1,
            column: 0,
            next_token_cache: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input.chars().nth(self.read_position).unwrap());
        }

        self.position = self.read_position;
        self.read_position += 1;

        if let Some(ch) = self.ch {
            if ch == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input.chars().nth(self.read_position).unwrap())
        }
    }

    pub fn next_token(&mut self) -> Token {
        if let Some(token) = self.next_token_cache.take() {
            return token;
        }

        self.skip_whitespace();

        if self.ch.is_none() {
            return Token::new(TokenType::EOF, self.line, self.column, 0);
        }

        let ch = self.ch.unwrap();

        match ch {
            '{' => {
                let token = Token::new(TokenType::OpenBrace, self.line, self.column, 1);
                self.read_char();
                token
            },
            '}' => {
                let token = Token::new(TokenType::CloseBrace, self.line, self.column, 1);
                self.read_char();
                token
            },
            '(' => {
                let token = Token::new(TokenType::OpenParen, self.line, self.column, 1);
                self.read_char();
                token
            },
            ')' => {
                let token = Token::new(TokenType::CloseParen, self.line, self.column, 1);
                self.read_char();
                token
            },
            '[' => {
                let token = Token::new(TokenType::OpenBracket, self.line, self.column, 1);
                self.read_char();
                token
            },
            ']' => {
                let token = Token::new(TokenType::CloseBracket, self.line, self.column, 1);
                self.read_char();
                token
            },
            ';' => {
                let token = Token::new(TokenType::Semicolon, self.line, self.column, 1);
                self.read_char();
                token
            },
            ':' => {
                let token = Token::new(TokenType::Colon, self.line, self.column, 1);
                self.read_char();
                token
            },
            ',' => {
                let token = Token::new(TokenType::Comma, self.line, self.column, 1);
                self.read_char();
                token
            },
            '.' => {
                let token = Token::new(TokenType::Dot, self.line, self.column, 1);
                self.read_char();
                token
            },
            '*' => {
                let token = Token::new(TokenType::Asterisk, self.line, self.column, 1);
                self.read_char();
                token
            },
            '+' => {
                let peek = self.peek_char();
                if peek.is_some() && peek.unwrap().is_digit(10) {
                    let start_col = self.column;
                    let (number, length) = self.read_number();

                    if self.ch.is_some() && (self.ch.unwrap().is_alphabetic() || self.ch.unwrap() == '%') {
                        let unit_start_col = self.column;
                        let unit = self.read_unit();

                        self.next_token_cache = Some(Token::new(TokenType::Unit(unit.clone()), self.line, unit_start_col, unit.len()));

                        return Token::new(TokenType::Number(number), self.line, start_col, length);
                    }

                    Token::new(TokenType::Number(number), self.line, start_col, length)
                } else {
                    let token = Token::new(TokenType::Plus, self.line, self.column, 1);
                    self.read_char();
                    token
                }
            },
            '/' => {
                if self.peek_char() == Some('*') {
                    self.skip_block_comment();
                    self.next_token()
                } else if self.peek_char() == Some('/') {
                    self.skip_line_comment();
                    self.next_token()
                } else {
                    let token = Token::new(TokenType::Slash, self.line, self.column, 1);
                    self.read_char();
                    token
                }
            },
            '#' => {
                let start_col = self.column;
                self.read_char();

                if self.ch.is_some() && self.is_hex_digit(self.ch.unwrap()) {
                    let hex_color = self.read_hex_color();
                    Token::new(TokenType::HexColor(hex_color.clone()), self.line, start_col, hex_color.len() + 1) // +1 for #
                } else {
                    Token::new(TokenType::Hash, self.line, start_col, 1)
                }
            },
            '"' | '\'' => {
                let quote_char = ch;
                let start_col = self.column;
                self.read_char();

                let string = self.read_string(quote_char);
                let length = string.len() + 2;
                self.read_char();
                Token::new(TokenType::String(string), self.line, start_col, length)
            },
            '0'..='9' | '-' => {
                let start_col = self.column;

                if ch == '-' {
                    let peek = self.peek_char();
                    if peek.is_none() || !self.is_digit_or_decimal(peek.unwrap()) {
                        let identifier = self.read_identifier();
                        return Token::new(TokenType::Identifier(identifier.clone()), self.line, start_col, identifier.len());
                    }
                }

                let (number, length) = self.read_number();

                if self.ch.is_some() && (self.ch.unwrap().is_alphabetic() || self.ch.unwrap() == '%') {
                    let unit_start_col = self.column;
                    let unit = self.read_unit();

                    self.next_token_cache = Some(Token::new(TokenType::Unit(unit.clone()), self.line, unit_start_col, unit.len()));

                    return Token::new(TokenType::Number(number), self.line, start_col, length);
                }

                Token::new(TokenType::Number(number), self.line, start_col, length)
            },
            c if self.is_identifier_start(c) => {
                let start_col = self.column;
                let identifier = self.read_identifier();
                Token::new(TokenType::Identifier(identifier.clone()), self.line, start_col, identifier.len())
            },
            _ => {
                self.read_char();
                self.next_token()
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_some() {
            let ch = self.ch.unwrap();
            if ch.is_whitespace() {
                self.read_char();
            } else if ch == '/' && self.peek_char() == Some('*') {
                self.skip_block_comment();
            } else if ch == '/' && self.peek_char() == Some('/') {
                self.skip_line_comment();
            } else {
                break;
            }
        }
    }

    fn skip_block_comment(&mut self) {
        self.read_char();
        self.read_char();

        while self.ch.is_some() {
            if self.ch == Some('*') && self.peek_char() == Some('/') {
                self.read_char();
                self.read_char();
                break;
            }
            self.read_char();
        }
    }

    fn skip_line_comment(&mut self) {
        while self.ch.is_some() {
            let ch = self.ch.unwrap();
            self.read_char();
            if ch == '\n' {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let start_position = self.position;

        if self.ch.is_some() {
            self.read_char();
        }

        while self.ch.is_some() && self.is_identifier_part(self.ch.unwrap()) {
            self.read_char();
        }

        self.input[start_position..self.position].to_string()
    }

    fn read_number(&mut self) -> (f64, usize) {
        let start_position = self.position;

        if self.ch == Some('-') || self.ch == Some('+') {
            self.read_char();
        }

        while self.ch.is_some() && self.ch.unwrap().is_digit(10) {
            self.read_char();
        }

        if self.ch == Some('.') {
            self.read_char();

            while self.ch.is_some() && self.ch.unwrap().is_digit(10) {
                self.read_char();
            }
        }

        let number_str = &self.input[start_position..self.position];
        let number = number_str.parse::<f64>().unwrap_or_else(|_| 0.0);

        (number, self.position - start_position)
    }

    fn read_unit(&mut self) -> String {
        let start_position = self.position;

        while self.ch.is_some() {
            let ch = self.ch.unwrap();
            if ch.is_alphabetic() || ch == '%' {
                self.read_char();
            } else {
                break;
            }
        }

        self.input[start_position..self.position].to_string()
    }

    fn read_hex_color(&mut self) -> String {
        let start_position = self.position;

        while self.ch.is_some() && self.is_hex_digit(self.ch.unwrap()) {
            self.read_char();
        }

        self.input[start_position..self.position].to_string()
    }

    fn read_string(&mut self, quote_char: char) -> String {
        let start_position = self.position;
        let mut escaped = false;

        while self.ch.is_some() {
            let ch = self.ch.unwrap();

            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote_char {
                break;
            }

            self.read_char();
        }

        self.input[start_position..self.position].to_string()
    }

    fn is_identifier_start(&self, ch: char) -> bool {
        ch.is_alphabetic() || ch == '_' || ch == '-'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_' || ch == '-'
    }

    fn is_hex_digit(&self, ch: char) -> bool {
        ch.is_digit(16)
    }

    fn is_digit_or_decimal(&self, ch: char) -> bool {
        ch.is_digit(10) || ch == '.'
    }
}
