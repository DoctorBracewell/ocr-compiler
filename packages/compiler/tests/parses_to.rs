use compiler::parser::{OCRParser, Rule};
use pest::{consumes_to, parses_to};

#[test]
fn tagged_variable_assignment() {
    parses_to! {
        parser: OCRParser,
        input: "global foo = 1",
        rule: Rule::program,
        tokens: [
            taggedVariableAssignment(0, 14, [
                globalKeyword(0, 6),
                variableAssignment(7, 14, [
                    identifier(7, 10),
                    expression(13, 14, [
                        literal(13, 14, [
                            positiveNumber(13, 14)
                        ])
                    ])

                ])
            ]),
        ]
    }
}

#[test]
fn expression() {
    parses_to! {
        parser: OCRParser,
        input: "123 + foo((-456) ^ bar, [1, 2,3])  AND  789 * baz[000]",
        rule: Rule::program,
        tokens: [
            expression(0, 54, [
                literal(0, 3, [
                    positiveNumber(0, 3)
                ]),
                operator(4, 5, [
                    additionOperator(4, 5)
                ]),
                functionCall(6, 33, [
                    identifier(6, 9),
                    functionParameters(9, 33, [
                        expression(10, 22, [
                            expression(11, 15, [
                                literal(11, 15, [
                                    negativeNumber(11, 15)
                                ])
                            ]),
                            operator(17, 18, [
                                exponentOperator(17, 18)
                            ]),
                            identifier(19, 22),
                        ]),
                        expression(24, 32, [
                            array(24, 32, [
                                arrayValues(24, 32, [
                                    expression(25, 26, [
                                        literal(25, 26, [
                                            positiveNumber(25, 26)
                                        ])
                                    ]),
                                    expression(28, 29, [
                                        literal(28, 29, [
                                            positiveNumber(28, 29)
                                        ])
                                    ]),
                                    expression(30, 31, [
                                        literal(30, 31, [
                                            positiveNumber(30, 31)
                                        ])
                                    ])
                                ])
                            ])
                        ])
                    ])
                ]),
                operator(35, 40, [
                    logicalAndOperator(35, 38)
                ]),
                literal(40, 43, [
                    positiveNumber(40, 43)
                ]),
                operator(44, 45, [
                    multiplicationOperator(44, 45)
                ]),
                arrayIndexing(46, 54, [
                    identifier(46, 49),
                    arrayValues(49, 54, [
                        expression(50, 53, [
                            literal(50, 53, [
                                positiveNumber(50, 53)
                            ])
                        ])

                    ])
                ])
            ])
        ]
    }
}
