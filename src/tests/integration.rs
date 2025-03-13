use super::common::parse_test_file;

#[test]
fn test_full_stylesheet_parsing() {
    let stylesheet = parse_test_file("integration-youtube-www-player.css").unwrap();

    assert_eq!(stylesheet.rules.len(), 2877);
    assert_eq!(stylesheet.at_rules.len(), 74);
}
