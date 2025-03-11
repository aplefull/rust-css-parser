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
        stylesheet.rules.first()?.selectors.first()?.groups.first()?.parts.first()
    }

    #[test]
    fn test_simple_class_selector() {
        let css = ".header { color: blue; }";
        let stylesheet = parse(css).unwrap();

        assert_eq!(stylesheet.rules.len(), 1);

        let rule = &stylesheet.rules[0];
        assert_eq!(rule.selectors.first().unwrap().groups[0].parts.len(), 1);
        assert_eq!(rule.selectors.first().unwrap().groups[0].parts[0], SelectorPart::Class("header".to_string()));

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
        assert_eq!(rule.selectors.first().unwrap().groups[0].parts.len(), 1);
        assert_eq!(rule.selectors.first().unwrap().groups[0].parts[0], SelectorPart::Id("main".to_string()));

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
        assert_eq!(rule.selectors.first().unwrap().groups[0].parts.len(), 1);
        assert_eq!(rule.selectors.first().unwrap().groups[0].parts[0], SelectorPart::Element("div".to_string()));

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
        assert_eq!(rule.selectors.first().unwrap().groups[0].parts.len(), 2);
        assert_eq!(rule.selectors.first().unwrap().groups[0].parts[0], SelectorPart::Element("div".to_string()));
        assert_eq!(rule.selectors.first().unwrap().groups[0].parts[1], SelectorPart::Class("container".to_string()));
    }

    #[test]
    fn test_pseudo_element() {
        let css = "div:before { content: \"Before\"; }";
        let stylesheet = parse(css).unwrap();

        let rule = &stylesheet.rules[0];
        assert_eq!(rule.selectors.first().unwrap().groups[0].parts.len(), 2);
        assert_eq!(rule.selectors.first().unwrap().groups[0].parts[0], SelectorPart::Element("div".to_string()));
        assert_eq!(rule.selectors.first().unwrap().groups[0].parts[1], SelectorPart::PseudoClass("before".to_string()));
    }

    #[test]
    fn test_custom_property() {
        let css = ":root { --primary-color: #3366ff; }";
        let stylesheet = parse(css).unwrap();

        let decl = get_first_declaration(&stylesheet).unwrap();
        assert_eq!(decl.property, "--primary-color");
        assert!(decl.is_custom_property);
        assert!(matches!(decl.value, Value::Color(Color::Hex(ref hex)) if hex == "#3366ff"));
    }

    #[test]
    fn test_var_function() {
        let css = ".header { color: var(--primary-color); }";
        let stylesheet = parse(css).unwrap();

        let decl = get_first_declaration(&stylesheet).unwrap();
        assert!(matches!(decl.value, Value::VarFunction(ref name, None) if name == "--primary-color"));
    }

    #[test]
    fn test_var_function_with_fallback() {
        let css = "#main { padding: var(--spacing-unit, 8px); }";
        let stylesheet = parse(css).unwrap();

        let decl = get_first_declaration(&stylesheet).unwrap();
        assert!(matches!(decl.value, Value::VarFunction(ref name, Some(_)) if name == "--spacing-unit"));
    }

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

    #[test]
    fn test_color_values() {
        let css = r#"
        .colors {
            color: blue;
            color: #ff0000;
            color: #000;
            color: rgb(255, 0, 0);
            color: rgba(255, 0, 0, 0.5);
            color: rgba(255 0 0 / 0.5);
            color: rgba(255 0 0 / 1);
            color: hsl(0, 100%, 50%);
            color: hsla(0 100% 50% / 0.5);
            color: transparent;
            color: currentColor;
            color: inherit;
            //color: oklab(0 0.5 0.5);
        }
        "#;
        let stylesheet = parse(css).unwrap();

        let rule = &stylesheet.rules[0];
        assert_eq!(rule.declarations.len(), 12);

        let color = &rule.declarations[0];
        assert_eq!(color.property, "color");
        assert!(matches!(color.value, Value::Color(Color::Named(ref name)) if name == "blue"));

        let hex = &rule.declarations[1];
        assert_eq!(hex.property, "color");
        assert!(matches!(hex.value, Value::Color(Color::Hex(ref hex)) if hex == "#ff0000"));

        let short_hex = &rule.declarations[2];
        assert_eq!(short_hex.property, "color");
        assert!(matches!(short_hex.value, Value::Color(Color::Hex(ref hex)) if hex == "#000"));

        let rgb = &rule.declarations[3];
        assert_eq!(rgb.property, "color");
        assert!(matches!(rgb.value, Value::Color(Color::Rgb(255, 0, 0))));

        let rgba = &rule.declarations[4];
        assert_eq!(rgba.property, "color");
        assert!(matches!(rgba.value, Value::Color(Color::Rgba(255, 0, 0, 0.5))));

        let rgba_percent = &rule.declarations[5];
        assert_eq!(rgba_percent.property, "color");
        assert!(matches!(rgba_percent.value, Value::Color(Color::Rgba(255, 0, 0, 0.5))));

        let rgba_percent2 = &rule.declarations[6];
        assert_eq!(rgba_percent2.property, "color");
        assert!(matches!(rgba_percent2.value, Value::Color(Color::Rgba(255, 0, 0, 1.0))));

        let hsl = &rule.declarations[7];
        assert_eq!(hsl.property, "color");
        assert!(matches!(hsl.value, Value::Color(Color::Hsl(0, 100, 50))));

        let hsla = &rule.declarations[8];
        assert_eq!(hsla.property, "color");
        assert!(matches!(hsla.value, Value::Color(Color::Hsla(0, 100, 50, 0.5))));

        let transparent = &rule.declarations[9];
        assert_eq!(transparent.property, "color");
        assert!(matches!(transparent.value, Value::Color(Color::Named(ref name)) if name == "transparent"));

        let current_color = &rule.declarations[10];
        assert_eq!(current_color.property, "color");
        assert!(matches!(current_color.value, Value::Literal(ref name) if name == "currentColor"));

        let inherit = &rule.declarations[11];
        assert_eq!(inherit.property, "color");
        assert!(matches!(inherit.value, Value::Keyword(ref name) if name == "inherit"));
    }

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

    #[test]
    fn test_calc() {
        let css = r#"
        .nightmare-calc {
    width: calc(
            min(
            100% - 2 * max(var(--sidebar-width, 300px), 20vw),
            (100vw - 4rem) / 2
            ) +
            clamp(
            1rem,
            (2vh + 1vw) * 0.5,
            3rem
            ) -
            (var(--padding, 8px) * (1 + var(--scale-factor, 0.25)))
    );

    margin: calc(((10px + 2em) * 1.5) / (3vh - 1rem) + max(5%, 10px) - var(--margin, 2rem));

    transform: translate(
            calc(50% + min(var(--offset-x, 10px), 5vw) * (var(--direction, -1))),
            calc(clamp(0px, var(--offset-y, 20px) / 2, 50px) - 10px)
    );

    grid-template-columns: calc(20% + 1fr * (2 / 3)) calc(1fr - 0.5 * var(--gap, 16px)) auto;
}
        "#;
        let stylesheet = parse(css).unwrap();

        assert_eq!(stylesheet.rules.len(), 1);
        let rule = &stylesheet.rules[0];

        assert_eq!(rule.selectors.first().unwrap().groups.first().unwrap().parts.len(), 1);
        assert_eq!(rule.selectors.first().unwrap().groups.first().unwrap().parts[0], SelectorPart::Class("nightmare-calc".to_string()));

        assert_eq!(rule.declarations.len(), 4);

        let width = &rule.declarations[0];
        assert_eq!(width.property, "width");
        assert!(matches!(width.value, Value::Calc(_)));

        let margin = &rule.declarations[1];
        assert_eq!(margin.property, "margin");
        assert!(matches!(margin.value, Value::Calc(_)));

        let transform = &rule.declarations[2];
        assert_eq!(transform.property, "transform");
        assert!(matches!(transform.value, Value::Function(ref name, _) if name == "translate"));

        let grid = &rule.declarations[3];
        assert_eq!(grid.property, "grid-template-columns");

        assert!(matches!(grid.value, Value::List(ref items, ListSeparator::Space) if items.len() == 3));

        // TODO
    }

    #[test]
    fn test_misc() {
        let css = r#"
        .misc {
            src: url(/_next/static/media/KaTeX_AMS-Regular.a79f1c31.woff2) format("woff2");
        }
        "#;

        let stylesheet = parse(css).unwrap();

        let decl = get_first_declaration(&stylesheet).unwrap();
        assert_eq!(decl.property, "src");
        
        match &decl.value {
            Value::List(items, ListSeparator::Space) => {
                assert_eq!(items.len(), 2);
                
                match &items[0] {
                    Value::Function(_, url) => {
                        match &url[0] {
                            Value::Literal(url) => {
                                assert_eq!(url, "/_next/static/media/KaTeX_AMS-Regular.a79f1c31.woff2");
                            }
                            _ => panic!("Unexpected value type"),
                        }
                    }
                    _ => panic!("Unexpected value type"),
                }
            }
            _ => panic!("Unexpected value type"),
        }
    }
}
