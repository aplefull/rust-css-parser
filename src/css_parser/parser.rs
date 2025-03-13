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

        let rule_name = if let Some(token) = self.next_token() {
            match &token.token_type {
                TokenType::Identifier(name) => name.clone(),
                _ => return Err(format!("Expected identifier after @, found {:?}", token.token_type)),
            }
        } else {
            return Err("Unexpected end of input after @".to_string());
        };

        let rule_type = if rule_name.eq_ignore_ascii_case("media") {
            AtRuleType::Media
        } else if rule_name.eq_ignore_ascii_case("keyframes")
            || rule_name.starts_with("-webkit-keyframes")
            || rule_name.starts_with("-moz-keyframes")
            || rule_name.starts_with("-o-keyframes")
            || rule_name.starts_with("-ms-keyframes") {
            AtRuleType::Keyframes
        } else if rule_name.eq_ignore_ascii_case("import") {
            AtRuleType::Import
        } else if rule_name.eq_ignore_ascii_case("font-face") {
            AtRuleType::FontFace
        } else if rule_name.eq_ignore_ascii_case("supports") {
            AtRuleType::Supports
        } else if rule_name.eq_ignore_ascii_case("charset") {
            AtRuleType::Charset
        } else if rule_name.eq_ignore_ascii_case("namespace") {
            AtRuleType::Namespace
        } else if rule_name.eq_ignore_ascii_case("page") {
            AtRuleType::Page
        } else if rule_name.eq_ignore_ascii_case("counter-style") {
            AtRuleType::CounterStyle
        } else if rule_name.eq_ignore_ascii_case("property") {
            AtRuleType::Property
        } else if rule_name.eq_ignore_ascii_case("layer") {
            AtRuleType::Layer
        } else if rule_name.eq_ignore_ascii_case("font-feature-values") {
            AtRuleType::FontFeatureValues
        } else if rule_name.eq_ignore_ascii_case("viewport")
            || rule_name.eq_ignore_ascii_case("-ms-viewport")
            || rule_name.eq_ignore_ascii_case("-webkit-viewport")
            || rule_name.eq_ignore_ascii_case("-moz-viewport") {
            AtRuleType::Viewport
        } else {
            AtRuleType::Unknown(rule_name.clone())
        };

        let simple_at_rules = [
            AtRuleType::Charset,
            AtRuleType::Import,
            AtRuleType::Namespace,
        ];

        if simple_at_rules.contains(&rule_type) {
            let mut query = String::new();

            while let Some(token) = self.peek_token() {
                match &token.token_type {
                    TokenType::Semicolon => {
                        self.next_token();
                        break;
                    },
                    _ => {
                        let token = self.next_token().unwrap();
                        match &token.token_type {
                            TokenType::String(text) => query.push_str(&format!("\"{}\"", text)),
                            TokenType::Identifier(name) => query.push_str(name),
                            _ => query.push_str(&format!("{} ", token.token_type)),
                        }
                    }
                }
            }

            return Ok(AtRule { rule_type, name: rule_name, query: query.trim().to_string(), rules: Vec::new(), at_rules: Vec::new() });
        }

        let mut query = String::new();

        while let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::OpenBrace => break,
                _ => {
                    let token = self.next_token().unwrap();
                    match &token.token_type {
                        TokenType::String(text) => query.push_str(&format!("\"{}\"", text)),
                        TokenType::Identifier(name) => query.push_str(name),
                        _ => query.push_str(&format!("{} ", token.token_type)),
                    }
                }
            }
        }

        query = query.trim().to_string();

        self.expect_open_brace()?;

        let mut rules = Vec::new();
        let mut nested_at_rules = Vec::new();

        match rule_type {
            AtRuleType::FontFace | AtRuleType::Page | AtRuleType::Property | AtRuleType::Viewport => {
                let declarations = self.parse_declarations()?;

                let rule = Rule {
                    selectors: vec![Selector { groups: vec![], combinators: vec![] }],
                    declarations,
                };

                rules.push(rule);

                self.expect_close_brace()?;
            },

            AtRuleType::Keyframes => {
                while let Some(token) = self.peek_token() {
                    match &token.token_type {
                        TokenType::CloseBrace => {
                            self.next_token();
                            break;
                        },
                        _ => {
                            let rule = self.parse_keyframe_rule()?;
                            rules.push(rule);
                        }
                    }
                }
            },

            _ => {
                while let Some(token) = self.peek_token() {
                    match &token.token_type {
                        TokenType::CloseBrace => {
                            self.next_token();
                            break;
                        },
                        TokenType::AtSymbol => {
                            let nested_at_rule = self.parse_at_rule()?;
                            nested_at_rules.push(nested_at_rule);
                        },
                        _ => {
                            let rule = self.parse_rule()?;
                            rules.push(rule);
                        }
                    }
                }
            }
        }

        let at_rule = AtRule {
            rule_type,
            name: rule_name,
            query,
            rules,
            at_rules: nested_at_rules,
        };

        Ok(at_rule)
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
                TokenType::UnicodeRange(range) => {
                    let range_clone = range.clone();
                    self.next_token();
                    Ok(Value::Literal(range_clone))
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
            match &token.token_type {
                TokenType::Identifier(name) => {
                    if name.chars().all(|c| c.is_digit(16)) {
                        Ok(Value::Color(Color::Hex(format!("#{}", name))))
                    } else {
                        Ok(Value::Literal(format!("#{}", name)))
                    }
                },
                _ => Err(format!("Expected identifier after #, found {:?}", token.token_type)),
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

        let name_lower = name.to_lowercase();

        let is_gradient =
            name_lower.ends_with("-gradient") ||
                name_lower == "-webkit-gradient" ||
                ["gradient", "linear", "radial", "conic"].iter()
                    .any(|term| name_lower.contains(term) &&
                        ["-webkit-", "-moz-", "-ms-", "-o-"].iter()
                            .any(|prefix| name_lower.starts_with(prefix)));

        if is_gradient {
            return self.parse_gradient_function(name);
        }

        if name.to_lowercase() == "calc" {
            return self.parse_calc_function();
        }

        let math_functions = ["min", "max", "clamp"];
        if math_functions.contains(&name.to_lowercase().as_str()) {
            return self.parse_css_math_function(name);
        }

        let space_separated_functions = ["drop-shadow", "box-shadow", "translate", "rotate", "scale", "rect", "translate", "scale", "rotate", "matrix", "perspective"];
        if space_separated_functions.contains(&name.to_lowercase().as_str()) {
            return self.parse_space_separated_function(name);
        }

        let color_functions = [
            "rgb", "rgba", "hsl", "hsla", "hwb", "lab", "lch", "oklab", "oklch", "color", "device-cmyk",
            "color-mix", "palette-mix"
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

    fn parse_gradient_function(&mut self, name: String) -> Result<Value, String> {
        let mut arguments = Vec::new();

        if let Some(token) = self.peek_token() {
            if let TokenType::Identifier(id) = &token.token_type {
                if id.to_lowercase() == "to" {
                    self.next_token();

                    let mut direction = String::from("to");

                    while let Some(token) = self.peek_token().cloned() {
                        match &token.token_type {
                            TokenType::Identifier(dir) => {
                                if ["top", "right", "bottom", "left"].contains(&dir.to_lowercase().as_str()) {
                                    self.next_token();
                                    direction.push_str(&format!(" {}", dir));
                                } else {
                                    break;
                                }
                            },
                            TokenType::Comma => {
                                break;
                            },
                            _ => break,
                        }
                    }

                    arguments.push(Value::Literal(direction));

                    if let Some(token) = self.peek_token() {
                        if matches!(token.token_type, TokenType::Comma) {
                            self.next_token();
                        }
                    }
                }
            }
        }

        loop {
            if let Some(token) = self.peek_token() {
                if matches!(token.token_type, TokenType::Comma) {
                    self.next_token();
                    continue;
                }
            }

            if let Some(token) = self.peek_token() {
                if matches!(token.token_type, TokenType::CloseParen) {
                    self.next_token();
                    break;
                }
            } else {
                return Err("Unexpected end of input in gradient function".to_string());
            }

            let color_stop = self.parse_gradient_color_stop()?;
            arguments.push(color_stop);
        }

        Ok(Value::Function(name, arguments))
    }

    fn parse_gradient_color_stop(&mut self) -> Result<Value, String> {
        let mut items = Vec::new();

        loop {
            if let Some(token) = self.peek_token() {
                match &token.token_type {
                    TokenType::Comma | TokenType::CloseParen => {
                        break;
                    },
                    _ => {
                        match self.parse_value() {
                            Ok(value) => {
                                items.push(value);
                            },
                            Err(_) => {
                                self.next_token();
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }

        if items.is_empty() {
            return Err("Expected at least one value in gradient color stop".to_string());
        }

        if items.len() == 1 {
            return Ok(items.remove(0));
        }

        Ok(Value::List(items, ListSeparator::Space))
    }

    fn parse_css_math_function(&mut self, name: String) -> Result<Value, String> {
        let mut arguments = Vec::new();

        loop {
            let expr = self.parse_calc_expression()?;
            arguments.push(expr);

            if let Some(token) = self.peek_token() {
                match &token.token_type {
                    TokenType::Comma => {
                        self.next_token();
                        continue;
                    },
                    TokenType::CloseParen => {
                        self.next_token();
                        break;
                    },
                    _ => return Err(format!("Expected comma or closing parenthesis in math function, found {:?}", token.token_type)),
                }
            } else {
                return Err("Unexpected end of input in math function".to_string());
            }
        }

        if arguments.is_empty() {
            return Err(format!("Function {} requires at least one argument", name));
        }

        if name.to_lowercase() == "clamp" && arguments.len() != 3 {
            return Err("clamp() function requires exactly 3 arguments: minimum, preferred, and maximum".to_string());
        }

        Ok(Value::Calc(CalcExpression::Function(name, arguments)))
    }

    fn parse_space_separated_function(&mut self, name: String) -> Result<Value, String> {
        let mut values = Vec::new();

        while let Some(token) = self.peek_token() {
            if matches!(token.token_type, TokenType::CloseParen) {
                self.next_token();
                break;
            }

            let value = self.parse_value()?;
            values.push(value);
        }

        if values.is_empty() {
            return Err("Expected at least one value in function".to_string());
        }

        let combined_args = Value::List(values, ListSeparator::Space);

        Ok(Value::Function(name, vec![combined_args]))
    }

    fn parse_url_argument(&mut self) -> Result<String, String> {
        match self.next_token() {
            Some(token) => {
                match token.token_type {
                    TokenType::String(text) => Ok(text),
                    _ => Err(format!("Expected string in url(), found {:?}", token.token_type))
                }
            },
            None => Err("Unexpected end of input while parsing url()".to_string())
        }
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
        let special_functions = ["color-mix", "palette-mix"];
        if special_functions.contains(&function_name.to_lowercase().as_str()) {
            return self.parse_special_color_function(function_name);
        }

        let mut components = Vec::new();

        let mut pre_slash_components = Vec::new();
        let mut post_slash_components = Vec::new();
        let mut has_slash = false;

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
                    if has_slash {
                        post_slash_components.push(component);
                    } else {
                        pre_slash_components.push(component);
                    }
                }
            }
        }

        let pre_slash_list = if pre_slash_components.len() > 1 {
            Value::List(pre_slash_components, ListSeparator::Space)
        } else if pre_slash_components.len() == 1 {
            pre_slash_components.remove(0)
        } else {
            Value::Literal("".to_string())
        };

        components.push(pre_slash_list);

        if has_slash {
            components.push(Value::Literal("/".to_string()));

            let post_slash_list = if post_slash_components.len() > 1 {
                Value::List(post_slash_components, ListSeparator::Space)
            } else if post_slash_components.len() == 1 {
                post_slash_components.remove(0)
            } else {
                Value::Literal("".to_string())
            };

            components.push(post_slash_list);
        }

        Ok(Value::Function(function_name, components))
    }

    fn parse_special_color_function(&mut self, function_name: String) -> Result<Value, String> {
        if let Some(token) = self.next_token() {
            match &token.token_type {
                TokenType::Identifier(word) if word.to_lowercase() == "in" => {},
                _ => return Err(format!("Expected 'in' after {}, found {:?}", function_name, token.token_type)),
            }
        } else {
            return Err(format!("Unexpected end of input after {}", function_name));
        }

        let mut color_space_components = Vec::new();

        if let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::Identifier(_) => {
                    let space = self.parse_value()?;
                    color_space_components.push(space);
                },
                _ => return Err(format!("Expected color space name, found {:?}", token.token_type)),
            }
        } else {
            return Err("Unexpected end of input while parsing color space".to_string());
        }

        while let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::Identifier(_) => {
                    let peek_token = self.peek_token().unwrap();
                    if let TokenType::Comma = peek_token.token_type {
                        break;
                    }

                    let component = self.parse_value()?;
                    color_space_components.push(component);
                },
                TokenType::Comma => {
                    break;
                },
                _ => return Err(format!("Unexpected token in color space specification: {:?}", token.token_type)),
            }
        }

        let color_space = Value::List(color_space_components, ListSeparator::Space);

        if let Some(token) = self.peek_token() {
            if !matches!(token.token_type, TokenType::Comma) {
                return Err(format!("Expected comma after color space, found {:?}", token.token_type));
            }
            self.next_token();
        } else {
            return Err("Unexpected end of input after color space".to_string());
        }

        let first_color = self.parse_color_mix_argument()?;

        if let Some(token) = self.peek_token() {
            if !matches!(token.token_type, TokenType::Comma) {
                return Err(format!("Expected comma after first color, found {:?}", token.token_type));
            }
            self.next_token();
        } else {
            return Err("Unexpected end of input after first color".to_string());
        }

        let second_color = self.parse_color_mix_argument()?;

        if let Some(token) = self.peek_token() {
            if !matches!(token.token_type, TokenType::CloseParen) {
                return Err(format!("Expected closing parenthesis, found {:?}", token.token_type));
            }
            self.next_token();
        } else {
            return Err("Unexpected end of input while parsing color-mix".to_string());
        }

        let args = vec![
            color_space,
            first_color,
            second_color
        ];

        Ok(Value::Function(function_name, args))
    }

    fn parse_color_mix_argument(&mut self) -> Result<Value, String> {
        let mut components = Vec::new();

        if let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::Identifier(name) if name.to_lowercase() == "color-mix" || name.to_lowercase() == "palette-mix" => {
                    let name_clone = name.clone();
                    self.next_token();

                    if let Some(token) = self.peek_token() {
                        if !matches!(token.token_type, TokenType::OpenParen) {
                            return Err(format!("Expected opening parenthesis after {}, found {:?}", name_clone, token.token_type));
                        }
                        self.next_token();
                    }

                    let nested_function = self.parse_special_color_function(name_clone)?;
                    components.push(nested_function);
                },
                _ => {
                    let color = self.parse_value()?;
                    components.push(color);
                }
            }
        } else {
            return Err("Unexpected end of input while parsing color mix argument".to_string());
        }

        if let Some(token) = self.peek_token() {
            if let TokenType::Number(_) = token.token_type {
                let percentage = self.parse_value()?;
                components.push(percentage);
            }
        }

        if components.len() > 1 {
            Ok(Value::List(components, ListSeparator::Space))
        } else {
            Ok(components.remove(0))
        }
    }

    fn parse_keyframe_rule(&mut self) -> Result<Rule, String> {
        let mut selectors = Vec::new();
        let first_selector = self.parse_keyframe_selector()?;
        selectors.push(first_selector);

        while let Some(token) = self.peek_token() {
            if matches!(token.token_type, TokenType::Comma) {
                self.next_token();
                let next_selector = self.parse_keyframe_selector()?;
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

    fn parse_keyframe_selector(&mut self) -> Result<Selector, String> {
        let mut group = SelectorGroup { parts: Vec::new() };

        if let Some(token) = self.peek_token() {
            match &token.token_type {
                TokenType::Identifier(name) if name.to_lowercase() == "from" || name.to_lowercase() == "to" => {
                    let name_clone = name.clone();
                    self.next_token();
                    group.parts.push(SelectorPart::Element(name_clone));
                },
                TokenType::Number(num) => {
                    let number = *num;
                    self.next_token();

                    if let Some(token) = self.peek_token() {
                        if let TokenType::Unit(unit) = &token.token_type {
                            if unit == "%" {
                                self.next_token();
                                group.parts.push(SelectorPart::Element(format!("{}%", number)));
                            } else {
                                return Err(format!("Expected % unit in keyframe selector, found {:?}", token.token_type));
                            }
                        } else {
                            return Err(format!("Expected % unit in keyframe selector, found {:?}", token.token_type));
                        }
                    } else {
                        return Err("Unexpected end of input after number in keyframe selector".to_string());
                    }
                },
                _ => return Err(format!("Expected 'from', 'to', or percentage value, found {:?}", token.token_type)),
            }
        } else {
            return Err("Unexpected end of input while parsing keyframe selector".to_string());
        }

        Ok(Selector {
            groups: vec![group],
            combinators: Vec::new(),
        })
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
