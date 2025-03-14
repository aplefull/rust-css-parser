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

        let css = ".misc { display: grid !important; }";
        let stylesheet = parse(css).unwrap();

        let decl = get_first_declaration(&stylesheet).unwrap();
        assert_eq!(decl.property, "display");
        assert!(decl.is_important);

        // TODO .size2{font-size:calc(.28929605em + .2 / .00)}
        // TODO .size2{font-size: rgba(0 0 0 / 0.5%);}
        // TODO --tw-ring-inset: var(--tw-empty, /*!*/ /*!*/);
        // TODO box-shadow:rgba(217,217,217,0.2) 0px 0px 5px,rgba(217,217,217,0.25) 0px 1px 4px 1px
        // TODO     background-image: -webkit-linear-gradient(top, rgba(0, 0, 0, .75) 0, transparent 100%);
        //     background-image: -webkit-gradient(linear, left top, left bottom, color-stop(0, rgba(0, 0, 0, .75)), to(transparent));
        //     background-image: linear-gradient(to bottom, rgba(0, 0, 0, .75) 0, transparent 100%);
        // TODO src: url(https://fonts.gstatic.com/s/robotomono/v23/L0xuDF4xlVMF-BfR8bXMIhJHg45mwgGEFl0_3vq_SeW4Ep0.woff2) format('woff2');
        //     // base64
        //     src: url(data:application/font-woff2;base64,d09GMgABAAAAA);
        //     // base64 svg
        //     background: url(data:image/svg+xml;base64,PD94b++Cg==);
        //      test: url("_/next/index.css");
        // TODO unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+0304, U+0308, U+0329, U+2000-206F, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
        //  unicode-range: U+26; /* single code point */
        //  unicode-range: U+0-7F;
        //  unicode-range: U+0025-00FF; /* code point range */
        //  unicode-range: U+4??; /* wildcard range */
        //  unicode-range: U+0025-00FF, U+4??; /* multiple values */
        // TODO @media screen and (max-width: 959px) {
        //     @-ms-viewport {
        //         width: 768px
        //     }
        // }
        // TODO  color: rgba(var(--test) / calc(4 + 8));
        //     color: oklab(40.1% 0.1143 0.045);
        //     color: oklab(59.69% 0.1007 0.1191);
        //     color: oklab(59.69% 0.1007 0.1191 / 0.5);
        //     color: oklab(from green l a b / 0.5);
        //     color: oklab(from #0000FF calc(l + 0.1) a b / calc(alpha * 0.9));
        //     color: oklab(from hsl(180 100% 50%) calc(l - 0.1) a b);
        //     /* color-mix(in <polar-color-space>, <color>, <color> <percentage>) */
        //     color: color-mix(in hsl, hsl(200 50 80), coral 80%);
        //      /* color-mix(in <polar-color-space> <hue-interpolation-method>, <color>, <color>) */
        //  color: color-mix(in lch longer hue, hsl(200deg 50% 80%), coral);
        //      /* color-mix(in <rectangular-color-space>, <color>, <color>) */
        //  color: color-mix(in srgb, plum, #f00);
        //      /* color-mix(in <rectangular-color-space>, <color> <percentage>, <color> <percentage> */
        //  color: color-mix(in lab, plum 60%, #f00 50%);
        //         /* color-mix(in <custom-color-space>, <color>, <color>) */
        //     color: color-mix(in --swop5c, red, blue);
        //     /* Blending font-defined palettes */
        //     font-palette: palette-mix(in lch, normal, dark);
        //     /* Blending author-defined palettes */
        //     font-palette: palette-mix(in lch, --blues, --yellows);
        //     /* Varying percentage of each palette mixed */
        //     font-palette: palette-mix(in lch, --blues 50%, --yellows 50%);
        //     font-palette: palette-mix(in lch, --blues 70%, --yellows 30%);
        //     /* Varying color interpolation method */
        //     font-palette: palette-mix(in srgb, --blues, --yellows);
        //     font-palette: palette-mix(in hsl, --blues, --yellows);
        //     font-palette: palette-mix(in hsl shorter hue, --blues, --yellows);
        // TODO .\!container {
        //     max-width: 1536px !important
        //     }
        //     .one\:two {
        //     }
        // TODO .prose :where(ol[type=I s]):not(:where([class~=not-prose] *)) {
        //     list-style-type: upper-roman
        //          }
        // TODO     box-shadow: var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow) !important
        // TODO .shadow-\[0_4px_24px_0_hsl\(var\(--always-black\)\/1\.57\%\)\2c 0_4px_32px_0_hsl\(var\(--always-black\)\/1\.57\%\)\2c 0_2px_64px_0_hsl\(var\(--always-black\)\/1\.18\%\)\2c 0_16px_32px_0_hsl\(var\(--always-black\)\/1\.18\%\)\]
    }
}
