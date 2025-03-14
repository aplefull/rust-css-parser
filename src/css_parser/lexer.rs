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
    DoubleColon,     // ::
    Comma,           // ,
    Hash,            // #
    Dot,             // .
    Asterisk,        // *
    ExclamationMark, // !
    Backslash,       // \
    Whitespace,      // Significant whitespace

    // Math operators
    Plus,            // +
    Minus,           // -
    Slash,           // /

    // CSS Selectors
    GreaterThan,     // > (child selector)
    Tilde,           // ~ (general sibling)
    // Plus is already defined above (adjacent sibling)

    // Comparison operators
    Equals,          // =
    Caret,           // ^
    Dollar,          // $
    Pipe,            // |

    // At-rules
    AtSymbol,        // @

    // Literals
    Identifier(String),      // property names, element names, etc.
    Number(f64),             // numeric values (without unit)
    String(String),          // quoted strings

    // Special CSS values
    Unit(String),            // px, em, %, etc.
    UnicodeRange(String),    // U+XXXX

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
            TokenType::DoubleColon => write!(f, "::"),
            TokenType::Comma => write!(f, ","),
            TokenType::Hash => write!(f, "#"),
            TokenType::Dot => write!(f, "."),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::ExclamationMark => write!(f, "!"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Backslash => write!(f, "\\"),
            TokenType::Whitespace => write!(f, "Whitespace"),
            TokenType::GreaterThan => write!(f, ">"),
            TokenType::Tilde => write!(f, "~"),
            TokenType::Equals => write!(f, "="),
            TokenType::Caret => write!(f, "^"),
            TokenType::Dollar => write!(f, "$"),
            TokenType::Pipe => write!(f, "|"),
            TokenType::AtSymbol => write!(f, "@"),
            TokenType::Identifier(val) => write!(f, "Identifier({})", val),
            TokenType::Number(val) => write!(f, "Number({})", val),
            TokenType::String(val) => write!(f, "String(\"{}\")", val),
            TokenType::Unit(val) => write!(f, "Unit({})", val),
            TokenType::UnicodeRange(val) => write!(f, "UnicodeRange({})", val),
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

#[derive(Debug, Clone, PartialEq)]
pub enum LexerMode {
    Normal,     // Skip whitespace (default)
    Selector,   // Return whitespace tokens (for parsing selectors)
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
    line: usize,
    column: usize,
    next_token_cache: Vec<Token>,
    pub mode: LexerMode,
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
            next_token_cache: Vec::new(),
            mode: LexerMode::Normal,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            let mut chars = self.input[self.read_position..].chars();
            let current_char = chars.next();

            self.ch = current_char;

            self.position = self.read_position;
            if let Some(ch) = current_char {
                self.read_position += ch.len_utf8();
            } else {
                self.read_position += 1;
            }
        }

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
            self.input[self.read_position..].chars().next()
        }
    }

    pub fn next_token(&mut self) -> Token {
        if !self.next_token_cache.is_empty() {
            return self.next_token_cache.remove(0);
        }

        if matches!(self.mode, LexerMode::Selector) {
            if self.ch.is_some() && self.ch.unwrap().is_whitespace() {
                let start_line = self.line;
                let start_column = self.column;

                let start_pos = self.position;
                while self.ch.is_some() && self.ch.unwrap().is_whitespace() {
                    self.read_char();
                }

                let length = self.position - start_pos;
                return Token::new(TokenType::Whitespace, start_line, start_column, length);
            }
        } else {
            self.skip_whitespace();
        }

        if self.ch.is_none() {
            return Token::new(TokenType::EOF, self.line, self.column, 0);
        }

        let ch = self.ch.unwrap();

        if (ch.is_alphabetic() || ch == '_' || ch == '-') &&
            (ch == 'u' || ch == 'U') &&
            self.position + 3 <= self.input.len() &&
            self.input[self.position..self.position+3].to_lowercase() == "url" {

            let peek_pos = self.position + 3;
            let mut peek_index = peek_pos;

            while peek_index < self.input.len() &&
                self.input[peek_index..].chars().next().unwrap().is_whitespace() {
                peek_index += 1;
            }

            if peek_index < self.input.len() &&
                self.input[peek_index..].chars().next().unwrap() == '(' {
                return self.handle_url_function();
            }
        }

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
                if self.peek_char() == Some(':') {
                    let token = Token::new(TokenType::DoubleColon, self.line, self.column, 2);
                    self.read_char();
                    self.read_char();
                    token
                } else {
                    let token = Token::new(TokenType::Colon, self.line, self.column, 1);
                    self.read_char();
                    token
                }
            },
            ',' => {
                let token = Token::new(TokenType::Comma, self.line, self.column, 1);
                self.read_char();
                token
            },
            '>' => {
                let token = Token::new(TokenType::GreaterThan, self.line, self.column, 1);
                self.read_char();
                token
            },
            '~' => {
                let token = Token::new(TokenType::Tilde, self.line, self.column, 1);
                self.read_char();
                token
            },
            '=' => {
                let token = Token::new(TokenType::Equals, self.line, self.column, 1);
                self.read_char();
                token
            },
            '^' => {
                let token = Token::new(TokenType::Caret, self.line, self.column, 1);
                self.read_char();
                token
            },
            '$' => {
                let token = Token::new(TokenType::Dollar, self.line, self.column, 1);
                self.read_char();
                token
            },
            '|' => {
                let token = Token::new(TokenType::Pipe, self.line, self.column, 1);
                self.read_char();
                token
            },
            '@' => {
                let token = Token::new(TokenType::AtSymbol, self.line, self.column, 1);
                self.read_char();
                token
            },
            '.' => {
                if let Some(next_ch) = self.peek_char() {
                    if next_ch.is_digit(10) {
                        let start_col = self.column;
                        self.read_char();

                        let mut number_str = String::from("0.");

                        while self.ch.is_some() && self.ch.unwrap().is_digit(10) {
                            number_str.push(self.ch.unwrap());
                            self.read_char();
                        }

                        let number = number_str.parse::<f64>().unwrap_or(0.0);
                        let length = number_str.len() - 1;

                        if self.ch.is_some() && (self.ch.unwrap().is_alphabetic() || self.ch.unwrap() == '%') {
                            let unit_start_col = self.column;
                            let unit = self.read_unit();

                            self.next_token_cache.push(Token::new(TokenType::Unit(unit.clone()),
                                                                  self.line,
                                                                  unit_start_col,
                                                                  unit.len()));

                            return Token::new(TokenType::Number(number), self.line, start_col, length);
                        }

                        return Token::new(TokenType::Number(number), self.line, start_col, length);
                    }
                }

                let token = Token::new(TokenType::Dot, self.line, self.column, 1);
                self.read_char();
                token
            },
            '*' => {
                let token = Token::new(TokenType::Asterisk, self.line, self.column, 1);
                self.read_char();
                token
            },
            '!' => {
                let token = Token::new(TokenType::ExclamationMark, self.line, self.column, 1);
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

                        self.next_token_cache.push(Token::new(TokenType::Unit(unit.clone()), self.line, unit_start_col, unit.len()));

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
            '\\' => {
                let start_col = self.column;
                self.read_char();

                if self.ch.is_none() {
                    return Token::new(TokenType::Backslash, self.line, start_col, 1);
                }

                match self.ch.unwrap() {
                    '[' => {
                        let token = Token::new(TokenType::Identifier("\\[".to_string()),
                                               self.line, start_col, 2);
                        self.read_char();
                        token
                    },
                    ']' => {
                        let token = Token::new(TokenType::Identifier("\\]".to_string()),
                                               self.line, start_col, 2);
                        self.read_char();
                        token
                    },
                    '!' => {
                        let token = Token::new(TokenType::Identifier("\\!".to_string()),
                                               self.line, start_col, 2);
                        self.read_char();
                        token
                    },
                    '.' => {
                        let token = Token::new(TokenType::Identifier("\\.".to_string()),
                                               self.line, start_col, 2);
                        self.read_char();
                        token
                    },
                    ':' => {
                        let token = Token::new(TokenType::Identifier("\\:".to_string()),
                                               self.line, start_col, 2);
                        self.read_char();
                        token
                    },
                    _ => {
                        Token::new(TokenType::Backslash, self.line, start_col, 1)
                    }
                }
            },
            '#' => {
                let hash_token = Token::new(TokenType::Hash, self.line, self.column, 1);
                self.read_char();

                if self.ch.is_some() && self.is_hex_digit(self.ch.unwrap()) {
                    let hex_start_col = self.column;
                    let hex_start_position = self.position;

                    while self.ch.is_some() && self.is_hex_digit(self.ch.unwrap()) {
                        self.read_char();
                    }

                    let hex_value = self.input[hex_start_position..self.position].to_string();
                    let hex_length = hex_value.len();

                    self.next_token_cache.push(Token::new(
                        TokenType::Identifier(hex_value),
                        self.line,
                        hex_start_col,
                        hex_length
                    ));
                }

                hash_token
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
            'U' => {
                if self.peek_char() == Some('+') {
                    self.read_unicode_range()
                } else {
                    let start_col = self.column;
                    let identifier = self.read_identifier();
                    Token::new(TokenType::Identifier(identifier.clone()),
                                      self.line, start_col, identifier.len())
                }
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

                    self.next_token_cache.push(Token::new(TokenType::Unit(unit.clone()), self.line, unit_start_col, unit.len()));

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

    fn handle_url_function(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;

        let url_identifier = self.read_identifier();
        assert_eq!(url_identifier, "url");

        self.skip_whitespace();

        if self.ch != Some('(') {
            return Token::new(TokenType::Identifier(url_identifier.clone()),
                              start_line, start_column, url_identifier.len());
        }

        self.read_char();

        self.skip_whitespace();

        let url_content_start = self.position;
        let mut paren_depth = 1;
        let mut in_quotes = false;
        let mut quote_char = ' ';

        while self.ch.is_some() {
            let ch = self.ch.unwrap();

            if !in_quotes {
                if ch == '(' {
                    paren_depth += 1;
                } else if ch == ')' {
                    paren_depth -= 1;
                    if paren_depth == 0 {
                        break;
                    }
                } else if ch == '"' || ch == '\'' {
                    in_quotes = true;
                    quote_char = ch;
                }
            } else if ch == quote_char && self.peek_char() != Some('\\') {
                in_quotes = false;
            }

            self.read_char();
        }

        let url_content = self.input[url_content_start..self.position].to_string();

        if self.ch == Some(')') {
            self.read_char();
        }

        self.next_token_cache.push(Token::new(
            TokenType::OpenParen,
            start_line, start_column + url_identifier.len(),
            1
        ));

        if !url_content.is_empty() {
            self.next_token_cache.push(Token::new(
                TokenType::String(url_content.clone()),
                start_line, start_column + url_identifier.len() + 1,
                url_content.len()
            ));
        }

        self.next_token_cache.push(Token::new(
            TokenType::CloseParen,
            self.line, self.column - 1,
            1
        ));

        Token::new(
            TokenType::Identifier(url_identifier.clone()),
            start_line, start_column,
            url_identifier.len()
        )
    }

    fn read_unicode_range(&mut self) -> Token {
        let start_position = self.position;
        let start_column = self.column;
        let start_line = self.line;

        self.read_char();

        if self.ch != Some('+') {
            return Token::new(TokenType::Identifier("U".to_string()),
                              start_line, start_column, 1);
        }

        self.read_char();

        while self.ch.is_some() {
            let ch = self.ch.unwrap();

            if ch.is_digit(16) || ch == '-' || ch == '?' {
                self.read_char();
            } else {
                break;
            }
        }

        let unicode_range = self.input[start_position..self.position].to_string();
        Token::new(TokenType::UnicodeRange(unicode_range.clone()),
                   start_line, start_column, unicode_range.len())
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

    fn read_escape(&mut self) -> Option<char> {
        self.read_char();

        if self.ch.is_none() {
            return None;
        }

        let ch = self.ch.unwrap();

        if ch.is_digit(16) {
            let start_position = self.position;

            let mut count = 0;
            while self.ch.is_some() && self.ch.unwrap().is_digit(16) && count < 6 {
                self.read_char();
                count += 1;
            }

            if self.ch == Some(' ') {
                self.read_char();
            }

            let hex_str = &self.input[start_position..self.position - if self.ch == Some(' ') { 1 } else { 0 }];

            match u32::from_str_radix(hex_str, 16) {
                Ok(code) => {
                    match std::char::from_u32(code) {
                        Some(c) => return Some(c),
                        None => return None
                    }
                },
                Err(_) => return None
            }
        } else {
            let escaped_char = ch;
            self.read_char();
            return Some(escaped_char);
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut result = String::new();

        if self.ch.is_some() {
            if self.ch == Some('\\') {
                if let Some(escaped_char) = self.read_escape() {
                    result.push(escaped_char);
                }
            } else {
                result.push(self.ch.unwrap());
                self.read_char();
            }
        }

        while self.ch.is_some() {
            let ch = self.ch.unwrap();

            if ch == '\\' {
                if let Some(escaped_char) = self.read_escape() {
                    result.push(escaped_char);
                }
            } else if self.is_identifier_part(ch) {
                result.push(ch);
                self.read_char();
            } else {
                break;
            }
        }

        result
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

    fn is_identifier_part(&self, ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_' || ch == '-' || ch == '\\' ||
            ch == '!' || ch == '#' || ch == '@' || ch == '$' || ch == '%' ||
            ch == '&' || ch == '*' || ch == '~' || ch == '.' ||
            ch > '\u{7F}'
    }

    fn is_identifier_start(&self, ch: char) -> bool {
        ch.is_alphabetic() || ch == '_' || ch == '-' || ch == '\\' ||
            ch == '!' || ch == '#' || ch == '@' || ch == '$' || ch == '%' ||
            ch == '&' || ch == '*' || ch == '~' ||
            ch > '\u{7F}'
    }

    fn is_hex_digit(&self, ch: char) -> bool {
        ch.is_digit(16)
    }

    fn is_digit_or_decimal(&self, ch: char) -> bool {
        ch.is_digit(10) || ch == '.'
    }
}
