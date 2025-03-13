use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeOperator {
    Exists,         // [attr]
    Equals,         // [attr=value]
    Includes,       // [attr~=value]
    DashMatch,      // [attr|=value]
    StartsWith,     // [attr^=value]
    EndsWith,       // [attr$=value]
    Contains,       // [attr*=value]
}

#[derive(Debug, Clone, PartialEq)]
pub enum CaseSensitivity {
    Sensitive,
    Insensitive,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectorPart {
    Class(String),
    Id(String),
    Element(String),
    Universal,
    PseudoClass(String),
    PseudoClassFunction(String, String),
    PseudoElement(String),
    AttributeSelector(String, Option<(AttributeOperator, String, Option<CaseSensitivity>)>),
}

impl fmt::Display for SelectorPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelectorPart::Class(name) => write!(f, ".{}", name),
            SelectorPart::Id(name) => write!(f, "#{}", name),
            SelectorPart::Element(name) => write!(f, "{}", name),
            SelectorPart::Universal => write!(f, "*"),
            SelectorPart::PseudoClass(name) => write!(f, ":{}", name),
            SelectorPart::PseudoClassFunction(name, args) => write!(f, ":{}({})", name, args),
            SelectorPart::PseudoElement(name) => write!(f, "::{}", name),
            SelectorPart::AttributeSelector(attr, None) => write!(f, "[{}]", attr),
            SelectorPart::AttributeSelector(attr, Some((op, value, case_sensitivity))) => {
                match op {
                    AttributeOperator::Exists => write!(f, "[{}", attr),
                    AttributeOperator::Equals => write!(f, "[{}=\"{}\"", attr, value),
                    AttributeOperator::Includes => write!(f, "[{}~=\"{}\"", attr, value),
                    AttributeOperator::DashMatch => write!(f, "[{}|=\"{}\"", attr, value),
                    AttributeOperator::StartsWith => write!(f, "[{}^=\"{}\"", attr, value),
                    AttributeOperator::EndsWith => write!(f, "[{}$=\"{}\"", attr, value),
                    AttributeOperator::Contains => write!(f, "[{}*=\"{}\"", attr, value),
                }?;

                if let Some(sensitivity) = case_sensitivity {
                    match sensitivity {
                        CaseSensitivity::Sensitive => write!(f, " s"),
                        CaseSensitivity::Insensitive => write!(f, " i"),
                    }?;
                }

                write!(f, "]")?;

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectorCombinator {
    Descendant,      // Space
    Child,           // >
    AdjacentSibling, // +
    GeneralSibling,  // ~
}

impl fmt::Display for SelectorCombinator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelectorCombinator::Descendant => write!(f, " "),
            SelectorCombinator::Child => write!(f, " > "),
            SelectorCombinator::AdjacentSibling => write!(f, " + "),
            SelectorCombinator::GeneralSibling => write!(f, " ~ "),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectorGroup {
    pub parts: Vec<SelectorPart>,
}

impl fmt::Display for SelectorGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for part in &self.parts {
            write!(f, "{}", part)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    pub groups: Vec<SelectorGroup>,
    pub combinators: Vec<SelectorCombinator>,
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.groups.is_empty() {
            return Ok(());
        }

        write!(f, "{}", self.groups[0])?;

        for i in 0..self.combinators.len() {
            if i < self.groups.len() - 1 {
                write!(f, "{}{}", self.combinators[i], self.groups[i + 1])?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum AtRuleType {
    Media,
    Keyframes,
    Import,
    FontFace,
    Supports,
    Charset,
    Namespace,
    Page,
    CounterStyle,
    Property,
    Layer,
    FontFeatureValues,
    Viewport,
    Unknown(String),
}

#[derive(Debug)]
pub struct AtRule {
    pub rule_type: AtRuleType,
    pub name: String,
    pub query: String,
    pub rules: Vec<Rule>,
    pub at_rules: Vec<AtRule>,
}

impl fmt::Display for AtRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}", self.name)?;

        if !self.query.is_empty() {
            write!(f, " {}", self.query)?;
        }

        match self.rule_type {
            AtRuleType::Charset | AtRuleType::Import | AtRuleType::Namespace => {
                writeln!(f, ";")
            },
            _ => {
                writeln!(f, " {{")?;

                for at_rule in &self.at_rules {
                    let at_rule_str = format!("{}", at_rule);
                    for line in at_rule_str.lines() {
                        writeln!(f, "    {}", line)?;
                    }
                }

                for rule in &self.rules {
                    let rule_str = format!("{}", rule);
                    for line in rule_str.lines() {
                        writeln!(f, "    {}", line)?;
                    }
                }

                write!(f, "}}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    Px,
    Em,
    Rem,
    Percent,
    Vh,
    Vw,
    Pt,
    Cm,
    Mm,
    In,
    Deg,
    Rad,
    Fr,
    S,
    Ms,
    None,
    Other(String),
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Px => write!(f, "px"),
            Unit::Em => write!(f, "em"),
            Unit::Rem => write!(f, "rem"),
            Unit::Percent => write!(f, "%"),
            Unit::Vh => write!(f, "vh"),
            Unit::Vw => write!(f, "vw"),
            Unit::Pt => write!(f, "pt"),
            Unit::Cm => write!(f, "cm"),
            Unit::Mm => write!(f, "mm"),
            Unit::In => write!(f, "in"),
            Unit::Deg => write!(f, "deg"),
            Unit::Rad => write!(f, "rad"),
            Unit::Fr => write!(f, "fr"),
            Unit::S => write!(f, "s"),
            Unit::Ms => write!(f, "ms"),
            Unit::None => write!(f, ""),
            Unit::Other(text) => write!(f, "{}", text),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Hex(String),                    // #fff, #ff0000
    Rgb(u8, u8, u8),                // rgb(255, 0, 0)
    Rgba(u8, u8, u8, f32),          // rgba(255, 0, 0, 0.5)
    Hsl(u16, u8, u8),               // hsl(0, 100%, 50%)
    Hsla(u16, u8, u8, f32),         // hsla(0, 100%, 50%, 0.5)
    Named(String),                  // red, blue, transparent
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Hex(hex) => write!(f, "{}", hex),
            Color::Rgb(r, g, b) => write!(f, "rgb({}, {}, {})", r, g, b),
            Color::Rgba(r, g, b, a) => write!(f, "rgba({}, {}, {}, {})", r, g, b, a),
            Color::Hsl(h, s, l) => write!(f, "hsl({}, {}%, {}%)", h, s, l),
            Color::Hsla(h, s, l, a) => write!(f, "hsla({}, {}%, {}%, {})", h, s, l, a),
            Color::Named(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListSeparator {
    Space,
    Comma,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CalcOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for CalcOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcOperator::Add => write!(f, " + "),
            CalcOperator::Subtract => write!(f, " - "),
            CalcOperator::Multiply => write!(f, " * "),
            CalcOperator::Divide => write!(f, " / "),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CalcExpression {
    Number(f64, Option<Unit>),
    Variable(String),
    BinaryOperation(Box<CalcExpression>, CalcOperator, Box<CalcExpression>),
    Function(String, Vec<CalcExpression>),
    Parenthesized(Box<CalcExpression>),
}

impl fmt::Display for CalcExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcExpression::Number(num, None) => write!(f, "{}", num),
            CalcExpression::Number(num, Some(unit)) => write!(f, "{}{}", num, unit),
            CalcExpression::Variable(name) => write!(f, "var({})", name),
            CalcExpression::BinaryOperation(left, op, right) => write!(f, "{}{}{}", left, op, right),
            CalcExpression::Function(name, args) => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            },
            CalcExpression::Parenthesized(expr) => write!(f, "({})", expr),
        }
    }
}

pub trait ValueExt {
    fn is(&self, value: &str) -> bool;
    fn is_value(&self, value: &Value) -> bool;
    fn is_function(&self, name: &str, arguments: Vec<Value>) -> bool;
    fn is_variable(&self, name: &str, fallback: Option<Box<Value>>) -> bool;
}

impl ValueExt for Value {
    fn is(&self, value: &str) -> bool {
        match self {
            Value::Keyword(keyword) => keyword == value,
            Value::Literal(text) => text == value,
            Value::QuotedString(text) => text == value,
            Value::Color(color) => {
                match color {
                    Color::Named(name) => name == value,
                    Color::Hex(hex) => hex == value,
                    _ => false
                }
            },
            Value::Number(num, unit) => {
                let value_str = if let Some(unit) = unit {
                    format!("{}{}", num, unit)
                } else {
                    num.to_string()
                };
                
                value_str == value
            }
            v => {
                panic!("Value::is() called on unsupported value type: {:?}", v);
            }
        }
    }
    
    fn is_value(&self, value: &Value) -> bool {
        match self {
            Value::Keyword(keyword) => {
                if let Value::Keyword(other_keyword) = value {
                    keyword == other_keyword
                } else {
                    false
                }
            },
            Value::Literal(text) => {
                if let Value::Literal(other_text) = value {
                    text == other_text
                } else {
                    false
                }
            },
            Value::QuotedString(text) => {
                if let Value::QuotedString(other_text) = value {
                    text == other_text
                } else {
                    false
                }
            },
            Value::Color(color) => {
                if let Value::Color(other_color) = value {
                    color == other_color
                } else {
                    false
                }
            },
            Value::Number(num, unit) => {
                if let Value::Number(other_num, other_unit) = value {
                    num == other_num && unit == other_unit
                } else {
                    false
                }
            },
            v => {
                panic!("Value::is_value() called on unsupported value type: {:?}", v);
            }
        }
    }
    
    fn is_function(&self, name: &str, arguments: Vec<Value>) -> bool {
        match self {
            Value::Function(func_name, func_args) => {
                if func_name != name {
                    return false;
                }

                if func_args.len() != arguments.len() {
                    return false;
                }

                for (i, arg) in func_args.iter().enumerate() {
                    if !arg.is_value(&arguments[i]) {
                        return false;
                    }
                }

                true
            },
            v => {
                panic!("Value::is_function() called on unsupported value type: {:?}", v);
            }
        }
    }
    
    fn is_variable(&self, name: &str, fallback: Option<Box<Value>>) -> bool {
        match self {
            Value::VarFunction(var_name, var_fallback) => {
                if var_name != name {
                    return false;
                }

                if let Some(fallback) = fallback {
                    if let Some(var_fallback) = var_fallback {
                        fallback.is(var_fallback.to_string().as_str())
                    } else {
                        false
                    }
                } else {
                    var_fallback.is_none()
                }
            },
            v => {
                panic!("Value::is_variable() called on unsupported value type: {:?}", v);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Literal(String),
    QuotedString(String),
    Number(f64, Option<Unit>),
    Color(Color),
    Function(String, Vec<Value>),
    VarFunction(String, Option<Box<Value>>),
    List(Vec<Value>, ListSeparator),
    Keyword(String),
    Calc(CalcExpression),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Literal(text) => write!(f, "{}", text),
            Value::QuotedString(text) => write!(f, "\"{}\"", text),
            Value::Number(num, None) => write!(f, "{}", num),
            Value::Number(num, Some(unit)) => write!(f, "{}{}", num, unit),
            Value::Color(color) => write!(f, "{}", color),
            Value::Function(name, args) => {
                write!(f, "{}(", name)?;
                let mut first = true;
                for arg in args {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                    first = false;
                }
                write!(f, ")")
            },
            Value::VarFunction(name, None) => write!(f, "var({})", name),
            Value::VarFunction(name, Some(fallback)) => write!(f, "var({}, {})", name, fallback),
            Value::List(items, ListSeparator::Space) => {
                let mut first = true;
                for item in items {
                    if !first {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                    first = false;
                }
                Ok(())
            },
            Value::List(items, ListSeparator::Comma) => {
                let mut first = true;
                for item in items {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                    first = false;
                }
                Ok(())
            },
            Value::Keyword(keyword) => write!(f, "{}", keyword),
            Value::Calc(expr) => write!(f, "calc({})", expr),
        }
    }
}

pub trait DeclarationExt {
    fn has_color(&self, color_str: &str) -> bool;
    fn color_is(&self, color_type: &str, value: &str) -> bool;
}

impl DeclarationExt for Declaration {
    fn has_color(&self, color_str: &str) -> bool {
        match &self.value {
            Value::Color(Color::Named(name)) => name == color_str,
            Value::Color(Color::Hex(hex)) => hex == color_str,
            _ => false
        }
    }

    fn color_is(&self, color_type: &str, value: &str) -> bool {
        match (color_type, &self.value) {
            ("named", Value::Color(Color::Named(name))) => name == value,
            ("hex", Value::Color(Color::Hex(hex))) => hex == value,
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub property: String,
    pub value: Value,
    pub is_custom_property: bool,
    pub is_important: bool,
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_important {
            write!(f, "{}: {} !important;", self.property, self.value)
        } else {
            write!(f, "{}: {};", self.property, self.value)
        }
    }
}

pub trait RuleExt {
    fn declaration_count(&self) -> usize;
    fn has_declaration(&self, property: &str) -> bool;
    fn get_declaration(&self, property: &str) -> Option<&Declaration>;
    fn get_declarations(&self, property: &str) -> Vec<&Declaration>;
    fn get_declaration_value(&self, property: &str) -> Option<&Value>;
}

impl RuleExt for Rule {
    fn declaration_count(&self) -> usize {
        self.declarations.len()
    }

    fn has_declaration(&self, property: &str) -> bool {
        self.declarations.iter().any(|decl| decl.property == property)
    }

    fn get_declaration(&self, property: &str) -> Option<&Declaration> {
        self.declarations.iter().find(|decl| decl.property == property)
    }
    
    fn get_declarations(&self, property: &str) -> Vec<&Declaration> {
        self.declarations.iter().filter(|decl| decl.property == property).collect()
    }
    
    fn get_declaration_value(&self, property: &str) -> Option<&Value> {
        self.get_declaration(property).map(|decl| &decl.value)
    }
}


#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, selector) in self.selectors.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", selector)?;
        }

        writeln!(f, " {{")?;
        for decl in &self.declarations {
            writeln!(f, "    {}", decl)?;
        }
        writeln!(f, "}}")
    }
}

pub trait StylesheetExt {
    fn get_rule_by_selector(&self, selector: &str) -> Option<&Rule>;
    fn has_selector(&self, selector: &str) -> bool;
}

impl StylesheetExt for Stylesheet {
    fn get_rule_by_selector(&self, selector: &str) -> Option<&Rule> {
        self.rules.iter().find(|rule| {
            rule.selectors.iter().any(|s| s.to_string() == selector)
        })
    }

    fn has_selector(&self, selector: &str) -> bool {
        self.get_rule_by_selector(selector).is_some()
    }
}

#[derive(Debug)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
    pub at_rules: Vec<AtRule>,
}

impl fmt::Display for Stylesheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rule in &self.rules {
            write!(f, "{}", rule)?;
        }

        for at_rule in &self.at_rules {
            write!(f, "{}", at_rule)?;
        }

        Ok(())
    }
}
