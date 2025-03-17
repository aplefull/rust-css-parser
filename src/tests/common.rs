use crate::css_parser::ast::{CalcExpression, Color, Stylesheet, Value};
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

pub fn compare_values(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Literal(a_str), Value::Literal(b_str)) => a_str == b_str,

        (Value::QuotedString(a_str), Value::QuotedString(b_str)) => a_str == b_str,

        (Value::Number(a_num, a_unit), Value::Number(b_num, b_unit)) => {
            (a_num - b_num).abs() < f64::EPSILON && a_unit == b_unit
        },

        (Value::Color(a_color), Value::Color(b_color)) => match (a_color, b_color) {
            (Color::Hex(a_hex), Color::Hex(b_hex)) => a_hex.to_lowercase() == b_hex.to_lowercase(),
            (Color::Named(a_name), Color::Named(b_name)) => a_name.to_lowercase() == b_name.to_lowercase(),
            _ => false,
        },

        (Value::Function(a_name, a_args), Value::Function(b_name, b_args)) => {
            if a_name != b_name || a_args.len() != b_args.len() {
                return false;
            }

            for (a_arg, b_arg) in a_args.iter().zip(b_args.iter()) {
                if !compare_values(a_arg, b_arg) {
                    return false;
                }
            }

            true
        },

        (Value::VarFunction(a_name, a_arg), Value::VarFunction(b_name, b_arg)) => {
            if a_name != b_name {
                return false;
            }

            match (a_arg, b_arg) {
                (Some(a_val), Some(b_val)) => compare_values(a_val, b_val),
                (None, None) => true,
                _ => false,
            }
        },

        (Value::List(a_items), Value::List(b_items)) => {
            if a_items.len() != b_items.len() {
                return false;
            }

            for (a_item, b_item) in a_items.iter().zip(b_items.iter()) {
                if !compare_values(a_item, b_item) {
                    return false;
                }
            }

            true
        },

        (Value::Keyword(a_key), Value::Keyword(b_key)) => a_key == b_key,

        (Value::Calc(a_calc), Value::Calc(b_calc)) => compare_calc_expressions(a_calc, b_calc),

        _ => false,
    }
}

fn compare_calc_expressions(a: &CalcExpression, b: &CalcExpression) -> bool {
    match (a, b) {
        (CalcExpression::Number(a_num, a_unit), CalcExpression::Number(b_num, b_unit)) => {
            (a_num - b_num).abs() < f64::EPSILON && a_unit == b_unit
        },

        (CalcExpression::Variable(a_var), CalcExpression::Variable(b_var)) => a_var == b_var,

        (CalcExpression::BinaryOperation(a_left, a_op, a_right),
            CalcExpression::BinaryOperation(b_left, b_op, b_right)) => {
            a_op == b_op &&
                compare_calc_expressions(a_left, b_left) &&
                compare_calc_expressions(a_right, b_right)
        },

        (CalcExpression::Function(a_name, a_args), CalcExpression::Function(b_name, b_args)) => {
            if a_name != b_name || a_args.len() != b_args.len() {
                return false;
            }

            for (a_arg, b_arg) in a_args.iter().zip(b_args.iter()) {
                if !compare_calc_expressions(a_arg, b_arg) {
                    return false;
                }
            }

            true
        },

        (CalcExpression::Parenthesized(a_expr), CalcExpression::Parenthesized(b_expr)) => {
            compare_calc_expressions(a_expr, b_expr)
        },

        _ => false,
    }
}

