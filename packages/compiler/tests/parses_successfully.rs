use compiler::parser::parse_with_program;

#[test]
fn padded_linebreaks() {
    parse_with_program("\n\n\n// This is a comment statement!\n\n\n").unwrap();
}

#[test]
fn padded_statement() {
    parse_with_program("    \n  // This is a comment statement! \n  ").unwrap();
}

#[test]
fn comments() {
    parse_with_program("// This is a comment statement!").unwrap();
    // parse_with_program("/* This is a comment statement! */").unwrap();
}

#[test]
fn identifiers() {
    parse_with_program("foo").unwrap();
    parse_with_program("Foo").unwrap();
    parse_with_program("FOO").unwrap();
    parse_with_program("foo_bar").unwrap();
    parse_with_program("foo_bar_123").unwrap();
    parse_with_program("const_foo").unwrap();
    parse_with_program("next123").unwrap();
}

#[test]
fn invalid_identifiers() {
    parse_with_program("1_foo").unwrap_err();
    parse_with_program("const").unwrap_err();
}

#[test]
fn variable_assignments() {
    parse_with_program("foo = 1").unwrap();
    parse_with_program("constfoo=1").unwrap();
    parse_with_program("global  foo  =  1").unwrap();
}

#[test]
fn array_declarations() {
    parse_with_program("array foo[1]").unwrap();
    parse_with_program("array foo[1,2,3]").unwrap();
    parse_with_program("array foo[1] = []").unwrap();
    parse_with_program("array foo[bar] = [1,2,baz]").unwrap();
    parse_with_program("array foo[1,2,3] = [[1,2,3],[bar,baz,xyz],[]]").unwrap();
}

#[test]
fn array_assignments() {
    parse_with_program("foo[1] = bar").unwrap();
    parse_with_program("foo[0,1,bar] = baz").unwrap();
}

#[test]
fn array_expressions() {
    parse_with_program("[]").unwrap();
    parse_with_program("[foo,bar,1,2,3]").unwrap();
    parse_with_program("[[foo,bar],[1,2,3,[baz]]]").unwrap();
    parse_with_program("foo[1]").unwrap();
}

#[test]
fn invalid_arrays() {
    parse_with_program("array foo[]").unwrap_err();
    parse_with_program("array foo[] = [1,2,3]").unwrap_err();
    parse_with_program("array const[]").unwrap_err();
    parse_with_program("foo[] = 1").unwrap_err();
}

#[test]
fn function_call_expressions() {
    parse_with_program("foo()").unwrap();
    parse_with_program("foo(bar)").unwrap();
    parse_with_program("foo(bar, baz)").unwrap();
    parse_with_program("foo(1, [bar, baz])").unwrap();
}

#[test]
fn strings() {
    parse_with_program(r#" "foo" "#).unwrap();
    parse_with_program(r#" "foo\"bar" "#).unwrap();
    parse_with_program(r#" "foo\nbar" "#).unwrap();
}

#[test]
fn numbers() {
    parse_with_program("123").unwrap();
    parse_with_program("123.456").unwrap();
    parse_with_program("+123").unwrap();
    parse_with_program("-123").unwrap();
}

#[test]
fn operator_expressions() {
    parse_with_program("123 + 456").unwrap();
    parse_with_program("123.456-foo").unwrap();
    parse_with_program("(123 * -123) + 33").unwrap();
    parse_with_program("(33) - (foo + (6))").unwrap();
    parse_with_program("foo ^ bar(123 MOD 456) / 789").unwrap();
    parse_with_program("123 <= foo AND 456 != bar").unwrap();
}

#[test]
fn not_operator_expressions() {
    parse_with_program("NOT true").unwrap();
    parse_with_program("NOT NOT true").unwrap();
    parse_with_program("NOT NOT (true)").unwrap();
    parse_with_program("NOT NOT (NOT true)").unwrap();
    parse_with_program("foo + NOT true").unwrap();
}

#[test]
fn invalid_operator_expressions() {
    parse_with_program("123 456").unwrap_err();
    parse_with_program("123 MOD123").unwrap_err();
    parse_with_program("123 + - foo").unwrap_err();
    parse_with_program("-foo - -foo - - -foo").unwrap_err();
    parse_with_program("123 + (foo - bar").unwrap_err();
    parse_with_program("123 <=").unwrap_err();
}

#[test]
fn for_loops() {
    parse_with_program("for i=0 to 10\nprint(i)\nnext i").unwrap();
    parse_with_program("for  i  =  0  to  foo  \n    print(i) \n next   i").unwrap();
    parse_with_program(
        "for foo=bar to get(baz) + 123
            // comment
            foo + bar
        next foo",
    )
    .unwrap();
}

#[test]
fn invalid_for_loops() {
    parse_with_program("for i=0 to 10\nnext i").unwrap_err();
    parse_with_program("for i = 0 to 10\ni\nnext j").unwrap_err();
    parse_with_program("for i = 0 too 10\ni\nnext i").unwrap_err();
}

#[test]
fn if_statements() {
    parse_with_program("if    foo then\n\nprint(foo)  \n\n endif").unwrap();
    parse_with_program(
        "if foo then
            print(foo)
        elseif bar then
            print(bar)
        elseif (baz) then
            print(baz)
        else
            print(123)
        endif
        ",
    )
    .unwrap();
    parse_with_program(
        "if 123 + 456 then
            print(123)
        else
            print(foo)
        endif",
    )
    .unwrap();
}

#[test]
fn invalid_if_statements() {
    parse_with_program("if foo\nprint(foo)\nendif").unwrap_err();
    parse_with_program("if").unwrap_err();
    parse_with_program("if foo then\nprint(foo)\nelseif bar\nprint(bar)\nendif").unwrap_err();
    parse_with_program("if foo then\nprint(foo)\nelseif bar then\nprint(bar)\n").unwrap_err();
}

// #[test]
#[allow(dead_code, unused_must_use)]
fn test() {
    let program = "NOT NOT (33)";

    dbg!(parse_with_program(program));
    panic!();
}
