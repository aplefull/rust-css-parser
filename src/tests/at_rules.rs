use super::common::{compare_values, parse_test_file};
use crate::css_parser::ast::{AtRuleType, Unit, Value, ValueExt};
use pretty_assertions::assert_eq;

#[test]
fn test_at_rules() {
    let stylesheet = parse_test_file("at_rules.css").unwrap();

    assert_eq!(stylesheet.rules.len(), 0);
    assert_eq!(stylesheet.at_rules.len(), 2);

    let import_rule = &stylesheet.at_rules[0];
    assert_eq!(import_rule.rule_type, AtRuleType::Import);
    assert_eq!(import_rule.name, "import");
    assert_eq!(import_rule.query, "\"style.css\"");

    // TODO fix query serialization
    let media_rule = &stylesheet.at_rules[1];
    assert_eq!(media_rule.rule_type, AtRuleType::Media);
    assert_eq!(media_rule.name, "media");
    /*assert_eq!(media_rule.query, "screen and (max-width: 959px)");*/
    assert_eq!(media_rule.rules.len(), 0);
    assert_eq!(media_rule.at_rules.len(), 1);

    let viewport_rule = &media_rule.at_rules[0];
    assert_eq!(viewport_rule.rule_type, AtRuleType::Viewport);
    assert_eq!(viewport_rule.name, "-ms-viewport");
    assert_eq!(viewport_rule.query, "");
    assert_eq!(viewport_rule.at_rules.len(), 0);
    assert_eq!(viewport_rule.rules.len(), 1);

    let inner_rule = &viewport_rule.rules[0];
    assert_eq!(inner_rule.selectors.len(), 1);
    assert_eq!(inner_rule.declarations.len(), 1);

    let declaration = &inner_rule.declarations[0];
    assert_eq!(declaration.property, "width");
    assert!(&declaration.value.is("768px"));
    assert_eq!(declaration.is_important, true);
}
