use super::common::{compare_values, parse_test_file};
use crate::css_parser::ast::ListSeparator::Space;
use crate::css_parser::ast::Value::{Function, List, Literal, Number};
use crate::css_parser::ast::{
    CalcExpression, CalcOperator, Color, RuleExt, StylesheetExt, Unit, Value,
};

#[test]
fn test_color_values() {
    let stylesheet = parse_test_file("functions.css").unwrap();

    let rule = stylesheet.get_rule_by_selector(".colors").unwrap();
    let declarations = rule.get_declarations("color");

    // rgb(255, 0, 0)
    let decl = declarations.get(0).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "rgb".to_string(),
            vec![
                Number(255f64, None),
                Literal(",".to_string()),
                Number(0f64, None),
                Literal(",".to_string()),
                Number(0f64, None),
            ],
        )
    ));

    // rgba(255, 0, 0, 0.5)
    let decl = declarations.get(1).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "rgba".to_string(),
            vec![
                Number(255f64, None),
                Literal(",".to_string()),
                Number(0f64, None),
                Literal(",".to_string()),
                Number(0f64, None),
                Literal(",".to_string()),
                Number(0.5f64, None),
            ],
        )
    ));

    // rgba(255 0 0 / 0.5)
    let decl = declarations.get(2).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "rgba".to_string(),
            vec![
                Number(255f64, None),
                Number(0f64, None),
                Number(0f64, None),
                Literal("/".to_string()),
                Number(0.5f64, None),
            ]
        )
    ));

    // rgba(255 0 0 / 1)
    let decl = declarations.get(3).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "rgba".to_string(),
            vec![
                Number(255f64, None),
                Number(0f64, None),
                Number(0f64, None),
                Literal("/".to_string()),
                Number(1f64, None),
            ]
        )
    ));

    // hsl(0, 100%, 50%)
    let decl = declarations.get(4).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "hsl".to_string(),
            vec![
                Number(0f64, None),
                Literal(",".to_string()),
                Number(100f64, Some(Unit::Percent)),
                Literal(",".to_string()),
                Number(50f64, Some(Unit::Percent)),
            ]
        )
    ));

    // hsla(0 100% 50% / 0.5)
    let decl = declarations.get(5).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "hsla".to_string(),
            vec![
                Number(0f64, None),
                Number(100f64, Some(Unit::Percent)),
                Number(50f64, Some(Unit::Percent)),
                Literal("/".to_string()),
                Number(0.5f64, None),
            ]
        )
    ));

    // oklab(0 0.5 0.5)
    let decl = declarations.get(6).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "oklab".to_string(),
            vec![
                Number(0f64, None),
                Number(0.5f64, None),
                Number(0.5f64, None),
            ]
        )
    ));

    // rgba(var(--test) / calc(4 + 8))
    let decl = declarations.get(7).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "rgba".to_string(),
            vec![
                Value::VarFunction("--test".to_string(), None),
                Literal("/".to_string()),
                Value::Calc(CalcExpression::BinaryOperation(
                    Box::new(CalcExpression::Number(4f64, None)),
                    CalcOperator::Add,
                    Box::new(CalcExpression::Number(8f64, None))
                ))
            ]
        )
    ));

    // oklab(40.1% 0.1143 0.045)
    let decl = declarations.get(8).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "oklab".to_string(),
            vec![
                Number(40.1f64, Some(Unit::Percent)),
                Number(0.1143f64, None),
                Number(0.045f64, None),
            ]
        )
    ));

    // oklab(59.69% 0.1007 0.1191)
    let decl = declarations.get(9).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "oklab".to_string(),
            vec![
                Number(59.69f64, Some(Unit::Percent)),
                Number(0.1007f64, None),
                Number(0.1191f64, None),
            ]
        )
    ));

    // oklab(59.69% 0.1007 0.1191 / 0.5)
    let decl = declarations.get(10).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "oklab".to_string(),
            vec![
                Number(59.69f64, Some(Unit::Percent)),
                Number(0.1007f64, None),
                Number(0.1191f64, None),
                Literal("/".to_string()),
                Number(0.5f64, None),
            ]
        )
    ));

    // oklab(from green l a b / 0.5)
    let decl = declarations.get(11).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "oklab".to_string(),
            vec![
                Literal("from".to_string()),
                Value::Color(Color::Named("green".to_string())),
                Literal("l".to_string()),
                Literal("a".to_string()),
                Literal("b".to_string()),
                Literal("/".to_string()),
                Number(0.5f64, None),
            ]
        )
    ));

    // oklab(from #0000FF calc(l + 0.1) a b / calc(alpha * 0.9))
    let decl = declarations.get(12).unwrap();
    // TODO l and alpha in calc expressions are not parsed correctly
    /*assert!(compare_values(
        &decl.value,
        &Function(
            "oklab".to_string(),
            vec![
                List(
                    vec![
                        Literal("from".to_string()),
                        Value::Color(Color::Hex("#0000FF".to_string())),
                        Value::Calc(CalcExpression::BinaryOperation(
                            Box::new(CalcExpression::Variable("l".to_string())),
                            CalcOperator::Add,
                            Box::new(CalcExpression::Number(0.1f64, None))
                        )),
                        Value::Keyword("a".to_string()),
                        Value::Keyword("b".to_string()),
                    ],
                    Space
                ),
                Literal("/".to_string()),
                Value::Calc(CalcExpression::BinaryOperation(
                    Box::new(CalcExpression::Variable("alpha".to_string())),
                    CalcOperator::Multiply,
                    Box::new(CalcExpression::Number(0.9f64, None))
                )),
            ]
        )
    ));*/

    // oklab(from hsl(180 100% 50%) calc(l - 0.1) a b)
    let decl = declarations.get(13).unwrap();
    // TODO l in calc expression is not parsed correctly
    /*assert!(compare_values(
        &decl.value,
        &Function(
            "oklab".to_string(),
            vec![List(
                vec![
                    Literal("from".to_string()),
                    Function(
                        "hsl".to_string(),
                        vec![List(
                            vec![
                                Number(180f64, None),
                                Number(100f64, Some(Unit::Percent)),
                                Number(50f64, Some(Unit::Percent)),
                            ],
                            Space
                        )]
                    ),
                    Value::Calc(CalcExpression::BinaryOperation(
                        Box::new(CalcExpression::Variable("l".to_string())),
                        CalcOperator::Subtract,
                        Box::new(CalcExpression::Number(0.1f64, None))
                    )),
                    Literal("a".to_string()),
                    Literal("b".to_string()),
                ],
                Space
            ),]
        )
    ));*/

    // rgba(0 0 0 / 0.5%)
    let decl = declarations.get(14).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "rgba".to_string(),
            vec![
                Number(0f64, None),
                Number(0f64, None),
                Number(0f64, None),
                Literal("/".to_string()),
                Number(0.5f64, Some(Unit::Percent)),
            ]
        )
    ));
}

#[test]
fn test_color_mix_values() {
    let stylesheet = parse_test_file("functions.css").unwrap();

    let rule = stylesheet.get_rule_by_selector(".color-mix").unwrap();
    let declarations = rule.get_declarations("color");

    // color-mix(in hsl, hsl(200 50 80), coral 80%)
    let decl = declarations.get(0).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "color-mix".to_string(),
            vec![
                Literal("in".to_string()),
                Literal("hsl".to_string()),
                Literal(",".to_string()),
                Function(
                    "hsl".to_string(),
                    vec![
                        Number(200f64, None),
                        Number(50f64, None),
                        Number(80f64, None),
                    ]
                ),
                Literal(",".to_string()),
                Value::Color(Color::Named("coral".to_string())),
                Number(80f64, Some(Unit::Percent)),
            ]
        )
    ));

    // color-mix(in lch longer hue, hsl(200deg 50% 80%), coral)
    let decl = declarations.get(1).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "color-mix".to_string(),
            vec![
                Literal("in".to_string()),
                Literal("lch".to_string()),
                Literal("longer".to_string()),
                Literal("hue".to_string()),
                Literal(",".to_string()),
                Function(
                    "hsl".to_string(),
                    vec![
                        Number(200f64, Some(Unit::Deg)),
                        Number(50f64, Some(Unit::Percent)),
                        Number(80f64, Some(Unit::Percent)),
                    ]
                ),
                Literal(",".to_string()),
                Value::Color(Color::Named("coral".to_string())),
            ]
        )
    ));

    // color-mix(in srgb, plum, #f00)
    let decl = declarations.get(2).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "color-mix".to_string(),
            vec![
                Literal("in".to_string()),
                Literal("srgb".to_string()),
                Literal(",".to_string()),
                Value::Color(Color::Named("plum".to_string())),
                Literal(",".to_string()),
                Value::Color(Color::Hex("#f00".to_string())),
            ]
        )
    ));

    // color-mix(in lab, plum 60%, #f00 50%)
    let decl = declarations.get(3).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "color-mix".to_string(),
            vec![
                Literal("in".to_string()),
                Literal("lab".to_string()),
                Literal(",".to_string()),
                Value::Color(Color::Named("plum".to_string())),
                Number(60f64, Some(Unit::Percent)),
                Literal(",".to_string()),
                Value::Color(Color::Hex("#f00".to_string())),
                Number(50f64, Some(Unit::Percent)),
            ]
        )
    ));

    // color-mix(in --swop5c, red, blue)
    let decl = declarations.get(4).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "color-mix".to_string(),
            vec![
                Literal("in".to_string()),
                Literal("--swop5c".to_string()),
                Literal(",".to_string()),
                Value::Color(Color::Named("red".to_string())),
                Literal(",".to_string()),
                Value::Color(Color::Named("blue".to_string())),
            ]
        )
    ));
}

#[test]
fn test_palette_mix_values() {
    let stylesheet = parse_test_file("functions.css").unwrap();

    let rule = stylesheet.get_rule_by_selector(".palette-mix").unwrap();
    let declarations = rule.get_declarations("font-palette");

    // palette-mix(in lch, normal, dark)
    let decl = declarations.get(0).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "palette-mix".to_string(),
            vec![
                Literal("in".to_string()),
                Literal("lch".to_string()),
                Literal(",".to_string()),
                Value::Keyword("normal".to_string()),
                Literal(",".to_string()),
                Literal("dark".to_string()),
            ]
        )
    ));

    // palette-mix(in lch, --blues, --yellows)
    let decl = declarations.get(1).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "palette-mix".to_string(),
            vec![
                Literal("in".to_string()),
                Literal("lch".to_string()),
                Literal(",".to_string()),
                Literal("--blues".to_string()),
                Literal(",".to_string()),
                Literal("--yellows".to_string()),
            ]
        )
    ));

    // palette-mix(in lch, --blues 50%, --yellows 50%)
    let decl = declarations.get(2).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "palette-mix".to_string(),
            vec![
                Literal("in".to_string()),
                Literal("lch".to_string()),
                Literal(",".to_string()),
                Literal("--blues".to_string()),
                Number(50f64, Some(Unit::Percent)),
                Literal(",".to_string()),
                Literal("--yellows".to_string()),
                Number(50f64, Some(Unit::Percent)),
            ]
        )
    ));

    // palette-mix(in hsl shorter hue, --blues, --yellows)
    let decl = declarations.get(3).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "palette-mix".to_string(),
            vec![
                Literal("in".to_string()),
                Literal("hsl".to_string()),
                Literal("shorter".to_string()),
                Literal("hue".to_string()),
                Literal(",".to_string()),
                Literal("--blues".to_string()),
                Literal(",".to_string()),
                Literal("--yellows".to_string()),
            ]
        )
    ));
}

#[test]
fn test_space_separated_functions() {
    let stylesheet = parse_test_file("functions.css").unwrap();

    let rule = stylesheet.get_rule_by_selector(".space-separated").unwrap();

    // blur(5px) brightness(120%)
    let decl = rule.get_declaration("filter").unwrap();
    assert!(compare_values(
        &decl.value,
        &List(
            vec![
                Function("blur".to_string(), vec![Number(5.0, Some(Unit::Px))]),
                Function(
                    "brightness".to_string(),
                    vec![Number(120.0, Some(Unit::Percent))]
                ),
            ],
        )
    ));

    // rotate(45deg) scale(2) translate(10px, calc(10px))
    let decl = rule.get_declaration("transform").unwrap();
    assert!(compare_values(
        &decl.value,
        &List(
            vec![
                Function("rotate".to_string(), vec![Number(45.0, Some(Unit::Deg))]),
                Function("scale".to_string(), vec![Number(2.0, None)]),
                Function(
                    "translate".to_string(),
                    vec![
                        Number(10.0, Some(Unit::Px)),
                        Literal(",".to_string()),
                        Value::Calc(CalcExpression::Number(10.0, Some(Unit::Px))),
                    ],
                ),
            ],
        )
    ));
}
