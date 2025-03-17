use super::common::{compare_values, parse_test_file};
use crate::css_parser::ast::ListSeparator::Comma;
use crate::css_parser::ast::Value::List;
use crate::css_parser::ast::{RuleExt, StylesheetExt, Value};

#[test]
fn test_unicode_range() {
    let stylesheet = parse_test_file("text.css").unwrap();

    let rule = stylesheet.get_rule_by_selector(".unicode-range").unwrap();
    let declarations = rule.get_declarations("unicode-range");

    let decl = declarations.get(0).unwrap();
    println!("{:?}", decl);
    assert!(compare_values(
        &decl.value,
        &List(vec![
            Value::Literal("U+0000-00FF".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+0131".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+0152-0153".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+02BB-02BC".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+02C6".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+02DA".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+02DC".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+0304".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+0308".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+0329".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+2000-206F".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+20AC".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+2122".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+2191".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+2193".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+2212".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+2215".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+FEFF".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+FFFD".to_string()),
        ],)
    ));

    let decl = declarations.get(1).unwrap();
    assert!(compare_values(
        &decl.value,
        &Value::Literal("U+26".to_string())
    ));

    let decl = declarations.get(2).unwrap();
    assert!(compare_values(
        &decl.value,
        &Value::Literal("U+0-7F".to_string())
    ));

    let decl = declarations.get(3).unwrap();
    assert!(compare_values(
        &decl.value,
        &Value::Literal("U+0025-00FF".to_string())
    ));

    let decl = declarations.get(4).unwrap();
    assert!(compare_values(
        &decl.value,
        &Value::Literal("U+4??".to_string())
    ));

    let decl = declarations.get(5).unwrap();
    assert!(compare_values(
        &decl.value,
        &List(vec![
            Value::Literal("U+0025-00FF".to_string()),
            Value::Literal(",".to_string()),
            Value::Literal("U+4??".to_string()),
        ],)
    ));
}
