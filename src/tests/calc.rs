use crate::css_parser::ast::StylesheetExt;
use crate::tests::common::parse_test_file;

#[test]
fn test_calc() {
    let stylesheet = parse_test_file("calc.css").unwrap();

    let rule = stylesheet.get_rule_by_selector(".calc").unwrap();

    // TODO
}
