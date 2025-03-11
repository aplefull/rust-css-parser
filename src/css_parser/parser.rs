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

    pub fn parse_stylesheet(&mut self) -> Result<Stylesheet, String> {
        let mut rules = Vec::new();
        let mut at_rules = Vec::new();
        let start_time = std::time::Instant::now();

        while self.peek_token().is_some() &&
            !matches!(self.peek_token().unwrap().token_type, TokenType::EOF) {

            if let Some(token) = self.peek_token() {
                match &token.token_type {
                    TokenType::AtSymbol => {
                        let at_rule = self.parse_at_rule()?;
                        at_rules.push(at_rule);
                        continue;
                    },
                    _ => {}
                }
            }

            let rule = self.parse_rule()?;
            rules.push(rule);
        }

        let elapsed = start_time.elapsed();
        println!("Parsed {} rules and {} at-rules in {:?}", rules.len(), at_rules.len(), elapsed);

        Ok(Stylesheet { rules, at_rules })
    }

    fn parse_rule(&mut self) -> Result<Rule, String> {
        let first_selector = self.parse_selector()?;
        let mut selectors = vec![first_selector];

        while let Some(token) = self.peek_token() {
            if matches!(token.token_type, TokenType::Comma) {
                self.next_token();

                let next_selector = self.parse_selector()?;
                selectors.push(next_selector);
            } else {
                break;
            }
        }

        self.expect_open_brace()?;
        let declarations = self.parse_declarations()?;
        self.expect_close_brace()?;

        Ok(Rule {
            selectors,
            declarations,
        })
    }

    fn parse_at_rule(&mut self) -> Result<AtRule, String> {
        self.next_token();

        let rule_type = if let Some(token) = self.next_token() {
            match &token.token_type {
                TokenType::Identifier(name) => {
                    match name.to_lowercase().as_str() {
                        "media" => AtRuleType::Media,
                        "keyframes" => AtRuleType::Keyframes,
                        "import" => AtRuleType::Import,
                        "font-face" => AtRuleType::FontFace,
                        "supports" => AtRuleType::Supports,
                        _ => AtRuleType::Unknown(name.clone()),
                    }
                },
                _ => return Err(format!("Expected identifier after @, found {:?}", token.token_type)),
            }
        } else {
            return Err("Unexpected end of input after @".to_string());
        };

        let mut query = String::new();
        while let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::OpenBrace => break,
                _ => {
                    let token = self.next_token().unwrap();
                    query.push_str(&format!("{} ", token.token_type));
                }
            }
        }

        query = query.trim().to_string();

        self.expect_open_brace()?;

        if matches!(rule_type, AtRuleType::Import) {
            while let Some(token) = self.peek_token() {
                if matches!(token.token_type, TokenType::Semicolon) {
                    self.next_token();
                    break;
                } else {
                    self.next_token();
                }
            }
            return Ok(AtRule { rule_type, query, rules: Vec::new() });
        }

        let mut rules = Vec::new();

        while let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::CloseBrace => {
                    self.next_token();
                    break;
                },
                _ => {
                    let rule = self.parse_rule()?;
                    rules.push(rule);
                }
            }
        }

        Ok(AtRule { rule_type, query, rules })
    }

    fn parse_selector(&mut self) -> Result<Selector, String> {
        let mut groups = Vec::new();
        let mut combinators = Vec::new();

        let first_group = self.parse_selector_group()?;
        groups.push(first_group);

        while let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::GreaterThan => {
                    self.next_token();
                    combinators.push(SelectorCombinator::Child);
                    let next_group = self.parse_selector_group()?;
                    groups.push(next_group);
                },
                TokenType::Plus => {
                    self.next_token();
                    combinators.push(SelectorCombinator::AdjacentSibling);
                    let next_group = self.parse_selector_group()?;
                    groups.push(next_group);
                },
                TokenType::Tilde => {
                    self.next_token();
                    combinators.push(SelectorCombinator::GeneralSibling);
                    let next_group = self.parse_selector_group()?;
                    groups.push(next_group);
                },
                TokenType::Identifier(_) | TokenType::Dot | TokenType::Hash |
                TokenType::Colon | TokenType::DoubleColon | TokenType::Asterisk => {
                    combinators.push(SelectorCombinator::Descendant);
                    let next_group = self.parse_selector_group()?;
                    groups.push(next_group);
                },
                TokenType::OpenBrace => break,
                _ => break,
            }
        }

        Ok(Selector { groups, combinators })
    }

    fn parse_selector_group(&mut self) -> Result<SelectorGroup, String> {
        let mut parts = Vec::new();

        while let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::OpenBrace | TokenType::GreaterThan | TokenType::Plus | TokenType::Tilde => break,
                TokenType::Identifier(_) | TokenType::Dot | TokenType::Hash |
                TokenType::Colon | TokenType::DoubleColon | TokenType::Asterisk | TokenType::OpenBracket => {
                    let part = self.parse_selector_part(true)?;
                    parts.push(part);
                },
                _ => break,
            }
        }

        if parts.is_empty() {
            return Err("Expected at least one selector part".to_string());
        }

        Ok(SelectorGroup { parts })
    }

    fn parse_selector_part(&mut self, allow_element: bool) -> Result<SelectorPart, String> {
        if let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::Dot => {
                    self.next_token();
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
                    self.next_token();
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
                    self.next_token();
                    match self.next_token() {
                        Some(token) => {
                            if let TokenType::Identifier(name) = token.token_type {
                                if let Some(peek_token) = self.peek_token() {
                                    if matches!(peek_token.token_type, TokenType::OpenParen) {
                                        self.next_token();
                                        let args = self.parse_pseudo_class_arguments()?;
                                        return Ok(SelectorPart::PseudoClassFunction(name, args));
                                    }
                                }
                                Ok(SelectorPart::PseudoClass(name))
                            } else {
                                Err(format!("Expected identifier after colon, found {:?}", token.token_type))
                            }
                        },
                        None => Err("Unexpected end of input after colon".to_string()),
                    }
                },

                TokenType::DoubleColon => {
                    self.next_token();
                    match self.next_token() {
                        Some(token) => {
                            if let TokenType::Identifier(name) = token.token_type {
                                Ok(SelectorPart::PseudoElement(name))
                            } else {
                                Err(format!("Expected identifier after double colon, found {:?}", token.token_type))
                            }
                        },
                        None => Err("Unexpected end of input after double colon".to_string()),
                    }
                },
                TokenType::Asterisk => {
                    self.next_token();
                    Ok(SelectorPart::Universal)
                },
                TokenType::Identifier(name) if allow_element => {
                    let name = name.clone();
                    self.next_token();
                    Ok(SelectorPart::Element(name))
                },
                TokenType::OpenBracket => {
                    self.parse_attribute_selector()
                },
                _ => Err(format!("Unexpected token in selector: {:?}", token.token_type)),
            }
        } else {
            Err("Unexpected end of input while parsing selector part".to_string())
        }
    }

    fn parse_pseudo_class_arguments(&mut self) -> Result<String, String> {
        let mut args = String::new();
        let mut paren_depth = 1;

        while paren_depth > 0 {
            if let Some(token) = self.peek_token() {
                match &token.token_type {
                    TokenType::OpenParen => {
                        paren_depth += 1;
                        args.push('(');
                        self.next_token();
                    },
                    TokenType::CloseParen => {
                        paren_depth -= 1;
                        if paren_depth > 0 {
                            args.push(')');
                        }
                        self.next_token();
                        if paren_depth == 0 {
                            break;
                        }
                    },
                    _ => {
                        let token = self.next_token().unwrap();
                        match &token.token_type {
                            TokenType::Identifier(name) => args.push_str(name),
                            TokenType::Number(num) => args.push_str(&num.to_string()),
                            TokenType::String(text) => args.push_str(&format!("\"{}\"", text)),
                            TokenType::Colon => args.push(':'),
                            TokenType::Dot => args.push('.'),
                            TokenType::Hash => args.push('#'),
                            TokenType::Plus => args.push('+'),
                            TokenType::Minus => args.push('-'),
                            TokenType::Asterisk => args.push('*'),
                            TokenType::Comma => args.push_str(", "),
                            _ => args.push_str(&format!("{}", token.token_type)),
                        }
                    }
                }
            } else {
                return Err("Unexpected end of input while parsing pseudo-class arguments".to_string());
            }
        }

        Ok(args.trim().to_string())
    }


    fn parse_declarations(&mut self) -> Result<Vec<Declaration>, String> {
        let mut declarations = Vec::new();

        loop {
            while let Some(token) = self.peek_token() {
                if matches!(token.token_type, TokenType::Semicolon) {
                    self.next_token();
                } else {
                    break;
                }
            }

            if let Some(token) = self.peek_token() {
                if matches!(token.token_type, TokenType::CloseBrace) {
                    break;
                }
            } else {
                return Err("Unexpected end of input while parsing declarations".to_string());
            }

            let declaration = self.parse_declaration()?;
            declarations.push(declaration);

            match self.peek_token() {
                Some(token) if matches!(token.token_type, TokenType::Semicolon) => {
                    self.next_token();
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
        if let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::Identifier(name) => {
                    let name_clone = name.clone();
                    self.next_token();

                    if let Some(next) = self.peek_token() {
                        if matches!(next.token_type, TokenType::OpenParen) {
                            return if name_clone == "var" {
                                self.parse_var_function()
                            } else {
                                self.parse_function(name_clone)
                            }
                        }
                    }

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
                    self.next_token();
                    Ok(Value::QuotedString(text_clone))
                },
                TokenType::Hash => self.parse_hex_color(),
                TokenType::HexColor(hex) => {
                    let hex_clone = hex.clone();
                    self.next_token();
                    Ok(Value::Color(Color::Hex(format!("#{}", hex_clone))))
                },
                _ => {
                    let token = self.next_token().unwrap();
                    Ok(Value::Literal(format!("{}", token.token_type)))
                }
            }
        } else {
            Err("Unexpected end of input while parsing value".to_string())
        }
    }

    fn parse_attribute_selector(&mut self) -> Result<SelectorPart, String> {
        self.next_token();

        let attr_name = match self.next_token() {
            Some(token) => {
                if let TokenType::Identifier(name) = token.token_type {
                    name
                } else {
                    return Err(format!("Expected attribute name, found {:?}", token.token_type));
                }
            },
            None => return Err("Unexpected end of input while parsing attribute selector".to_string()),
        };

        if let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::CloseBracket => {
                    self.next_token();
                    return Ok(SelectorPart::AttributeSelector(attr_name, None));
                },
                _ => {
                    let operator = self.parse_attribute_operator()?;
                    let value = self.parse_attribute_value()?;

                    let mut case_sensitivity = None;

                    if let Some(token) = self.peek_token() {
                        if let TokenType::Identifier(modifier) = &token.token_type {
                            if modifier == "i" {
                                case_sensitivity = Some(CaseSensitivity::Insensitive);
                                self.next_token();
                            } else if modifier == "s" {
                                case_sensitivity = Some(CaseSensitivity::Sensitive);
                                self.next_token();
                            }
                        }
                    }

                    if let Some(token) = self.next_token() {
                        if !matches!(token.token_type, TokenType::CloseBracket) {
                            return Err(format!("Expected closing bracket, found {:?}", token.token_type));
                        }
                    } else {
                        return Err("Unexpected end of input while parsing attribute selector".to_string());
                    }

                    return Ok(SelectorPart::AttributeSelector(attr_name, Some((operator, value, case_sensitivity))));
                }
            }
        }

        Err("Unexpected end of input while parsing attribute selector".to_string())
    }

    fn parse_attribute_operator(&mut self) -> Result<AttributeOperator, String> {
        match self.next_token() {
            Some(token) => {
                match &token.token_type {
                    TokenType::Equals => {
                        Ok(AttributeOperator::Equals)
                    },
                    TokenType::Tilde => {
                        if let Some(next) = self.next_token() {
                            if matches!(next.token_type, TokenType::Equals) {
                                Ok(AttributeOperator::Includes)
                            } else {
                                Err(format!("Expected = after ~, found {:?}", next.token_type))
                            }
                        } else {
                            Err("Unexpected end of input after ~".to_string())
                        }
                    },
                    TokenType::Pipe => {
                        if let Some(next) = self.next_token() {
                            if matches!(next.token_type, TokenType::Equals) {
                                Ok(AttributeOperator::DashMatch)
                            } else {
                                Err(format!("Expected = after |, found {:?}", next.token_type))
                            }
                        } else {
                            Err("Unexpected end of input after |".to_string())
                        }
                    },
                    TokenType::Caret => {
                        if let Some(next) = self.next_token() {
                            if matches!(next.token_type, TokenType::Equals) {
                                Ok(AttributeOperator::StartsWith)
                            } else {
                                Err(format!("Expected = after ^, found {:?}", next.token_type))
                            }
                        } else {
                            Err("Unexpected end of input after ^".to_string())
                        }
                    },
                    TokenType::Dollar => {
                        if let Some(next) = self.next_token() {
                            if matches!(next.token_type, TokenType::Equals) {
                                Ok(AttributeOperator::EndsWith)
                            } else {
                                Err(format!("Expected = after $, found {:?}", next.token_type))
                            }
                        } else {
                            Err("Unexpected end of input after $".to_string())
                        }
                    },
                    TokenType::Asterisk => {
                        if let Some(next) = self.next_token() {
                            if matches!(next.token_type, TokenType::Equals) {
                                Ok(AttributeOperator::Contains)
                            } else {
                                Err(format!("Expected = after *, found {:?}", next.token_type))
                            }
                        } else {
                            Err("Unexpected end of input after *".to_string())
                        }
                    },
                    _ => Err(format!("Expected attribute operator, found {:?}", token.token_type)),
                }
            },
            None => Err("Unexpected end of input while parsing attribute operator".to_string()),
        }
    }

    fn parse_attribute_value(&mut self) -> Result<String, String> {
        match self.next_token() {
            Some(token) => {
                match &token.token_type {
                    TokenType::String(value) => Ok(value.clone()),
                    TokenType::Identifier(value) => Ok(value.clone()),
                    _ => Err(format!("Expected attribute value, found {:?}", token.token_type)),
                }
            },
            None => Err("Unexpected end of input while parsing attribute value".to_string()),
        }
    }

    fn parse_number(&mut self) -> Result<Value, String> {
        if let Some(token) = self.next_token() {
            if let TokenType::Number(num) = token.token_type {
                if let Some(next) = self.peek_token().cloned() {
                    if let TokenType::Unit(unit_str) = &next.token_type {
                        self.next_token();
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
        self.next_token();

        if let Some(token) = self.next_token() {
            if let TokenType::HexColor(hex) = token.token_type {
                println!("Parsed hex color: {}", hex);
                Ok(Value::Color(Color::Hex(format!("#{}", hex))))
            } else if let TokenType::Identifier(name) = token.token_type {
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

        if name.to_lowercase() == "url" {
            let url_value = self.parse_url_argument()?;
            self.expect_close_paren()?;
            return Ok(Value::Function(name, vec![Value::Literal(url_value)]));
        }

        if name.to_lowercase() == "calc" {
            return self.parse_calc_function();
        }

        let math_functions = ["min", "max", "clamp"];
        if math_functions.contains(&name.to_lowercase().as_str()) {
            let expression = self.parse_calc_expression()?;
            self.expect_close_paren()?;
            return Ok(Value::Calc(CalcExpression::Function(name, vec![expression])));
        }

        let color_functions = [
            "rgb", "rgba", "hsl", "hsla", "hwb", "lab", "lch", "color"
        ];

        if color_functions.contains(&name.as_str()) {
            return self.parse_color_function(name);
        }

        let mut arguments = Vec::new();

        if let Some(token) = self.peek_token() {
            if matches!(token.token_type, TokenType::CloseParen) {
                self.next_token();
                return Ok(Value::Function(name, arguments));
            }
        }

        loop {
            let arg = self.parse_function_argument()?;
            arguments.push(arg);

            if let Some(token) = self.peek_token() {
                match &token.token_type {
                    TokenType::Comma => {
                        self.next_token();
                    },
                    TokenType::CloseParen => {
                        self.next_token();
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

    fn parse_url_argument(&mut self) -> Result<String, String> {
        let mut url = String::new();
        let mut paren_depth = 0;

        if let Some(token) = self.peek_token().cloned() {
            match &token.token_type {
                TokenType::String(text) => {
                    self.next_token();
                    return Ok(text.clone());
                },
                _ => {
                    while let Some(token) = self.peek_token() {
                        match &token.token_type {
                            TokenType::OpenParen => {
                                paren_depth += 1;
                                url.push('(');
                                self.next_token();
                            },
                            TokenType::CloseParen => {
                                if paren_depth == 0 {
                                    break;
                                }
                                paren_depth -= 1;
                                url.push(')');
                                self.next_token();
                            },
                            TokenType::Semicolon | TokenType::OpenBrace | TokenType::CloseBrace => {
                                break;
                            },
                            _ => {
                                let token = self.next_token().unwrap();
                                match &token.token_type {
                                    TokenType::Identifier(text) => url.push_str(text),
                                    TokenType::Number(num) => url.push_str(&num.to_string()),
                                    TokenType::Dot => url.push('.'),
                                    TokenType::Slash => url.push('/'),
                                    TokenType::Minus => url.push('-'),
                                    TokenType::Colon => url.push(':'),
                                    TokenType::Hash => url.push('#'),
                                    TokenType::Plus => url.push('+'),
                                    _ => {
                                        url.push_str(&format!("{}", token.token_type))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(url.trim().to_string())
    }

    fn parse_calc_function(&mut self) -> Result<Value, String> {
        let expression = self.parse_calc_expression()?;

        self.expect_close_paren()?;

        Ok(Value::Calc(expression))
    }

    fn parse_calc_expression(&mut self) -> Result<CalcExpression, String> {
        self.parse_calc_add_sub()
    }

    fn parse_calc_add_sub(&mut self) -> Result<CalcExpression, String> {
        let mut left = self.parse_calc_mul_div()?;

        loop {
            if let Some(token) = self.peek_token() {
                match &token.token_type {
                    TokenType::Plus => {
                        self.next_token();
                        let right = self.parse_calc_mul_div()?;
                        left = CalcExpression::BinaryOperation(
                            Box::new(left),
                            CalcOperator::Add,
                            Box::new(right)
                        );
                    },
                    TokenType::Minus => {
                        self.next_token();
                        let right = self.parse_calc_mul_div()?;
                        left = CalcExpression::BinaryOperation(
                            Box::new(left),
                            CalcOperator::Subtract,
                            Box::new(right)
                        );
                    },
                    TokenType::Identifier(name) if name == "-" => {
                        self.next_token();
                        let right = self.parse_calc_mul_div()?;
                        left = CalcExpression::BinaryOperation(
                            Box::new(left),
                            CalcOperator::Subtract,
                            Box::new(right)
                        );
                    },
                    _ => break,
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_calc_mul_div(&mut self) -> Result<CalcExpression, String> {
        let mut left = self.parse_calc_primary()?;

        loop {
            if let Some(token) = self.peek_token() {
                match &token.token_type {
                    TokenType::Asterisk => {
                        self.next_token();
                        let right = self.parse_calc_primary()?;
                        left = CalcExpression::BinaryOperation(
                            Box::new(left),
                            CalcOperator::Multiply,
                            Box::new(right)
                        );
                    },
                    TokenType::Slash => {
                        self.next_token();
                        let right = self.parse_calc_primary()?;
                        left = CalcExpression::BinaryOperation(
                            Box::new(left),
                            CalcOperator::Divide,
                            Box::new(right)
                        );
                    },
                    TokenType::Identifier(name) if name == "/" => {
                        self.next_token();
                        let right = self.parse_calc_primary()?;
                        left = CalcExpression::BinaryOperation(
                            Box::new(left),
                            CalcOperator::Divide,
                            Box::new(right)
                        );
                    },
                    _ => break,
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_calc_primary(&mut self) -> Result<CalcExpression, String> {
        if let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::Number(_) => {
                    let (num, unit) = self.parse_number_with_unit()?;
                    Ok(CalcExpression::Number(num, unit))
                },
                TokenType::Identifier(name) if name == "var" => {
                    let var_name = self.parse_var_name()?;
                    Ok(CalcExpression::Variable(var_name))
                },
                TokenType::Identifier(name) if name == "-" => {
                    self.next_token();
                    let expr = self.parse_calc_primary()?;

                    Ok(CalcExpression::BinaryOperation(
                        Box::new(CalcExpression::Number(0.0, None)),
                        CalcOperator::Subtract,
                        Box::new(expr)
                    ))
                },
                TokenType::Identifier(name) => {
                    let name_clone = name.clone();
                    self.next_token();

                    if let Some(token) = self.peek_token() {
                        if matches!(token.token_type, TokenType::OpenParen) {
                            self.next_token();

                            let mut args = Vec::new();

                            if let Some(token) = self.peek_token() {
                                if matches!(token.token_type, TokenType::CloseParen) {
                                    self.next_token();
                                    return Ok(CalcExpression::Function(name_clone, args));
                                }
                            }

                            let arg = self.parse_calc_expression()?;
                            args.push(arg);

                            while let Some(token) = self.peek_token() {
                                match &token.token_type {
                                    TokenType::Comma => {
                                        self.next_token();
                                        let arg = self.parse_calc_expression()?;
                                        args.push(arg);
                                    },
                                    TokenType::CloseParen => {
                                        self.next_token();
                                        break;
                                    },
                                    _ => return Err(format!("Expected comma or closing parenthesis, found {:?}", token.token_type)),
                                }
                            }

                            return Ok(CalcExpression::Function(name_clone, args));
                        }
                    }

                    Ok(CalcExpression::Number(0.0, None))
                },
                TokenType::OpenParen => {
                    self.next_token();
                    let expr = self.parse_calc_expression()?;

                    if let Some(token) = self.next_token() {
                        if matches!(token.token_type, TokenType::CloseParen) {
                            return Ok(CalcExpression::Parenthesized(Box::new(expr)));
                        } else {
                            return Err(format!("Expected closing parenthesis, found {:?}", token.token_type));
                        }
                    } else {
                        return Err("Unexpected end of input while parsing parenthesized expression".to_string());
                    }
                },
                TokenType::Plus => {
                    self.next_token();
                    self.parse_calc_primary()
                },
                TokenType::Minus => {
                    self.next_token();
                    let expr = self.parse_calc_primary()?;

                    Ok(CalcExpression::BinaryOperation(
                        Box::new(CalcExpression::Number(0.0, None)),
                        CalcOperator::Subtract,
                        Box::new(expr)
                    ))
                },
                _ => Err(format!("Unexpected token in calc expression: {:?}", token.token_type)),
            }
        } else {
            Err("Unexpected end of input while parsing calc expression".to_string())
        }
    }

    fn parse_number_with_unit(&mut self) -> Result<(f64, Option<Unit>), String> {
        if let Some(token) = self.next_token() {
            if let TokenType::Number(num) = token.token_type {
                if let Some(next) = self.peek_token().cloned() {
                    if let TokenType::Unit(unit_str) = &next.token_type {
                        self.next_token();
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
                        return Ok((num, Some(unit)));
                    }
                }

                Ok((num, None))
            } else {
                Err(format!("Expected number, found {:?}", token.token_type))
            }
        } else {
            Err("Unexpected end of input while parsing number".to_string())
        }
    }

    fn parse_var_name(&mut self) -> Result<String, String> {
        self.next_token();

        self.expect_open_paren()?;

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

        let mut paren_depth = 1;
        while paren_depth > 0 {
            if let Some(token) = self.peek_token() {
                match &token.token_type {
                    TokenType::OpenParen => {
                        paren_depth += 1;
                        self.next_token();
                    },
                    TokenType::CloseParen => {
                        paren_depth -= 1;
                        if paren_depth > 0 {
                            self.next_token();
                        }
                    },
                    _ => {
                        self.next_token();
                    }
                }
            } else {
                return Err("Unexpected end of input while parsing var function".to_string());
            }
        }

        self.next_token();

        Ok(variable_name)
    }

    fn parse_color_function(&mut self, function_name: String) -> Result<Value, String> {
        let mut has_slash = false;
        let mut contains_var = false;
        let mut components = Vec::new();

        while let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::CloseParen => {
                    self.next_token();
                    break;
                },
                TokenType::Slash => {
                    if has_slash {
                        return Err("Multiple slashes in color function not allowed".to_string());
                    }
                    has_slash = true;
                    self.next_token();
                    continue;
                },
                TokenType::Comma => {
                    self.next_token();
                    continue;
                },
                _ => {
                    let component = self.parse_value()?;

                    if let Value::VarFunction(_, _) = &component {
                        contains_var = true;
                    }

                    components.push(component);
                }
            }
        }

        if contains_var {
            return Ok(Value::Function(function_name, components));
        }

        match function_name.as_str() {
            "rgb" | "rgba" => {
                if components.len() < 3 || components.len() > 4 {
                    return Err(format!("RGB function requires 3 or 4 components, found {}", components.len()));
                }

                let r = self.extract_component(&components[0])?;
                let g = self.extract_component(&components[1])?;
                let b = self.extract_component(&components[2])?;

                if components.len() == 4 || has_slash {
                    let alpha_idx = if has_slash { 3 } else { 3 };
                    if alpha_idx < components.len() {
                        let a = self.extract_alpha(&components[alpha_idx])?;
                        Ok(Value::Color(Color::Rgba(r, g, b, a)))
                    } else {
                        Err("Expected alpha component after slash".to_string())
                    }
                } else {
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
                    let alpha_idx = if has_slash { 3 } else { 3 };
                    if alpha_idx < components.len() {
                        let a = self.extract_alpha(&components[alpha_idx])?;
                        Ok(Value::Color(Color::Hsla(h, s, l, a)))
                    } else {
                        Err("Expected alpha component after slash".to_string())
                    }
                } else {
                    Ok(Value::Color(Color::Hsl(h, s, l)))
                }
            },
            "hwb" => {
                if components.len() < 3 {
                    return Err(format!("HWB function requires at least 3 components, found {}", components.len()));
                }

                Ok(Value::Function("hwb".to_string(), components))
            },
            "lab" => {
                if components.len() < 3 {
                    return Err(format!("LAB function requires at least 3 components, found {}", components.len()));
                }

                Ok(Value::Function("lab".to_string(), components))
            },
            "lch" => {
                if components.len() < 3 {
                    return Err(format!("LCH function requires at least 3 components, found {}", components.len()));
                }

                Ok(Value::Function("lch".to_string(), components))
            },
            "color" => {
                if components.len() < 1 {
                    return Err("Color function requires a color space parameter".to_string());
                }

                Ok(Value::Function("color".to_string(), components))
            },
            _ => {
                Err(format!("Unsupported color function: {}", function_name))
            }
        }
    }

    fn extract_component(&self, value: &Value) -> Result<u8, String> {
        match value {
            Value::Number(num, _) => {
                let clamped = num.max(0.0).min(255.0);
                Ok(clamped as u8)
            },
            _ => Err(format!("Expected number for color component, found {:?}", value)),
        }
    }

    fn extract_hue(&self, value: &Value) -> Result<u16, String> {
        match value {
            Value::Number(num, _) => {
                let wrapped = num.rem_euclid(360.0);
                Ok(wrapped as u16)
            },
            _ => Err(format!("Expected number for hue component, found {:?}", value)),
        }
    }

    fn extract_percentage(&self, value: &Value) -> Result<u8, String> {
        match value {
            Value::Number(num, Some(Unit::Percent)) => {
                let clamped = num.max(0.0).min(100.0);
                Ok(clamped as u8)
            },
            _ => Err(format!("Expected percentage for saturation/lightness component, found {:?}", value)),
        }
    }

    fn extract_alpha(&self, value: &Value) -> Result<f32, String> {
        match value {
            Value::Number(num, Some(Unit::Percent)) => {
                let alpha = num / 100.0;
                let clamped = alpha.max(0.0).min(1.0);
                Ok(clamped as f32)
            },
            Value::Number(num, _) => {
                let clamped = num.max(0.0).min(1.0);
                Ok(clamped as f32)
            },
            _ => Err(format!("Expected number or percentage for alpha component, found {:?}", value)),
        }
    }

    fn parse_var_function(&mut self) -> Result<Value, String> {
        self.expect_open_paren()?;

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

        let fallback = if let Some(token) = self.peek_token() {
            if matches!(token.token_type, TokenType::Comma) {
                self.next_token();

                if let Some(next_token) = self.peek_token() {
                    if matches!(next_token.token_type, TokenType::CloseParen) {
                        Some(Box::new(Value::Literal("".to_string())))
                    } else {
                        Some(Box::new(self.parse_function_argument()?))
                    }
                } else {
                    return Err("Unexpected end of input after comma in var function".to_string());
                }
            } else {
                None
            }
        } else {
            None
        };

        self.expect_close_paren()?;

        Ok(Value::VarFunction(variable_name, fallback))
    }

    fn parse_function_argument(&mut self) -> Result<Value, String> {
        self.parse_value()
    }

    fn parse_declaration(&mut self) -> Result<Declaration, String> {
        let mut is_custom_property = false;
        if let Some(token) = self.peek_token() {
            if let TokenType::Identifier(name) = &token.token_type {
                if name.starts_with("--") {
                    is_custom_property = true;
                }
            }
        }

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

        self.expect_colon()?;

        let value = self.parse_value_possibly_list()?;

        let mut is_important = false;
        if let Some(token) = self.peek_token() {
            if matches!(token.token_type, TokenType::ExclamationMark) {
                self.next_token();

                if let Some(token) = self.next_token() {
                    if let TokenType::Identifier(name) = token.token_type {
                        if name.to_lowercase() == "important" {
                            is_important = true;
                        } else {
                            return Err(format!("Expected 'important' after '!', found {}", name));
                        }
                    } else {
                        return Err(format!("Expected 'important' after '!', found {:?}", token.token_type));
                    }
                } else {
                    return Err("Unexpected end of input after '!'".to_string());
                }
            }
        }

        Ok(Declaration {
            property,
            value,
            is_custom_property,
            is_important,
        })
    }

    // TODO handle cases like font-family: Fira Code, Fira Mono, Menlo, Consolas, DejaVu Sans Mono, monospace; correctly
    fn parse_value_possibly_list(&mut self) -> Result<Value, String> {
        let first_value = self.parse_value()?;

        let mut values = vec![first_value];
        let mut separator = None;
        let mut current_unquoted_string = String::new();
        let mut building_unquoted_font = false;

        loop {
            if let Some(token) = self.peek_token() {
                match &token.token_type {
                    TokenType::Semicolon | TokenType::CloseBrace | TokenType::ExclamationMark => {
                        if building_unquoted_font && !current_unquoted_string.is_empty() {
                            if let Some(last) = values.last_mut() {
                                if let Value::Literal(name) = last {
                                    *name = current_unquoted_string.trim().to_string();
                                }
                            }
                        }
                        break;
                    },
                    TokenType::Comma => {
                        if building_unquoted_font && !current_unquoted_string.is_empty() {
                            if let Some(last) = values.last_mut() {
                                if let Value::Literal(name) = last {
                                    *name = current_unquoted_string.trim().to_string();
                                }
                            }
                            building_unquoted_font = false;
                            current_unquoted_string.clear();
                        }

                        if separator.is_none() {
                            separator = Some(ListSeparator::Comma);
                        }

                        self.next_token();

                        let next_value = self.parse_value()?;
                        values.push(next_value);

                        if let Value::Literal(_) = values.last().unwrap() {
                            building_unquoted_font = true;
                            if let Value::Literal(name) = values.last().unwrap() {
                                current_unquoted_string = name.clone();
                            }
                        }
                    },
                    TokenType::Identifier(ident) => {
                        if building_unquoted_font {
                            current_unquoted_string.push(' ');
                            current_unquoted_string.push_str(ident);
                            self.next_token();
                        } else {
                            if separator.is_none() {
                                separator = Some(ListSeparator::Space);
                            }

                            let result = self.parse_value();
                            match result {
                                Ok(next_value) => {
                                    values.push(next_value);
                                },
                                Err(_) => {
                                    break;
                                }
                            }
                        }
                    },
                    _ => {
                        let result = self.parse_value();
                        match result {
                            Ok(next_value) => {
                                if separator.is_none() {
                                    separator = Some(ListSeparator::Space);
                                }

                                values.push(next_value);
                            },
                            Err(_) => {
                                break;
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }

        if building_unquoted_font && !current_unquoted_string.is_empty() {
            if let Some(last) = values.last_mut() {
                if let Value::Literal(name) = last {
                    *name = current_unquoted_string.trim().to_string();
                }
            }
        }

        if values.len() == 1 {
            Ok(values.remove(0))
        } else {
            Ok(Value::List(values, separator.unwrap_or(ListSeparator::Space)))
        }
    }
    
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
