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
    fn test_misc() {
        // TODO box-shadow:rgba(217,217,217,0.2) 0px 0px 5px,rgba(217,217,217,0.25) 0px 1px 4px 1px
        // TODO     background-image: -webkit-linear-gradient(top, rgba(0, 0, 0, .75) 0, transparent 100%);
        //     background-image: -webkit-gradient(linear, left top, left bottom, color-stop(0, rgba(0, 0, 0, .75)), to(transparent));
        //     background-image: linear-gradient(to bottom, rgba(0, 0, 0, .75) 0, transparent 100%);
        // TODO
        // TODO .prose :where(ol[type=I s]):not(:where([class~=not-prose] *)) {
        //     list-style-type: upper-roman
        //          }
        // TODO     box-shadow: var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow) !important
        // TODO --tw-skew-x: skewX(< value >);
        //     --tw-skew-y: skewY(< value >);
        //     test: alpha(opacity=50);
        // TODO background-color: color-mix(in oklab, var(--color-cyan-400) var(--my-alpha-value), transparent)
    }
}
