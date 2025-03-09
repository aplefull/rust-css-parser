use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SelectorPart {
    Class(String),
    Id(String),
    Element(String),
    Universal,
    PseudoElement(String),
}

impl fmt::Display for SelectorPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelectorPart::Class(name) => write!(f, ".{}", name),
            SelectorPart::Id(name) => write!(f, "#{}", name),
            SelectorPart::Element(name) => write!(f, "{}", name),
            SelectorPart::Universal => write!(f, "*"),
            SelectorPart::PseudoElement(name) => write!(f, ":{}", name),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    pub parts: Vec<SelectorPart>,
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for part in &self.parts {
            write!(f, "{}", part)?;
        }
        Ok(())
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
            Color::Hex(hex) => write!(f, "#{}", hex),
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

#[derive(Debug, Clone)]
pub enum Value {
    Literal(String),                              // Basic literals
    QuotedString(String),                         // "example"
    Number(f64, Option<Unit>),                    // 10px, 2em, 1.5
    Color(Color),                                 // #333, rgb(), etc.
    Function(String, Vec<Value>),                 // calc(), linear-gradient()
    VarFunction(String, Option<Box<Value>>),      // var(--name, fallback)
    List(Vec<Value>, ListSeparator),              // space or comma-separated values
    Keyword(String),                              // inherit, auto, etc.
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
        }
    }
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub property: String,
    pub value: Value,
    pub is_custom_property: bool,
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {};", self.property, self.value)
    }
}

#[derive(Debug)]
pub struct Rule {
    pub selector: Selector,
    pub declarations: Vec<Declaration>,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {{", self.selector)?;
        for decl in &self.declarations {
            writeln!(f, "    {}", decl)?;
        }
        writeln!(f, "}}")
    }
}

#[derive(Debug)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

impl fmt::Display for Stylesheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rule in &self.rules {
            write!(f, "{}", rule)?;
        }
        Ok(())
    }
}
