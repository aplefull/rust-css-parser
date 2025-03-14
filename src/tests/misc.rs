use super::common::{compare_values, parse_test_file};
use crate::css_parser::ast::ListSeparator::Space;
use crate::css_parser::ast::Value::List;
use crate::css_parser::ast::{RuleExt, StylesheetExt, Value};

#[test]
fn test_url() {
    let stylesheet = parse_test_file("misc.css").unwrap();

    let rule = stylesheet.get_rule_by_selector(".urls").unwrap();
    let declarations = rule.get_declarations("src");

    let decl = declarations.get(0).unwrap();
    assert!(compare_values(
        &decl.value,
        &List(
            vec![
                Value::Function(
                    "url".to_string(),
                    vec![Value::Literal("https://fonts.gstatic.com/s/robotomono/v23/L0xuDF4xlVMF-BfR8bXMIhJHg45mwgGEFl0_3vq_SeW4Ep0.woff2".to_string())]
                ),
                Value::Function(
                    "format".to_string(),
                    vec![Value::QuotedString("woff2".to_string())]
                ),
            ],
            Space
        )
    ));

    let decl = declarations.get(1).unwrap();
    assert!(compare_values(
        &decl.value,
        &Value::Function(
            "url".to_string(),
            vec![Value::Literal(
                "data:application/font-woff2;base64,d09GMgABAAAAA".to_string()
            )]
        )
    ));

    let decl = declarations.get(2).unwrap();
    assert!(compare_values(
        &decl.value,
        &Value::Function(
            "url".to_string(),
            vec![Value::Literal(
                "data:image/svg+xml;base64,PD94b++Cg==".to_string()
            )]
        )
    ));

    let decl = declarations.get(3).unwrap();
    assert!(compare_values(
        &decl.value,
        &Value::Function(
            "url".to_string(),
            vec![Value::Literal(
                "\"/_next/static/media/KaTeX_AMS-Regular.a79f1c31.woff2\"".to_string()
            )]
        )
    ));
}
