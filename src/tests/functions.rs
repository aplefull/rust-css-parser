use super::common::{compare_values, parse_test_file};
use crate::css_parser::ast::ListSeparator::Space;
use crate::css_parser::ast::Value::{Function, List};
use crate::css_parser::ast::{
    CalcExpression, CalcOperator, Color, RuleExt, StylesheetExt, Unit, Value,
};

#[test]
fn test_color_values() {
    let stylesheet = parse_test_file("functions.css").unwrap();

    let rule = stylesheet.get_rule_by_selector(".colors").unwrap();
    let declarations = rule.get_declarations("color");

    let decl = declarations.get(0).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "rgb".to_string(),
            vec![List(
                vec![
                    Value::Number(255f64, None),
                    Value::Number(0f64, None),
                    Value::Number(0f64, None),
                ],
                Space
            )]
        )
    ));

    let decl = declarations.get(1).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "rgba".to_string(),
            vec![List(
                vec![
                    Value::Number(255f64, None),
                    Value::Number(0f64, None),
                    Value::Number(0f64, None),
                    Value::Number(0.5f64, None),
                ],
                Space
            )]
        )
    ));

    let decl = declarations.get(2).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "rgba".to_string(),
            vec![
                List(
                    vec![
                        Value::Number(255f64, None),
                        Value::Number(0f64, None),
                        Value::Number(0f64, None),
                    ],
                    Space
                ),
                Value::Literal("/".to_string()),
                Value::Number(0.5f64, None),
            ]
        )
    ));

    let decl = declarations.get(3).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "rgba".to_string(),
            vec![
                List(
                    vec![
                        Value::Number(255f64, None),
                        Value::Number(0f64, None),
                        Value::Number(0f64, None),
                    ],
                    Space
                ),
                Value::Literal("/".to_string()),
                Value::Number(1f64, None),
            ]
        )
    ));

    let decl = declarations.get(4).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "hsl".to_string(),
            vec![List(
                vec![
                    Value::Number(0f64, None),
                    Value::Number(100f64, Some(Unit::Percent)),
                    Value::Number(50f64, Some(Unit::Percent)),
                ],
                Space
            )]
        )
    ));

    let decl = declarations.get(5).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "hsla".to_string(),
            vec![
                List(
                    vec![
                        Value::Number(0f64, None),
                        Value::Number(100f64, Some(Unit::Percent)),
                        Value::Number(50f64, Some(Unit::Percent)),
                    ],
                    Space
                ),
                Value::Literal("/".to_string()),
                Value::Number(0.5f64, None),
            ]
        )
    ));

    let decl = declarations.get(6).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "oklab".to_string(),
            vec![List(
                vec![
                    Value::Number(0f64, None),
                    Value::Number(0.5f64, None),
                    Value::Number(0.5f64, None),
                ],
                Space
            )]
        )
    ));

    let decl = declarations.get(7).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "rgba".to_string(),
            vec![
                Value::VarFunction("--test".to_string(), None),
                Value::Literal("/".to_string()),
                Value::Calc(CalcExpression::BinaryOperation(
                    Box::new(CalcExpression::Number(4f64, None)),
                    CalcOperator::Add,
                    Box::new(CalcExpression::Number(8f64, None))
                ))
            ]
        )
    ));

    let decl = declarations.get(8).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "oklab".to_string(),
            vec![List(
                vec![
                    Value::Number(40.1f64, Some(Unit::Percent)),
                    Value::Number(0.1143f64, None),
                    Value::Number(0.045f64, None),
                ],
                Space
            )]
        )
    ));

    let decl = declarations.get(9).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "oklab".to_string(),
            vec![List(
                vec![
                    Value::Number(59.69f64, Some(Unit::Percent)),
                    Value::Number(0.1007f64, None),
                    Value::Number(0.1191f64, None),
                ],
                Space
            )]
        )
    ));

    let decl = declarations.get(10).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "oklab".to_string(),
            vec![
                List(
                    vec![
                        Value::Number(59.69f64, Some(Unit::Percent)),
                        Value::Number(0.1007f64, None),
                        Value::Number(0.1191f64, None),
                    ],
                    Space
                ),
                Value::Literal("/".to_string()),
                Value::Number(0.5f64, None),
            ]
        )
    ));

    let decl = declarations.get(11).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "oklab".to_string(),
            vec![
                List(
                    vec![
                        Value::Literal("from".to_string()),
                        Value::Color(Color::Named("green".to_string())),
                        Value::Literal("l".to_string()),
                        Value::Literal("a".to_string()),
                        Value::Literal("b".to_string()),
                    ],
                    Space
                ),
                Value::Literal("/".to_string()),
                Value::Number(0.5f64, None),
            ]
        )
    ));

    let decl = declarations.get(12).unwrap();
    // TODO l and alpha in calc expressions are not parsed correctly
    /*assert!(compare_values(
        &decl.value,
        &Function(
            "oklab".to_string(),
            vec![
                List(
                    vec![
                        Value::Literal("from".to_string()),
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
                Value::Literal("/".to_string()),
                Value::Calc(CalcExpression::BinaryOperation(
                    Box::new(CalcExpression::Variable("alpha".to_string())),
                    CalcOperator::Multiply,
                    Box::new(CalcExpression::Number(0.9f64, None))
                )),
            ]
        )
    ));*/

    let decl = declarations.get(13).unwrap();
    // TODO l in calc expression is not parsed correctly
    /*assert!(compare_values(
        &decl.value,
        &Function(
            "oklab".to_string(),
            vec![List(
                vec![
                    Value::Literal("from".to_string()),
                    Function(
                        "hsl".to_string(),
                        vec![List(
                            vec![
                                Value::Number(180f64, None),
                                Value::Number(100f64, Some(Unit::Percent)),
                                Value::Number(50f64, Some(Unit::Percent)),
                            ],
                            Space
                        )]
                    ),
                    Value::Calc(CalcExpression::BinaryOperation(
                        Box::new(CalcExpression::Variable("l".to_string())),
                        CalcOperator::Subtract,
                        Box::new(CalcExpression::Number(0.1f64, None))
                    )),
                    Value::Literal("a".to_string()),
                    Value::Literal("b".to_string()),
                ],
                Space
            ),]
        )
    ));*/

    let decl = declarations.get(14).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "rgba".to_string(),
            vec![
                List(
                    vec![
                        Value::Number(0f64, None),
                        Value::Number(0f64, None),
                        Value::Number(0f64, None),
                    ],
                    Space
                ),
                Value::Literal("/".to_string()),
                Value::Number(0.5f64, Some(Unit::Percent)),
            ]
        )
    ));
}

#[test]
fn test_color_mix_values() {
    let stylesheet = parse_test_file("functions.css").unwrap();

    let rule = stylesheet.get_rule_by_selector(".color-mix").unwrap();
    let declarations = rule.get_declarations("color");

    let decl = declarations.get(0).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "color-mix".to_string(),
            vec![
                List(vec![Value::Literal("hsl".to_string()),], Space),
                Function(
                    "hsl".to_string(),
                    vec![List(
                        vec![
                            Value::Number(200f64, None),
                            Value::Number(50f64, None),
                            Value::Number(80f64, None),
                        ],
                        Space
                    )]
                ),
                List(
                    vec![
                        Value::Color(Color::Named("coral".to_string())),
                        Value::Number(80f64, Some(Unit::Percent)),
                    ],
                    Space
                ),
            ]
        )
    ));

    let decl = declarations.get(1).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "color-mix".to_string(),
            vec![
                List(
                    vec![
                        Value::Literal("lch".to_string()),
                        Value::Literal("longer".to_string()),
                        Value::Literal("hue".to_string()),
                    ],
                    Space
                ),
                Function(
                    "hsl".to_string(),
                    vec![List(
                        vec![
                            Value::Number(200f64, Some(Unit::Deg)),
                            Value::Number(50f64, Some(Unit::Percent)),
                            Value::Number(80f64, Some(Unit::Percent)),
                        ],
                        Space
                    )]
                ),
                Value::Color(Color::Named("coral".to_string())),
            ]
        )
    ));

    let decl = declarations.get(2).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "color-mix".to_string(),
            vec![
                List(vec![Value::Literal("srgb".to_string()),], Space),
                Value::Color(Color::Named("plum".to_string())),
                Value::Color(Color::Hex("#f00".to_string())),
            ]
        )
    ));

    let decl = declarations.get(3).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "color-mix".to_string(),
            vec![
                List(vec![Value::Literal("lab".to_string()),], Space),
                List(
                    vec![
                        Value::Color(Color::Named("plum".to_string())),
                        Value::Number(60f64, Some(Unit::Percent)),
                    ],
                    Space
                ),
                List(
                    vec![
                        Value::Color(Color::Hex("#f00".to_string())),
                        Value::Number(50f64, Some(Unit::Percent)),
                    ],
                    Space
                ),
            ]
        )
    ));

    let decl = declarations.get(4).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "color-mix".to_string(),
            vec![
                List(vec![Value::Literal("--swop5c".to_string())], Space),
                Value::Color(Color::Named("red".to_string())),
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

    let decl = declarations.get(0).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "palette-mix".to_string(),
            vec![
                List(vec![Value::Literal("lch".to_string()),], Space),
                Value::Keyword("normal".to_string()),
                Value::Literal("dark".to_string()),
            ]
        )
    ));

    let decl = declarations.get(1).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "palette-mix".to_string(),
            vec![
                List(vec![Value::Literal("lch".to_string()),], Space),
                Value::Literal("--blues".to_string()),
                Value::Literal("--yellows".to_string()),
            ]
        )
    ));

    let decl = declarations.get(2).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "palette-mix".to_string(),
            vec![
                List(vec![Value::Literal("lch".to_string()),], Space),
                List(
                    vec![
                        Value::Literal("--blues".to_string()),
                        Value::Number(50f64, Some(Unit::Percent)),
                    ],
                    Space
                ),
                List(
                    vec![
                        Value::Literal("--yellows".to_string()),
                        Value::Number(50f64, Some(Unit::Percent)),
                    ],
                    Space
                ),
            ]
        )
    ));

    let decl = declarations.get(3).unwrap();
    assert!(compare_values(
        &decl.value,
        &Function(
            "palette-mix".to_string(),
            vec![
                List(
                    vec![
                        Value::Literal("hsl".to_string()),
                        Value::Literal("shorter".to_string()),
                        Value::Literal("hue".to_string()),
                    ],
                    Space
                ),
                Value::Literal("--blues".to_string()),
                Value::Literal("--yellows".to_string()),
            ]
        )
    ));
}

#[test]
fn test_space_separated_functions() {
    let stylesheet = parse_test_file("functions.css").unwrap();

    let rule = stylesheet.get_rule_by_selector(".space-separated").unwrap();

    let decl = rule.get_declaration("filter").unwrap();
    assert!(compare_values(
        &decl.value,
        &List(
            vec![
                Function(
                    "blur".to_string(),
                    vec![Value::Number(5.0, Some(Unit::Px))]
                ),
                Function(
                    "brightness".to_string(),
                    vec![Value::Number(120.0, Some(Unit::Percent))]
                ),
            ],
            Space
        )
    ));

    let decl = rule.get_declaration("transform").unwrap();
    assert!(compare_values(
        &decl.value,
        &List(
            vec![
                Function(
                    "rotate".to_string(),
                    vec![
                        List(
                            vec![Value::Number(45.0, Some(Unit::Deg))],
                            Space
                        )
                    ]
                ),
                Function(
                    "scale".to_string(),
                    vec![
                        List(
                            vec![Value::Number(2.0, None)],
                            Space
                        )
                    ]
                ),
                Function(
                    "translate".to_string(),
                    vec![
                        List(
                            vec![
                                Value::Number(10.0, Some(Unit::Px)),
                                Value::Literal(",".to_string()),
                                Value::Calc(
                                    CalcExpression::Number(10.0, Some(Unit::Px))
                                ),
                            ],
                            Space
                        )
                    ]
                ),
            ],
            Space
        )
    ));
}
