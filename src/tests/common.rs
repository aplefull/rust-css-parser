use crate::css_parser::ast::Stylesheet;
use crate::css_parser::parser::CssParser;

pub fn read_test_file(filename: &str) -> String {
    let test_dir = std::path::Path::new("src/tests/resources");
    let file_path = test_dir.join(filename);

    std::fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Failed to read test file: {}", filename))
}

pub fn parse_test_file(filename: &str) -> Result<Stylesheet, String> {
    let css = read_test_file(filename);
    let mut parser = CssParser::new(css);

    parser.parse_stylesheet()
}
