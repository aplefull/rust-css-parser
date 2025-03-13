use super::common::parse_test_file;

#[test]
fn test_at_rules() {
    let stylesheet = parse_test_file("at_rules.css").unwrap();

    assert_eq!(stylesheet.rules.len(), 0);
    assert_eq!(stylesheet.at_rules.len(), 1);
}
