use crate::css_parser::ast::*;
use crate::css_parser::lexer::*;

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
    lexer: Lexer,
    current_token: Option<Token>,
}

impl CssParser {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = Some(lexer.next_token());

        CssParser {
            lexer,
            current_token,
        }
    }

    // Token management methods
    fn next_token(&mut self) -> Option<Token> {
        let current = self.current_token.take();
        if current.as_ref().map_or(false, |t| matches!(t.token_type, TokenType::EOF)) {
            self.current_token = current.clone();
        } else {
            self.current_token = Some(self.lexer.next_token());
        }
        current
    }

    fn peek_token(&self) -> Option<&Token> {
        self.current_token.as_ref()
    }

    // Helper methods for expecting specific tokens
    fn expect_open_brace(&mut self) -> Result<(), String> {
        if let Some(token) = self.next_token() {
            match token.token_type {
                TokenType::OpenBrace => Ok(()),
                _ => Err(format!("Expected open brace, found {:?}", token.token_type)),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }

    fn expect_close_brace(&mut self) -> Result<(), String> {
        if let Some(token) = self.next_token() {
            match token.token_type {
                TokenType::CloseBrace => Ok(()),
                _ => Err(format!("Expected close brace, found {:?}", token.token_type)),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }

    fn expect_colon(&mut self) -> Result<(), String> {
        if let Some(token) = self.next_token() {
            match token.token_type {
                TokenType::Colon => Ok(()),
                _ => Err(format!("Expected colon, found {:?}", token.token_type)),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }

    // Main parsing methods
    pub fn parse_stylesheet(&mut self) -> Result<Stylesheet, String> {
        let mut stylesheet = Stylesheet { rules: Vec::new() };
        let start_time = std::time::Instant::now();

        while self.peek_token().is_some() &&
            !matches!(self.peek_token().unwrap().token_type, TokenType::EOF) {
            let rule = self.parse_rule()?;
            stylesheet.rules.push(rule);
        }

        let elapsed = start_time.elapsed();
        println!("Parsed {} rules in {:?}", stylesheet.rules.len(), elapsed);

        Ok(stylesheet)
    }

    fn parse_rule(&mut self) -> Result<Rule, String> {
        let selector = self.parse_selector()?;
        self.expect_open_brace()?;
        let declarations = self.parse_declarations()?;
        self.expect_close_brace()?;

        Ok(Rule {
            selector,
            declarations,
        })
    }

    fn parse_selector(&mut self) -> Result<Selector, String> {
        let mut parts = Vec::new();
        let mut first_part = true;

        while let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::OpenBrace => break,
                TokenType::Identifier(_) | TokenType::Dot | TokenType::Hash |
                TokenType::Colon | TokenType::Asterisk => {
                    let part = self.parse_selector_part(first_part)?;
                    parts.push(part);
                    first_part = false;
                },
                _ => break,
            }
        }

        if parts.is_empty() {
            return Err("Expected at least one selector part".to_string());
        }

        Ok(Selector { parts })
    }

    fn parse_selector_part(&mut self, allow_element: bool) -> Result<SelectorPart, String> {
        if let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::Dot => {
                    self.next_token(); // Consume dot
                    match self.next_token() {
                        Some(token) => {
                            if let TokenType::Identifier(name) = token.token_type {
                                Ok(SelectorPart::Class(name))
                            } else {
                                Err(format!("Expected identifier after dot, found {:?}", token.token_type))
                            }
                        },
                        None => Err("Unexpected end of input after dot".to_string()),
                    }
                },
                TokenType::Hash => {
                    self.next_token(); // Consume hash
                    match self.next_token() {
                        Some(token) => {
                            if let TokenType::Identifier(name) = token.token_type {
                                Ok(SelectorPart::Id(name))
                            } else {
                                Err(format!("Expected identifier after hash, found {:?}", token.token_type))
                            }
                        },
                        None => Err("Unexpected end of input after hash".to_string()),
                    }
                },
                TokenType::Colon => {
                    self.next_token(); // Consume colon
                    match self.next_token() {
                        Some(token) => {
                            if let TokenType::Identifier(name) = token.token_type {
                                Ok(SelectorPart::PseudoElement(name))
                            } else {
                                Err(format!("Expected identifier after colon, found {:?}", token.token_type))
                            }
                        },
                        None => Err("Unexpected end of input after colon".to_string()),
                    }
                },
                TokenType::Asterisk => {
                    self.next_token(); // Consume asterisk
                    Ok(SelectorPart::Universal)
                },
                TokenType::Identifier(name) if allow_element => {
                    let name = name.clone();
                    self.next_token(); // Consume identifier
                    Ok(SelectorPart::Element(name))
                },
                _ => Err(format!("Unexpected token in selector: {:?}", token.token_type)),
            }
        } else {
            Err("Unexpected end of input while parsing selector part".to_string())
        }
    }

    fn parse_declarations(&mut self) -> Result<Vec<Declaration>, String> {
        let mut declarations = Vec::new();

        loop {
            // Skip any semicolons before declarations
            while let Some(token) = self.peek_token() {
                if matches!(token.token_type, TokenType::Semicolon) {
                    self.next_token();
                } else {
                    break;
                }
            }

            // Check if we've reached the end of declarations
            if let Some(token) = self.peek_token() {
                if matches!(token.token_type, TokenType::CloseBrace) {
                    break;
                }
            } else {
                return Err("Unexpected end of input while parsing declarations".to_string());
            }

            // Parse a declaration
            let declaration = self.parse_declaration()?;
            declarations.push(declaration);

            // Expect a semicolon or closing brace
            match self.peek_token() {
                Some(token) if matches!(token.token_type, TokenType::Semicolon) => {
                    self.next_token(); // Consume semicolon
                },
                Some(token) if matches!(token.token_type, TokenType::CloseBrace) => {
                    break;
                },
                Some(token) => {
                    return Err(format!("Expected semicolon or closing brace after declaration, found {:?}", token.token_type));
                },
                None => {
                    return Err("Unexpected end of input while parsing declarations".to_string());
                }
            }
        }

        Ok(declarations)
    }
    
    fn parse_value(&mut self) -> Result<Value, String> {
        // This will replace our simplified parse_simple_value
        if let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::Identifier(name) => {
                    // Check if this is a function
                    let name_clone = name.clone();
                    self.next_token(); // Consume the identifier

                    if let Some(next) = self.peek_token() {
                        if matches!(next.token_type, TokenType::OpenParen) {
                            // This is a function
                            return if name_clone == "var" {
                                self.parse_var_function()
                            } else {
                                self.parse_function(name_clone)
                            }
                        }
                    }

                    // Not a function, handle as identifier
                    if is_color_name(&name_clone) {
                        Ok(Value::Color(Color::Named(name_clone)))
                    } else if is_css_keyword(&name_clone) {
                        Ok(Value::Keyword(name_clone))
                    } else {
                        Ok(Value::Literal(name_clone))
                    }
                },
                TokenType::Number(_) => self.parse_number(),
                TokenType::String(text) => {
                    let text_clone = text.clone();
                    self.next_token(); // Consume the string
                    Ok(Value::QuotedString(text_clone))
                },
                TokenType::Hash => self.parse_hex_color(),
                TokenType::HexColor(hex) => {
                    let hex_clone = hex.clone();
                    self.next_token(); // Consume token
                    Ok(Value::Color(Color::Hex(format!("#{}", hex_clone))))
                },
                _ => {
                    // For other token types, try to parse a literal
                    let token = self.next_token().unwrap();
                    Ok(Value::Literal(format!("{}", token.token_type)))
                }
            }
        } else {
            Err("Unexpected end of input while parsing value".to_string())
        }
    }

    fn parse_number(&mut self) -> Result<Value, String> {
        if let Some(token) = self.next_token() {
            if let TokenType::Number(num) = token.token_type {
                // Check for unit
                if let Some(next) = self.peek_token().cloned() {
                    if let TokenType::Unit(unit_str) = &next.token_type {
                        self.next_token(); // Consume unit token
                        let unit = match unit_str.as_str() {
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
                            _ => Unit::Other(unit_str.clone()),
                        };
                        return Ok(Value::Number(num, Some(unit)));
                    }
                }

                Ok(Value::Number(num, None))
            } else {
                Err(format!("Expected number, found {:?}", token.token_type))
            }
        } else {
            Err("Unexpected end of input while parsing number".to_string())
        }
    }

    fn parse_hex_color(&mut self) -> Result<Value, String> {
        self.next_token(); // Consume hash token

        if let Some(token) = self.next_token() {
            if let TokenType::HexColor(hex) = token.token_type {
                println!("Parsed hex color: {}", hex);
                Ok(Value::Color(Color::Hex(format!("#{}", hex))))
            } else if let TokenType::Identifier(name) = token.token_type {
                // Handle case where lexer didn't recognize it as HexColor
                if name.chars().all(|c| c.is_digit(16)) {
                    Ok(Value::Color(Color::Hex(format!("#{}", name))))
                } else {
                    Ok(Value::Literal(format!("#{}", name)))
                }
            } else {
                Err(format!("Expected hex color after #, found {:?}", token.token_type))
            }
        } else {
            Err("Unexpected end of input after #".to_string())
        }
    }

    fn parse_function(&mut self, name: String) -> Result<Value, String> {
        self.expect_open_paren()?;

        if name == "calc" {
            return self.parse_calc_function();
        }

        let color_functions = [
            "rgb", "rgba", "hsl", "hsla", "hwb", "lab", "lch", "color"
        ];

        // Special handling for color functions
        if color_functions.contains(&name.as_str()) {
            return self.parse_color_function(name);
        }

        // Parse function arguments
        let mut arguments = Vec::new();

        // Check for empty function
        if let Some(token) = self.peek_token() {
            if matches!(token.token_type, TokenType::CloseParen) {
                self.next_token(); // Consume closing parenthesis
                return Ok(Value::Function(name, arguments));
            }
        }

        // Parse arguments
        loop {
            let arg = self.parse_function_argument()?;
            arguments.push(arg);

            // Check for comma or closing parenthesis
            if let Some(token) = self.peek_token() {
                match &token.token_type {
                    TokenType::Comma => {
                        self.next_token(); // Consume comma
                    },
                    TokenType::CloseParen => {
                        self.next_token(); // Consume closing parenthesis
                        break;
                    },
                    _ => return Err(format!("Expected comma or closing parenthesis, found {:?}", token.token_type)),
                }
            } else {
                return Err("Unexpected end of input while parsing function arguments".to_string());
            }
        }

        Ok(Value::Function(name, arguments))
    }

    fn parse_calc_function(&mut self) -> Result<Value, String> {
        // TODO
        Err("calc function not implemented".to_string())
    }

    fn parse_color_function(&mut self, function_name: String) -> Result<Value, String> {
        // Track if we've seen a slash for alpha
        let mut has_slash = false;
        let mut components = Vec::new();

        // Parse until we reach the closing parenthesis
        while let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::CloseParen => {
                    self.next_token(); // Consume closing parenthesis
                    break;
                },
                TokenType::Slash => {
                    if has_slash {
                        return Err("Multiple slashes in color function not allowed".to_string());
                    }
                    has_slash = true;
                    self.next_token(); // Consume slash
                    // The next value should be the alpha component
                    continue;
                },
                TokenType::Comma => {
                    // Traditional syntax with commas
                    self.next_token(); // Consume comma
                    continue;
                },
                _ => {
                    // Parse the next component
                    let component = self.parse_value()?;
                    components.push(component);
                }
            }
        }

        // Convert to the appropriate color type based on the function name
        match function_name.as_str() {
            "rgb" | "rgba" => {
                if components.len() < 3 || components.len() > 4 {
                    return Err(format!("RGB function requires 3 or 4 components, found {}", components.len()));
                }

                let r = self.extract_component(&components[0])?;
                let g = self.extract_component(&components[1])?;
                let b = self.extract_component(&components[2])?;

                if components.len() == 4 || has_slash {
                    // We have an alpha component
                    let alpha_idx = if has_slash { 3 } else { 3 };
                    if alpha_idx < components.len() {
                        let a = self.extract_alpha(&components[alpha_idx])?;
                        Ok(Value::Color(Color::Rgba(r, g, b, a)))
                    } else {
                        Err("Expected alpha component after slash".to_string())
                    }
                } else {
                    // No alpha
                    Ok(Value::Color(Color::Rgb(r, g, b)))
                }
            },
            "hsl" | "hsla" => {
                if components.len() < 3 || components.len() > 4 {
                    return Err(format!("HSL function requires 3 or 4 components, found {}", components.len()));
                }

                let h = self.extract_hue(&components[0])?;
                let s = self.extract_percentage(&components[1])?;
                let l = self.extract_percentage(&components[2])?;

                if components.len() == 4 || has_slash {
                    // We have an alpha component
                    let alpha_idx = if has_slash { 3 } else { 3 };
                    if alpha_idx < components.len() {
                        let a = self.extract_alpha(&components[alpha_idx])?;
                        Ok(Value::Color(Color::Hsla(h, s, l, a)))
                    } else {
                        Err("Expected alpha component after slash".to_string())
                    }
                } else {
                    // No alpha
                    Ok(Value::Color(Color::Hsl(h, s, l)))
                }
            },
            "hwb" => {
                if components.len() < 3 {
                    return Err(format!("HWB function requires at least 3 components, found {}", components.len()));
                }

                // Since we don't have a dedicated HWB type, we'll use the Function type
                Ok(Value::Function("hwb".to_string(), components))
            },
            "lab" => {
                if components.len() < 3 {
                    return Err(format!("LAB function requires at least 3 components, found {}", components.len()));
                }

                // Since we don't have a dedicated LAB type, we'll use the Function type
                Ok(Value::Function("lab".to_string(), components))
            },
            "lch" => {
                if components.len() < 3 {
                    return Err(format!("LCH function requires at least 3 components, found {}", components.len()));
                }

                // Since we don't have a dedicated LCH type, we'll use the Function type
                Ok(Value::Function("lch".to_string(), components))
            },
            "color" => {
                if components.len() < 1 {
                    return Err("Color function requires a color space parameter".to_string());
                }

                // Since we don't have a dedicated type for the color() function, we'll use the Function type
                Ok(Value::Function("color".to_string(), components))
            },
            _ => {
                // This shouldn't happen based on our function name check
                Err(format!("Unsupported color function: {}", function_name))
            }
        }
    }

    fn extract_component(&self, value: &Value) -> Result<u8, String> {
        match value {
            Value::Number(num, _) => {
                // Clamp to 0-255 range
                let clamped = num.max(0.0).min(255.0);
                Ok(clamped as u8)
            },
            _ => Err(format!("Expected number for color component, found {:?}", value)),
        }
    }

    fn extract_hue(&self, value: &Value) -> Result<u16, String> {
        match value {
            Value::Number(num, _) => {
                // Wrap hue values to 0-360 range
                let wrapped = num.rem_euclid(360.0);
                Ok(wrapped as u16)
            },
            _ => Err(format!("Expected number for hue component, found {:?}", value)),
        }
    }

    fn extract_percentage(&self, value: &Value) -> Result<u8, String> {
        match value {
            Value::Number(num, Some(Unit::Percent)) => {
                // Clamp to 0-100 range
                let clamped = num.max(0.0).min(100.0);
                Ok(clamped as u8)
            },
            _ => Err(format!("Expected percentage for saturation/lightness component, found {:?}", value)),
        }
    }

    fn extract_alpha(&self, value: &Value) -> Result<f32, String> {
        match value {
            Value::Number(num, _) => {
                // Clamp to 0-1 range
                let clamped = num.max(0.0).min(1.0);
                Ok(clamped as f32)
            },
            _ => Err(format!("Expected number for alpha component, found {:?}", value)),
        }
    }

    fn parse_var_function(&mut self) -> Result<Value, String> {
        self.expect_open_paren()?;

        // Parse variable name
        let variable_name = match self.next_token() {
            Some(token) => {
                if let TokenType::Identifier(name) = token.token_type {
                    if !name.starts_with("--") {
                        return Err(format!("Variable name must start with --, found {}", name));
                    }
                    name
                } else {
                    return Err(format!("Expected variable name, found {:?}", token.token_type));
                }
            },
            None => return Err("Unexpected end of input while parsing var function".to_string()),
        };

        // Check for fallback value
        let fallback = if let Some(token) = self.peek_token() {
            if matches!(token.token_type, TokenType::Comma) {
                self.next_token(); // Consume comma
                Some(Box::new(self.parse_function_argument()?))
            } else {
                None
            }
        } else {
            None
        };

        // Expect closing parenthesis
        self.expect_close_paren()?;

        Ok(Value::VarFunction(variable_name, fallback))
    }

    fn parse_function_argument(&mut self) -> Result<Value, String> {
        self.parse_value()
    }

    fn parse_declaration(&mut self) -> Result<Declaration, String> {
        // Check for custom property
        let mut is_custom_property = false;
        if let Some(token) = self.peek_token() {
            if let TokenType::Identifier(name) = &token.token_type {
                if name.starts_with("--") {
                    is_custom_property = true;
                }
            }
        }

        // Parse property name
        let property = match self.next_token() {
            Some(token) => {
                if let TokenType::Identifier(name) = token.token_type {
                    name
                } else {
                    return Err(format!("Expected property name, found {:?}", token.token_type));
                }
            },
            None => return Err("Unexpected end of input while parsing property".to_string()),
        };

        // Expect colon
        self.expect_colon()?;

        // Parse the value, which might be a list
        let value = self.parse_value_possibly_list()?;

        Ok(Declaration {
            property,
            value,
            is_custom_property,
        })
    }

    fn parse_value_possibly_list(&mut self) -> Result<Value, String> {
        // Parse the first value
        let first_value = self.parse_value()?;

        // Check if there are more values to form a list
        let mut values = vec![first_value];
        let mut separator = None;

        loop {
            if let Some(token) = self.peek_token() {
                match &token.token_type {
                    TokenType::Semicolon | TokenType::CloseBrace => {
                        // End of the declaration, stop parsing
                        break;
                    },
                    TokenType::Comma => {
                        // Comma-separated list
                        if separator.is_some() && separator != Some(ListSeparator::Comma) {
                            return Err("Mixed separators in list not allowed".to_string());
                        }
                        separator = Some(ListSeparator::Comma);
                        self.next_token(); // Consume the comma
                        let next_value = self.parse_value()?;
                        values.push(next_value);
                    },
                    _ => {
                        // Space-separated list (or error)
                        if separator.is_some() && separator != Some(ListSeparator::Space) {
                            return Err("Mixed separators in list not allowed".to_string());
                        }

                        // Try to parse the next value
                        let result = self.parse_value();
                        match result {
                            Ok(next_value) => {
                                separator = Some(ListSeparator::Space);
                                values.push(next_value);
                            },
                            Err(_) => {
                                // Not a valid value, end of the list
                                break;
                            }
                        }
                    }
                }
            } else {
                // End of input
                break;
            }
        }

        if values.len() == 1 {
            // Just a single value, not a list
            Ok(values.remove(0))
        } else {
            // A list of values
            Ok(Value::List(values, separator.unwrap_or(ListSeparator::Space)))
        }
    }

    // Helper methods for expecting specific tokens
    fn expect_open_paren(&mut self) -> Result<(), String> {
        if let Some(token) = self.next_token() {
            match token.token_type {
                TokenType::OpenParen => Ok(()),
                _ => Err(format!("Expected opening parenthesis, found {:?}", token.token_type)),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }

    fn expect_close_paren(&mut self) -> Result<(), String> {
        if let Some(token) = self.next_token() {
            match token.token_type {
                TokenType::CloseParen => Ok(()),
                _ => Err(format!("Expected closing parenthesis, found {:?}", token.token_type)),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }
}
