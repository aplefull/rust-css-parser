use super::common::parse_test_file;

#[test]
fn test_basic_rules() {
    let stylesheet = parse_test_file("basic.css").unwrap();

    assert_eq!(stylesheet.rules.len(), 1);
    assert_eq!(stylesheet.at_rules.len(), 0);
}
