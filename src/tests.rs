#[cfg(test)]
mod tests {
    use crate::css_parser::ast::*;
    use crate::css_parser::parser::CssParser;

    fn parse(css: &str) -> Result<Stylesheet, String> {
        let mut parser = CssParser::new(css.to_string());
        parser.parse_stylesheet()
    }

    fn get_first_declaration(stylesheet: &Stylesheet) -> Option<&Declaration> {
        stylesheet.rules.first()?.declarations.first()
    }

    fn get_first_selector_part(stylesheet: &Stylesheet) -> Option<&SelectorPart> {
        stylesheet.rules.first()?.selector.parts.first()
    }

    #[test]
    fn test_simple_class_selector() {
        let css = ".header { color: blue; }";
        let stylesheet = parse(css).unwrap();

        assert_eq!(stylesheet.rules.len(), 1);

        let rule = &stylesheet.rules[0];
        assert_eq!(rule.selector.parts.len(), 1);
        assert_eq!(rule.selector.parts[0], SelectorPart::Class("header".to_string()));

        assert_eq!(rule.declarations.len(), 1);
        let decl = &rule.declarations[0];
        assert_eq!(decl.property, "color");
        assert!(matches!(decl.value, Value::Color(Color::Named(ref name)) if name == "blue"));
    }

    #[test]
    fn test_id_selector() {
        let css = "#main { background-color: white; }";
        let stylesheet = parse(css).unwrap();

        assert_eq!(stylesheet.rules.len(), 1);

        let rule = &stylesheet.rules[0];
        assert_eq!(rule.selector.parts.len(), 1);
        assert_eq!(rule.selector.parts[0], SelectorPart::Id("main".to_string()));

        assert_eq!(rule.declarations.len(), 1);
        let decl = &rule.declarations[0];
        assert_eq!(decl.property, "background-color");
        assert!(matches!(decl.value, Value::Color(Color::Named(ref name)) if name == "white"));
    }

    #[test]
    fn test_element_selector() {
        let css = "div { margin: 0; }";
        let stylesheet = parse(css).unwrap();

        assert_eq!(stylesheet.rules.len(), 1);

        let rule = &stylesheet.rules[0];
        assert_eq!(rule.selector.parts.len(), 1);
        assert_eq!(rule.selector.parts[0], SelectorPart::Element("div".to_string()));

        assert_eq!(rule.declarations.len(), 1);
        let decl = &rule.declarations[0];
        assert_eq!(decl.property, "margin");
        assert!(matches!(decl.value, Value::Number(num, _) if num == 0.0));
    }

    #[test]
    fn test_universal_selector() {
        let css = "* { box-sizing: border-box; }";
        let stylesheet = parse(css).unwrap();

        let selector_part = get_first_selector_part(&stylesheet).unwrap();
        assert_eq!(*selector_part, SelectorPart::Universal);
    }

    #[test]
    fn test_combined_selector() {
        let css = "div.container { max-width: 1200px; }";
        let stylesheet = parse(css).unwrap();

        let rule = &stylesheet.rules[0];
        assert_eq!(rule.selector.parts.len(), 2);
        assert_eq!(rule.selector.parts[0], SelectorPart::Element("div".to_string()));
        assert_eq!(rule.selector.parts[1], SelectorPart::Class("container".to_string()));
    }

    #[test]
    fn test_pseudo_element() {
        let css = "div:before { content: \"Before\"; }";
        let stylesheet = parse(css).unwrap();

        let rule = &stylesheet.rules[0];
        assert_eq!(rule.selector.parts.len(), 2);
        assert_eq!(rule.selector.parts[0], SelectorPart::Element("div".to_string()));
        assert_eq!(rule.selector.parts[1], SelectorPart::PseudoElement("before".to_string()));
    }

    #[test]
    fn test_custom_property() {
        let css = ":root { --primary-color: #3366ff; }";
        let stylesheet = parse(css).unwrap();

        let decl = get_first_declaration(&stylesheet).unwrap();
        assert_eq!(decl.property, "--primary-color");
        assert!(decl.is_custom_property);
        assert!(matches!(decl.value, Value::Color(Color::Hex(ref hex)) if hex == "3366ff"));
    }

    #[test]
    fn test_var_function() {
        let css = ".header { color: var(--primary-color); }";
        let stylesheet = parse(css).unwrap();

        let decl = get_first_declaration(&stylesheet).unwrap();
        assert!(matches!(decl.value, Value::VarFunction(ref name, None) if name == "--primary-color"));
    }

    /*#[test]
    fn test_var_function_with_fallback() {
        let css = "#main { padding: var(--spacing-unit, 8px); }";
        let stylesheet = parse(css).unwrap();

        let decl = get_first_declaration(&stylesheet).unwrap();
        assert!(matches!(decl.value, Value::VarFunction(ref name, Some(_)) if name == "--spacing-unit"));
    }*/

    #[test]
    fn test_numeric_values() {
        let css = ".sizes { width: 100%; height: 50vh; margin: 10px; line-height: 1.6; }";
        let stylesheet = parse(css).unwrap();

        let rule = &stylesheet.rules[0];
        assert_eq!(rule.declarations.len(), 4);

        let width = &rule.declarations[0];
        assert_eq!(width.property, "width");
        assert!(matches!(width.value, Value::Number(num, Some(Unit::Percent)) if num == 100.0));

        let height = &rule.declarations[1];
        assert_eq!(height.property, "height");
        assert!(matches!(height.value, Value::Number(num, Some(Unit::Vh)) if num == 50.0));

        let margin = &rule.declarations[2];
        assert_eq!(margin.property, "margin");
        assert!(matches!(margin.value, Value::Number(num, Some(Unit::Px)) if num == 10.0));

        let line_height = &rule.declarations[3];
        assert_eq!(line_height.property, "line-height");
        assert!(matches!(line_height.value, Value::Number(num, None) if num == 1.6));
    }

    /*#[test]
    fn test_color_values() {
        let css = ".colors { color: #f00; background-color: rgb(0, 128, 255); border-color: rgba(255, 0, 0, 0.5); }";
        let stylesheet = parse(css).unwrap();

        let rule = &stylesheet.rules[0];
        assert_eq!(rule.declarations.len(), 3);

        let color = &rule.declarations[0];
        assert_eq!(color.property, "color");
        assert!(matches!(color.value, Value::Color(Color::Hex(ref hex)) if hex == "f00"));

        let bg_color = &rule.declarations[1];
        assert_eq!(bg_color.property, "background-color");
        assert!(matches!(bg_color.value, Value::Color(Color::Rgb(r, g, b)) if r == 0 && g == 128 && b == 255));

        let border_color = &rule.declarations[2];
        assert_eq!(border_color.property, "border-color");
        assert!(matches!(border_color.value, Value::Color(Color::Rgba(r, g, b, a)) if r == 255 && g == 0 && b == 0 && a == 0.5));
    }*/

    #[test]
    fn test_function_values() {
        let css = ".transforms { transform: rotate(45deg); }";
        let stylesheet = parse(css).unwrap();

        let decl = get_first_declaration(&stylesheet).unwrap();
        assert_eq!(decl.property, "transform");
        assert!(matches!(decl.value, Value::Function(ref name, _) if name == "rotate"));
    }

    #[test]
    fn test_space_separated_functions() {
        let css = ".filters { filter: blur(5px) brightness(120%); }";
        let stylesheet = parse(css).unwrap();

        let decl = get_first_declaration(&stylesheet).unwrap();
        assert_eq!(decl.property, "filter");
        assert!(matches!(decl.value, Value::List(ref items, ListSeparator::Space) if items.len() == 2));

        if let Value::List(items, _) = &decl.value {
            assert!(matches!(items[0], Value::Function(ref name, _) if name == "blur"));
            assert!(matches!(items[1], Value::Function(ref name, _) if name == "brightness"));
        }
    }

    #[test]
    fn test_font_families() {
        let css = "body { font-family: Arial, \"Helvetica Neue\", sans-serif; }";
        let stylesheet = parse(css).unwrap();

        let decl = get_first_declaration(&stylesheet).unwrap();
        assert_eq!(decl.property, "font-family");
        assert!(matches!(decl.value, Value::List(ref items, ListSeparator::Comma) if items.len() == 3));

        if let Value::List(items, _) = &decl.value {
            assert!(matches!(items[0], Value::Literal(ref name) if name == "Arial"));
            assert!(matches!(items[1], Value::QuotedString(ref name) if name == "Helvetica Neue"));
            assert!(matches!(items[2], Value::Literal(ref name) if name == "sans-serif"));
        }
    }

    #[test]
    fn test_quoted_content() {
        let css = "div:after { content: \"This content { has } some { reserved characters }\"; }";
        let stylesheet = parse(css).unwrap();

        let decl = get_first_declaration(&stylesheet).unwrap();
        assert_eq!(decl.property, "content");
        assert!(matches!(decl.value, Value::QuotedString(ref content) if content == "This content { has } some { reserved characters }"));
    }
}
