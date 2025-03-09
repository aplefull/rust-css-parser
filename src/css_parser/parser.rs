use crate::css_parser::ast::*;

fn is_color_name(name: &str) -> bool {
    let color_names = [
        "transparent", "currentColor", "black", "silver", "gray", "white",
        "maroon", "red", "purple", "fuchsia", "green", "lime", "olive",
        "yellow", "navy", "blue", "teal", "aqua", "orange", "aliceblue",
        "antiquewhite", "aquamarine", "azure", "beige", "bisque", "blanchedalmond",
        "blueviolet", "brown", "burlywood", "cadetblue", "chartreuse", "chocolate",
        "coral", "cornflowerblue", "cornsilk", "crimson", "cyan", "darkblue",
        "darkcyan", "darkgoldenrod", "darkgray", "darkgreen", "darkgrey", "darkkhaki",
        "darkmagenta", "darkolivegreen", "darkorange", "darkorchid", "darkred",
        "darksalmon", "darkseagreen", "darkslateblue", "darkslategray", "darkslategrey",
        "darkturquoise", "darkviolet", "deeppink", "deepskyblue", "dimgray", "dimgrey",
        "dodgerblue", "firebrick", "floralwhite", "forestgreen", "gainsboro",
        "ghostwhite", "gold", "goldenrod", "greenyellow", "grey", "honeydew",
        "hotpink", "indianred", "indigo", "ivory", "khaki", "lavender", "lavenderblush",
        "lawngreen", "lemonchiffon", "lightblue", "lightcoral", "lightcyan",
        "lightgoldenrodyellow", "lightgray", "lightgreen", "lightgrey", "lightpink",
        "lightsalmon", "lightseagreen", "lightskyblue", "lightslategray", "lightslategrey",
        "lightsteelblue", "lightyellow", "limegreen", "linen", "magenta", "mediumaquamarine",
        "mediumblue", "mediumorchid", "mediumpurple", "mediumseagreen", "mediumslateblue",
        "mediumspringgreen", "mediumturquoise", "mediumvioletred", "midnightblue",
        "mintcream", "mistyrose", "moccasin", "navajowhite", "oldlace", "olivedrab",
        "orangered", "orchid", "palegoldenrod", "palegreen", "paleturquoise",
        "palevioletred", "papayawhip", "peachpuff", "peru", "pink", "plum", "powderblue",
        "rosybrown", "royalblue", "saddlebrown", "salmon", "sandybrown", "seagreen",
        "seashell", "sienna", "skyblue", "slateblue", "slategray", "slategrey", "snow",
        "springgreen", "steelblue", "tan", "thistle", "tomato", "turquoise", "violet",
        "wheat", "whitesmoke", "yellowgreen"
    ];

    color_names.contains(&name.to_lowercase().as_str())
}

fn is_css_keyword(keyword: &str) -> bool {
    let keywords = [
        "inherit", "initial", "unset", "revert", "auto", "none", "normal", "bold",
        "italic", "oblique", "underline", "overline", "line-through", "blink",
        "solid", "dotted", "dashed", "double", "groove", "ridge", "inset", "outset",
        "baseline", "sub", "super", "top", "middle", "bottom", "text-top", "text-bottom",
        "capitalize", "uppercase", "lowercase", "hidden", "visible", "collapse",
        "static", "relative", "absolute", "fixed", "sticky", "block", "inline",
        "inline-block", "flex", "inline-flex", "grid", "inline-grid", "table",
        "table-row", "table-cell", "list-item", "content-box", "border-box",
        "nowrap", "pre", "pre-wrap", "pre-line", "break-spaces", "row", "column",
        "row-reverse", "column-reverse", "wrap", "wrap-reverse", "start", "end",
        "center", "stretch", "space-between", "space-around", "space-evenly"
    ];

    keywords.contains(&keyword.to_lowercase().as_str())
}

pub struct CssParser {
    input: String,
    position: usize,
}

impl CssParser {
    pub fn new(input: String) -> Self {
        CssParser {
            input,
            position: 0,
        }
    }

    fn get_context(&self, pos: usize, context_chars: usize) -> String {
        let start = if pos > context_chars { pos - context_chars } else { 0 };
        let end = if pos + context_chars < self.input.len() { pos + context_chars } else { self.input.len() };

        let prefix = &self.input[start..pos];
        let suffix = &self.input[pos..end];

        format!("...{}[HERE->]{}...", prefix, suffix)
    }


    pub fn parse_stylesheet(&mut self) -> Result<Stylesheet, String> {
        let mut stylesheet = Stylesheet { rules: Vec::new() };

        let start_time = std::time::Instant::now();

        self.consume_whitespace();

        while self.position < self.input.len() {
            let rule = self.parse_rule()?;
            stylesheet.rules.push(rule);
            self.consume_whitespace();
        }

        let elapsed = start_time.elapsed();

        println!("Parsed {} rules in {:?}", stylesheet.rules.len(), elapsed);

        Ok(stylesheet)
    }

    fn parse_rule(&mut self) -> Result<Rule, String> {
        let selector = self.parse_selector()?;
        self.consume_whitespace();

        self.expect_character('{')?;
        self.consume_whitespace();

        let declarations = self.parse_declarations()?;

        self.expect_character('}')?;
        self.consume_whitespace();

        Ok(Rule {
            selector,
            declarations,
        })
    }

    fn parse_selector(&mut self) -> Result<Selector, String> {
        if self.position >= self.input.len() {
            return Err("Unexpected end of input while parsing selector".to_string());
        }

        let mut parts = Vec::new();
        let mut first_part = true;

        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();

            if current_char.is_whitespace() || current_char == '{' {
                break;
            }

            let part = self.parse_selector_part(first_part)?;
            parts.push(part);
            first_part = false;
        }

        if parts.is_empty() {
            return Err("Expected at least one selector part".to_string());
        }

        Ok(Selector { parts })
    }

    fn parse_selector_part(&mut self, allow_element: bool) -> Result<SelectorPart, String> {
        if self.position >= self.input.len() {
            return Err("Unexpected end of input while parsing selector part".to_string());
        }

        let current_char = self.input.chars().nth(self.position).unwrap();

        match current_char {
            '.' => {
                self.position += 1;
                let name = self.parse_identifier()?;
                Ok(SelectorPart::Class(name))
            },
            '#' => {
                self.position += 1;
                let name = self.parse_identifier()?;
                Ok(SelectorPart::Id(name))
            },
            ':' => {
                self.position += 1;
                let name = self.parse_identifier()?;
                Ok(SelectorPart::PseudoElement(name))
            },
            '*' => {
                self.position += 1;
                Ok(SelectorPart::Universal)
            },
            _ if allow_element => {
                let name = self.parse_identifier()?;
                Ok(SelectorPart::Element(name))
            },
            _ => Err(format!("Unexpected character in selector: {}", current_char)),
        }
    }

    fn parse_declarations(&mut self) -> Result<Vec<Declaration>, String> {
        let mut declarations = Vec::new();

        loop {
            self.consume_whitespace();

            if self.position >= self.input.len() {
                return Err("Unexpected end of input while parsing declarations".to_string());
            }

            if self.input.chars().nth(self.position).unwrap() == '}' {
                break;
            }

            let declaration_start = self.position;

            let declaration = match self.parse_declaration() {
                Ok(decl) => decl,
                Err(e) => {
                    return Err(format!("{} near {}", e, self.get_context(self.position, 20)));
                }
            };
            declarations.push(declaration);

            self.consume_whitespace();

            if self.position < self.input.len() {
                let c = self.input.chars().nth(self.position).unwrap();

                if c == ';' {
                    self.position += 1;
                } else if c == '}' {
                    break;
                } else {
                    let context = self.get_context(self.position, 30);
                    return Err(format!("Expected semicolon or closing brace after declaration, found '{}' at position {} near {}",
                                       c, self.position, context));
                }
            } else {
                return Err("Unexpected end of input while parsing declarations".to_string());
            }
        }

        Ok(declarations)
    }

    fn parse_declaration(&mut self) -> Result<Declaration, String> {
        let mut is_custom_property = false;
        if self.position + 1 < self.input.len() {
            let c1 = self.input.chars().nth(self.position).unwrap();
            let c2 = self.input.chars().nth(self.position + 1).unwrap();

            if c1 == '-' && c2 == '-' {
                is_custom_property = true;
            }
        }

        let property = self.parse_property()?;
        self.consume_whitespace();

        self.expect_character(':')?;
        self.consume_whitespace();

        let start_pos = self.position;
        let mut end_pos = start_pos;
        let mut in_parens = 0;
        let mut in_quotes = false;
        let mut escape_next = false;

        while end_pos < self.input.len() {
            let c = self.input.chars().nth(end_pos).unwrap();

            if escape_next {
                escape_next = false;
                end_pos += 1;
                continue;
            }

            match c {
                '\\' => {
                    escape_next = true;
                    end_pos += 1;
                },
                '"' | '\'' => {
                    in_quotes = !in_quotes;
                    end_pos += 1;
                },
                '(' => {
                    in_parens += 1;
                    end_pos += 1;
                },
                ')' => {
                    if in_parens > 0 {
                        in_parens -= 1;
                    }
                    end_pos += 1;
                },
                ';' | '}' if !in_quotes && in_parens == 0 => {
                    break;
                },
                _ => {
                    end_pos += 1;
                }
            }
        }

        let value_str = self.input[start_pos..end_pos].trim().to_string();

        self.position = end_pos;

        let value = if value_str.contains(' ') && !value_str.starts_with('"') && !value_str.starts_with('\'') {
            self.parse_space_list(value_str)?
        } else {
            let mut value_parser = CssParser::new(value_str.clone());
            match value_parser.parse_value_no_comma_list() {
                Ok(parsed_value) => parsed_value,
                Err(_) => Value::Literal(value_str),
            }
        };

        Ok(Declaration {
            property,
            value,
            is_custom_property,
        })
    }

    fn parse_property(&mut self) -> Result<String, String> {
        let mut result = String::new();

        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            if c.is_alphanumeric() || c == '-' || c == '_' {
                result.push(c);
                self.position += 1;
            } else {
                break;
            }
        }

        if result.is_empty() {
            return Err("Expected property name".to_string());
        }

        Ok(result)
    }

    fn parse_identifier(&mut self) -> Result<String, String> {
        self.parse_property()
    }

    fn parse_value(&mut self) -> Result<Value, String> {
        self.consume_whitespace();

        if self.position >= self.input.len() {
            return Err("Unexpected end of input while parsing value".to_string());
        }

        let start_pos = self.position;

        let c = self.input.chars().nth(self.position).unwrap();

        let parsed_value = match c {
            '"' | '\'' => self.parse_quoted_string()?,

            '0'..='9' | '.' | '-' | '+' => self.parse_number()?,

            '#' => self.parse_color()?,

            'a'..='z' | 'A'..='Z' => {
                let identifier_start = self.position;
                let identifier = self.parse_identifier()?;

                self.consume_whitespace();
                if self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() == '(' {
                    self.position = identifier_start;

                    if identifier == "var" {
                        self.parse_var_function()?
                    } else {
                        self.parse_function()?
                    }
                }
                else if is_color_name(&identifier) {
                    Value::Color(Color::Named(identifier))
                }
                else if is_css_keyword(&identifier) {
                    Value::Keyword(identifier)
                }
                else {
                    Value::Literal(identifier)
                }
            },

            '(' => {
                return Err("Grouped expressions not yet supported".to_string());
            },

            _ => {
                let mut result = String::new();

                while self.position < self.input.len() {
                    let c = self.input.chars().nth(self.position).unwrap();

                    if c == ';' || c == '}' || c == ',' {
                        break;
                    }

                    result.push(c);
                    self.position += 1;
                }

                let result = result.trim().to_string();

                if result.is_empty() {
                    return Err("Expected value".to_string());
                }

                if result.contains(' ') {
                    self.parse_space_list(result)?
                } else {
                    Value::Literal(result)
                }
            }
        };

        self.consume_whitespace();
        if self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() == ',' {
            let mut list_values = vec![parsed_value];

            while self.position < self.input.len() {
                let c = self.input.chars().nth(self.position).unwrap();

                if c == ',' {
                    self.position += 1;
                    self.consume_whitespace();

                    if self.position >= self.input.len() ||
                        self.input.chars().nth(self.position).unwrap() == ';' ||
                        self.input.chars().nth(self.position).unwrap() == '}' {
                        break;
                    }

                    let next_value = self.parse_value_no_comma_list()?;
                    list_values.push(next_value);
                } else if c == ';' || c == '}' {
                    break;
                } else {
                    return Err(format!("Unexpected character in comma-separated list: '{}'", c));
                }
            }

            return Ok(Value::List(list_values, ListSeparator::Comma));
        }

        Ok(parsed_value)
    }

    fn parse_value_no_comma_list(&mut self) -> Result<Value, String> {
        self.consume_whitespace();

        if self.position >= self.input.len() {
            return Err("Unexpected end of input while parsing value".to_string());
        }

        let c = self.input.chars().nth(self.position).unwrap();

        match c {
            '"' | '\'' => self.parse_quoted_string(),

            '0'..='9' | '.' | '-' | '+' => self.parse_number(),

            '#' => self.parse_color(),

            'a'..='z' | 'A'..='Z' => {
                let identifier_start = self.position;
                let identifier = self.parse_identifier()?;

                self.consume_whitespace();
                if self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() == '(' {
                    self.position = identifier_start;

                    if identifier == "var" {
                        self.parse_var_function()
                    } else {
                        self.parse_function()
                    }
                }
                else if is_color_name(&identifier) {
                    Ok(Value::Color(Color::Named(identifier)))
                }
                else if is_css_keyword(&identifier) {
                    Ok(Value::Keyword(identifier))
                }
                else {
                    Ok(Value::Literal(identifier))
                }
            },

            '(' => {
                Err("Grouped expressions not yet supported".to_string())
            },

            _ => {
                let mut result = String::new();

                while self.position < self.input.len() {
                    let c = self.input.chars().nth(self.position).unwrap();

                    if c == ';' || c == '}' || c == ',' {
                        break;
                    }

                    result.push(c);
                    self.position += 1;
                }

                let result = result.trim().to_string();

                if result.is_empty() {
                    return Err("Expected value".to_string());
                }

                if result.contains(' ') {
                    self.parse_space_list(result)
                } else {
                    Ok(Value::Literal(result))
                }
            }
        }
    }

    fn parse_number(&mut self) -> Result<Value, String> {
        let mut number_str = String::new();
        let mut has_decimal = false;

        if self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            if c == '-' || c == '+' {
                number_str.push(c);
                self.position += 1;
            }
        }

        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();

            if c.is_digit(10) {
                number_str.push(c);
                self.position += 1;
            } else if c == '.' && !has_decimal {
                has_decimal = true;
                number_str.push(c);
                self.position += 1;
            } else {
                break;
            }
        }

        let numeric_value = match number_str.parse::<f64>() {
            Ok(num) => num,
            Err(_) => return Err(format!("Invalid number: {}", number_str)),
        };

        let mut unit_str = String::new();
        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();

            if c.is_alphabetic() || c == '%' {
                unit_str.push(c);
                self.position += 1;
            } else {
                break;
            }
        }

        let unit = if unit_str.is_empty() {
            None
        } else {
            Some(match unit_str.as_str() {
                "px" => Unit::Px,
                "em" => Unit::Em,
                "rem" => Unit::Rem,
                "%" => Unit::Percent,
                "vh" => Unit::Vh,
                "vw" => Unit::Vw,
                "pt" => Unit::Pt,
                "cm" => Unit::Cm,
                "mm" => Unit::Mm,
                "in" => Unit::In,
                "deg" => Unit::Deg,
                "rad" => Unit::Rad,
                "fr" => Unit::Fr,
                "s" => Unit::S,
                "ms" => Unit::Ms,
                _ => Unit::Other(unit_str),
            })
        };

        Ok(Value::Number(numeric_value, unit))
    }

    fn parse_color(&mut self) -> Result<Value, String> {
        self.position += 1;

        let mut hex_str = String::new();
        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();

            if c.is_digit(16) {
                hex_str.push(c);
                self.position += 1;
            } else {
                break;
            }
        }

        if ![3, 4, 6, 8].contains(&hex_str.len()) {
            return Err(format!("Invalid hex color format: #{}", hex_str));
        }

        Ok(Value::Color(Color::Hex(hex_str)))
    }

    fn parse_function(&mut self) -> Result<Value, String> {
        let function_name = self.parse_identifier()?;
        self.consume_whitespace();

        self.expect_character('(')?;
        self.consume_whitespace();

        let mut arguments = Vec::new();

        if self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() == ')' {
            self.position += 1;
            return Ok(Value::Function(function_name, arguments));
        }

        if function_name == "rgb" || function_name == "rgba" ||
            function_name == "hsl" || function_name == "hsla" {
            return self.parse_color_function(function_name);
        }

        let start_pos = self.position;
        let mut paren_level = 1;

        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();

            if c == '(' {
                paren_level += 1;
            } else if c == ')' {
                paren_level -= 1;
                if paren_level == 0 {
                    break;
                }
            }

            self.position += 1;
        }

        if paren_level != 0 {
            return Err("Unclosed function parenthesis".to_string());
        }

        let args_str = self.input[start_pos..self.position].trim();

        self.position += 1;

        if !args_str.is_empty() {
            let mut current_arg = String::new();
            let mut nested_level = 0;
            let mut in_quotes = false;
            let mut escape_next = false;

            for c in args_str.chars() {
                if escape_next {
                    current_arg.push(c);
                    escape_next = false;
                    continue;
                }

                match c {
                    '\\' => {
                        escape_next = true;
                        current_arg.push(c);
                    },
                    '"' | '\'' => {
                        in_quotes = !in_quotes;
                        current_arg.push(c);
                    },
                    '(' => {
                        nested_level += 1;
                        current_arg.push(c);
                    },
                    ')' => {
                        nested_level -= 1;
                        current_arg.push(c);
                    },
                    ',' if !in_quotes && nested_level == 0 => {
                        let arg = current_arg.trim().to_string();
                        if !arg.is_empty() {
                            let mut arg_parser = CssParser::new(arg.clone());
                            match arg_parser.parse_value_no_comma_list() {
                                Ok(value) => arguments.push(value),
                                Err(_) => arguments.push(Value::Literal(arg)),
                            }
                        }
                        current_arg = String::new();
                    },
                    _ => {
                        current_arg.push(c);
                    }
                }
            }

            let arg = current_arg.trim().to_string();
            if !arg.is_empty() {
                let mut arg_parser = CssParser::new(arg.clone());
                match arg_parser.parse_value_no_comma_list() {
                    Ok(value) => arguments.push(value),
                    Err(_) => arguments.push(Value::Literal(arg)),
                }
            }
        }

        Ok(Value::Function(function_name, arguments))
    }

    fn parse_color_function(&mut self, function_name: String) -> Result<Value, String> {
        let mut paren_depth = 1;
        let start_pos = self.position;

        while self.position < self.input.len() && paren_depth > 0 {
            let c = self.input.chars().nth(self.position).unwrap();
            self.position += 1;

            if c == '(' {
                paren_depth += 1;
            } else if c == ')' {
                paren_depth -= 1;
            }
        }

        if paren_depth > 0 {
            return Err("Unterminated color function".to_string());
        }

        let args_str = self.input[start_pos..self.position-1].trim().to_string();

        match function_name.as_str() {
            "rgb" => {
                let parts: Vec<&str> = args_str.split(',').collect();
                if parts.len() != 3 {
                    return Err("RGB function requires 3 arguments".to_string());
                }

                let r = parts[0].trim().parse::<u8>().map_err(|_| "Invalid red component".to_string())?;
                let g = parts[1].trim().parse::<u8>().map_err(|_| "Invalid green component".to_string())?;
                let b = parts[2].trim().parse::<u8>().map_err(|_| "Invalid blue component".to_string())?;

                Ok(Value::Color(Color::Rgb(r, g, b)))
            },
            "rgba" => {
                let parts: Vec<&str> = args_str.split(',').collect();
                if parts.len() != 4 {
                    return Err("RGBA function requires 4 arguments".to_string());
                }

                let r = parts[0].trim().parse::<u8>().map_err(|_| "Invalid red component".to_string())?;
                let g = parts[1].trim().parse::<u8>().map_err(|_| "Invalid green component".to_string())?;
                let b = parts[2].trim().parse::<u8>().map_err(|_| "Invalid blue component".to_string())?;
                let a = parts[3].trim().parse::<f32>().map_err(|_| "Invalid alpha component".to_string())?;

                Ok(Value::Color(Color::Rgba(r, g, b, a)))
            },
            "hsl" => {
                let parts: Vec<&str> = args_str.split(',').collect();
                if parts.len() != 3 {
                    return Err("HSL function requires 3 arguments".to_string());
                }

                let h = parts[0].trim().parse::<u16>().map_err(|_| "Invalid hue component".to_string())?;

                let s_str = parts[1].trim();
                let s = if s_str.ends_with('%') {
                    s_str[..s_str.len()-1].parse::<u8>().map_err(|_| "Invalid saturation component".to_string())?
                } else {
                    return Err("Saturation must be a percentage".to_string());
                };

                let l_str = parts[2].trim();
                let l = if l_str.ends_with('%') {
                    l_str[..l_str.len()-1].parse::<u8>().map_err(|_| "Invalid lightness component".to_string())?
                } else {
                    return Err("Lightness must be a percentage".to_string());
                };

                Ok(Value::Color(Color::Hsl(h, s, l)))
            },
            "hsla" => {
                let parts: Vec<&str> = args_str.split(',').collect();
                if parts.len() != 4 {
                    return Err("HSLA function requires 4 arguments".to_string());
                }

                let h = parts[0].trim().parse::<u16>().map_err(|_| "Invalid hue component".to_string())?;

                let s_str = parts[1].trim();
                let s = if s_str.ends_with('%') {
                    s_str[..s_str.len()-1].parse::<u8>().map_err(|_| "Invalid saturation component".to_string())?
                } else {
                    return Err("Saturation must be a percentage".to_string());
                };

                let l_str = parts[2].trim();
                let l = if l_str.ends_with('%') {
                    l_str[..l_str.len()-1].parse::<u8>().map_err(|_| "Invalid lightness component".to_string())?
                } else {
                    return Err("Lightness must be a percentage".to_string());
                };

                let a = parts[3].trim().parse::<f32>().map_err(|_| "Invalid alpha component".to_string())?;

                Ok(Value::Color(Color::Hsla(h, s, l, a)))
            },
            _ => {
                let args = vec![Value::Literal(args_str)];
                Ok(Value::Function(function_name, args))
            }
        }
    }

    fn parse_space_list(&mut self, value: String) -> Result<Value, String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut nesting_level = 0;

        let mut chars = value.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '(' => {
                    nesting_level += 1;
                    current_token.push(c);
                },
                ')' => {
                    nesting_level -= 1;
                    current_token.push(c);

                    if nesting_level == 0 && chars.peek().map_or(true, |&next| next.is_whitespace()) {
                        if !current_token.trim().is_empty() {
                            tokens.push(current_token.trim().to_string());
                            current_token = String::new();
                        }
                    }
                },
                ' ' | '\t' | '\n' | '\r' if nesting_level == 0 => {
                    if !current_token.trim().is_empty() {
                        tokens.push(current_token.trim().to_string());
                        current_token = String::new();
                    }
                },
                _ => {
                    current_token.push(c);
                }
            }
        }

        if !current_token.trim().is_empty() {
            tokens.push(current_token.trim().to_string());
        }

        if tokens.len() <= 1 {
            return Ok(Value::Literal(value));
        }

        let mut parsed_items = Vec::new();
        for token in tokens {
            if let Some(paren_pos) = token.find('(') {
                let func_name = &token[0..paren_pos];
                if !func_name.is_empty() {
                    let mut temp_parser = CssParser::new(token.clone());
                    match temp_parser.parse_value_no_comma_list() {
                        Ok(parsed_value) => {
                            parsed_items.push(parsed_value);
                            continue;
                        },
                        Err(_) => {
                            parsed_items.push(Value::Literal(token));
                            continue;
                        }
                    }
                }
            }

            let trimmed = token.trim();

            if let Some(first_char) = trimmed.chars().next() {
                if first_char.is_digit(10) || first_char == '-' || first_char == '+' || first_char == '.' {
                    let mut num_str = String::new();
                    let mut unit_str = String::new();
                    let mut in_unit = false;

                    for c in trimmed.chars() {
                        if !in_unit && (c.is_digit(10) || c == '-' || c == '+' || c == '.') {
                            num_str.push(c);
                        } else {
                            in_unit = true;
                            unit_str.push(c);
                        }
                    }

                    if let Ok(num) = num_str.parse::<f64>() {
                        let unit = if unit_str.is_empty() {
                            None
                        } else {
                            Some(match unit_str.as_str() {
                                "px" => Unit::Px,
                                "em" => Unit::Em,
                                "rem" => Unit::Rem,
                                "%" => Unit::Percent,
                                "vh" => Unit::Vh,
                                "vw" => Unit::Vw,
                                "pt" => Unit::Pt,
                                "cm" => Unit::Cm,
                                "mm" => Unit::Mm,
                                "in" => Unit::In,
                                "deg" => Unit::Deg,
                                "rad" => Unit::Rad,
                                "fr" => Unit::Fr,
                                "s" => Unit::S,
                                "ms" => Unit::Ms,
                                _ => Unit::Other(unit_str),
                            })
                        };

                        parsed_items.push(Value::Number(num, unit));
                        continue;
                    }
                }
            }

            if trimmed.starts_with('#') {
                let hex = &trimmed[1..];
                if [3, 4, 6, 8].contains(&hex.len()) {
                    if hex.chars().all(|c| c.is_digit(16)) {
                        parsed_items.push(Value::Color(Color::Hex(hex.to_string())));
                        continue;
                    }
                }
            }

            if is_color_name(trimmed) {
                parsed_items.push(Value::Color(Color::Named(trimmed.to_string())));
                continue;
            }

            parsed_items.push(Value::Literal(trimmed.to_string()));
        }

        Ok(Value::List(parsed_items, ListSeparator::Space))
    }


    fn parse_quoted_string(&mut self) -> Result<Value, String> {
        let quote_char = self.input.chars().nth(self.position).unwrap();
        self.position += 1;

        let mut content = String::new();
        let mut escaped = false;

        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            self.position += 1;

            if escaped {
                content.push(c);
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == quote_char {
                return Ok(Value::QuotedString(content));
            } else {
                content.push(c);
            }
        }

        Err("Unterminated string literal".to_string())
    }

    fn parse_var_function(&mut self) -> Result<Value, String> {
        self.position += 4;

        if self.position + 1 >= self.input.len() {
            return Err("Unexpected end of input in var() function".to_string());
        }

        self.consume_whitespace();

        let c1 = self.input.chars().nth(self.position).unwrap();
        let c2 = self.input.chars().nth(self.position + 1).unwrap();

        if c1 != '-' || c2 != '-' {
            return Err("Variable name in var() must start with --".to_string());
        }

        let var_name = self.parse_property()?;
        self.consume_whitespace();

        let fallback = if self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() == ',' {
            self.position += 1;
            self.consume_whitespace();

            Some(Box::new(self.parse_function_argument(')')?))
        } else {
            None
        };

        self.expect_character(')')?;

        Ok(Value::VarFunction(var_name, fallback))
    }

    fn parse_function_argument(&mut self, end_char: char) -> Result<Value, String> {
        let start_pos = self.position;
        let mut paren_depth = 0;

        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();

            if c == '(' {
                paren_depth += 1;
                self.position += 1;
            } else if c == ')' {
                if paren_depth == 0 {
                    break;
                }
                paren_depth -= 1;
                self.position += 1;
            } else if c == ',' && paren_depth == 0 && end_char == ')' {
                break;
            } else if c == '"' || c == '\'' {
                let quote_char = c;
                self.position += 1;

                let mut escaped = false;
                while self.position < self.input.len() {
                    let c = self.input.chars().nth(self.position).unwrap();
                    self.position += 1;

                    if escaped {
                        escaped = false;
                    } else if c == '\\' {
                        escaped = true;
                    } else if c == quote_char {
                        break;
                    }
                }
            } else {
                self.position += 1;
            }
        }

        if self.position >= self.input.len() {
            return Err("Unexpected end of input while parsing function argument".to_string());
        }

        let arg_text = self.input[start_pos..self.position].trim();

        if arg_text.is_empty() {
            return Err("Empty function argument".to_string());
        }

        if arg_text.starts_with("var(") {
            self.position = start_pos;
            return self.parse_var_function();
        }

        Ok(Value::Literal(arg_text.to_string()))
    }

    fn consume_whitespace(&mut self) {
        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            if c.is_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn expect_character(&mut self, expected: char) -> Result<(), String> {
        if self.position >= self.input.len() {
            return Err(format!("Expected '{}', found end of input", expected));
        }

        let c = self.input.chars().nth(self.position).unwrap();
        if c == expected {
            self.position += 1;
            Ok(())
        } else {
            Err(format!("Expected '{}', found '{}'", expected, c))
        }
    }
}
