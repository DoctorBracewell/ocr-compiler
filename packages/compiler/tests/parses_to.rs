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
