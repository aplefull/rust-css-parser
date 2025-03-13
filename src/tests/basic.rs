use super::common::parse_test_file;
use crate::css_parser::ast::{DeclarationExt, RuleExt, StylesheetExt, ValueExt};

#[test]
fn test_basic_selectors() {
    let stylesheet = parse_test_file("basic.css").unwrap();

    assert_eq!(stylesheet.rules.len(), 25);
    assert_eq!(stylesheet.at_rules.len(), 0);

    let rule = stylesheet.get_rule_by_selector(".class").unwrap();
    let value = rule.get_declaration_value("color").unwrap();
    assert!(value.is("red"));

    let rule = stylesheet.get_rule_by_selector("#id").unwrap();
    let value = rule.get_declaration_value("background-color").unwrap();
    assert!(value.is("white"));

    let rule = stylesheet.get_rule_by_selector("element").unwrap();
    let value = rule.get_declaration_value("font-size").unwrap();
    assert!(value.is("12px"));

    let rule = stylesheet.get_rule_by_selector("*").unwrap();
    let value = rule.get_declaration_value("box-sizing").unwrap();
    assert!(value.is("border-box"));

    let rule = stylesheet
        .get_rule_by_selector("[attribute=\"value\"]")
        .unwrap();
    let value = rule.get_declaration_value("display").unwrap();
    assert!(value.is("block"));
}

#[test]
fn test_numeric_values_class() {
    let stylesheet = parse_test_file("basic.css").unwrap();

    let rule = stylesheet.get_rule_by_selector(".numeric-values").unwrap();

    let value = rule.get_declaration_value("width").unwrap();
    assert!(value.is("100px"));

    let value = rule.get_declaration_value("height").unwrap();
    assert!(value.is("50vh"));

    let value = rule.get_declaration_value("margin").unwrap();
    assert!(value.is("10px"));

    let value = rule.get_declaration_value("padding").unwrap();
    assert!(value.is("5vw"));

    let value = rule.get_declaration_value("font-size").unwrap();
    assert!(value.is("12pt"));

    let value = rule.get_declaration_value("border-width").unwrap();
    assert!(value.is("2mm"));

    let value = rule.get_declaration_value("outline-width").unwrap();
    assert!(value.is("0.5cm"));

    let value = rule.get_declaration_value("letter-spacing").unwrap();
    assert!(value.is("0.1in"));

    let value = rule.get_declaration_value("word-spacing").unwrap();
    assert!(value.is("3q"));

    let value = rule.get_declaration_value("max-width").unwrap();
    assert!(value.is("80%"));

    let value = rule.get_declaration_value("min-height").unwrap();
    assert!(value.is("10em"));

    let value = rule.get_declaration_value("line-height").unwrap();
    assert!(value.is("1.6"));

    let value = rule.get_declaration_value("text-indent").unwrap();
    assert!(value.is("2ex"));

    let value = rule.get_declaration_value("min-width").unwrap();
    assert!(value.is("10vmin"));

    let value = rule.get_declaration_value("max-height").unwrap();
    assert!(value.is("20vmax"));

    let value = rule.get_declaration_value("flex-basis").unwrap();
    assert!(value.is("30fr"));

    let value = rule.get_declaration_value("animation-duration").unwrap();
    assert!(value.is("2s"));

    let value = rule.get_declaration_value("transition-duration").unwrap();
    assert!(value.is("500ms"));
}

#[test]
fn test_custom_properties() {
    let stylesheet = parse_test_file("basic.css").unwrap();

    let rule = stylesheet
        .get_rule_by_selector(".custom-properties")
        .unwrap();

    for declaration in &rule.declarations {
        assert_eq!(declaration.is_custom_property, true);
    }

    let value = rule.get_declaration_value("--primary-color").unwrap();
    assert!(value.is("#3366ff"));

    let value = rule.get_declaration_value("--secondary-color").unwrap();
    assert!(value.is("#ff6633"));

    let value = rule.get_declaration_value("--text-content").unwrap();
    assert!(value.is("Hello, world;"));
}

#[test]
fn test_combined_selectors() {
    let stylesheet = parse_test_file("basic.css").unwrap();

    let rule = stylesheet.get_rule_by_selector("div.container").unwrap();
    let value = rule.get_declaration_value("max-width").unwrap();
    assert!(value.is("1200px"));

    let rule = stylesheet
        .get_rule_by_selector("header#main-header")
        .unwrap();
    let value = rule.get_declaration_value("position").unwrap();
    assert!(value.is("sticky"));

    let rule = stylesheet
        .get_rule_by_selector("input[type=\"text\"]")
        .unwrap();
    let value = rule.get_declaration_value("border-radius").unwrap();
    assert!(value.is("4px"));

    let rule = stylesheet
        .get_rule_by_selector(".navigation#primary")
        .unwrap();
    let value = rule.get_declaration_value("background-color").unwrap();
    assert!(value.is("#333333"));

    // TODO
    /*
    let rule = stylesheet.get_rule_by_selector("nav li").unwrap();
    let value = rule.get_declaration_value("list-style").unwrap();
    assert!(value.is("none"));*/

    let rule = stylesheet.get_rule_by_selector("ul > li").unwrap();
    let value = rule.get_declaration_value("margin-bottom").unwrap();
    assert!(value.is("10px"));

    let rule = stylesheet.get_rule_by_selector("h2 + p").unwrap();
    let value = rule.get_declaration_value("font-weight").unwrap();
    assert!(value.is("bold"));

    let rule = stylesheet.get_rule_by_selector("h1 ~ p").unwrap();
    let value = rule.get_declaration_value("color").unwrap();
    assert!(value.is("#666666"));

    /*let rule = stylesheet.get_rule_by_selector("header nav > ul.menu li a").unwrap();
    let value = rule.get_declaration_value("text-decoration").unwrap();
    assert!(value.is("none"));*/

    let rule = stylesheet.get_rule_by_selector(".btn.btn-primary").unwrap();
    let value = rule.get_declaration_value("background-color").unwrap();
    assert!(value.is("blue"));

    let rule = stylesheet
        .get_rule_by_selector("a[href^=\"https\"]")
        .unwrap();
    let value = rule.get_declaration_value("color").unwrap();
    assert!(value.is("green"));

    let rule = stylesheet
        .get_rule_by_selector("a[href$=\".pdf\"]")
        .unwrap();
    let value = rule.get_declaration_value("color").unwrap();
    assert!(value.is("red"));

    let rule = stylesheet
        .get_rule_by_selector("a[href*=\"example\"]")
        .unwrap();
    let value = rule.get_declaration_value("font-style").unwrap();
    assert!(value.is("italic"));

    let rule = stylesheet.get_rule_by_selector("a:hover").unwrap();
    let value = rule.get_declaration_value("text-decoration").unwrap();
    assert!(value.is("underline"));

    let rule = stylesheet.get_rule_by_selector("p::first-line").unwrap();
    let value = rule.get_declaration_value("font-variant").unwrap();
    assert!(value.is("small-caps"));

    let rule = stylesheet.get_rule_by_selector(".content:before").unwrap();
    let value = rule.get_declaration_value("content").unwrap();
    assert!(value.is("Reserved characters: .{} !important a > b '' `` \\\"\\\""));

    /*let rule = stylesheet.get_rule_by_selector("p:nth-child(2n+1)").unwrap();
    let value = rule.get_declaration_value("background-color").unwrap();
    assert!(value.is("#f5f5f5"));*/
}
