use super::common::parse_test_file;
use crate::css_parser::ast::{Color, DeclarationExt, RuleExt, StylesheetExt, Value, ValueExt};

#[test]
fn test_color_values() {
    let stylesheet = parse_test_file("functions.css").unwrap();

    let rule = stylesheet.get_rule_by_selector(".colors").unwrap();
    let declarations = rule.get_declarations("color");

    // TODO
}
