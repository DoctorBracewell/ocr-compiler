use compiler::parser::parse;

#[test]
fn padded_linebreaks() {
    parse("\n\n\n// This is a comment statement!\n\n\n").unwrap();
}

#[test]
fn padded_statement() {
    parse("    \n  // This is a comment statement! \n  ").unwrap();
}

#[test]
fn comment() {
    parse("// This is a comment statement!").unwrap();
    parse("/* This is a comment statement! */").unwrap();
}

#[test]
fn identifiers() {
    parse("foo").unwrap();
    parse("Foo").unwrap();
    parse("FOO").unwrap();
    parse("foo_bar").unwrap();
    parse("foo_bar_123").unwrap();
    parse("const_foo").unwrap();
}

#[test]
fn invalid_identifier() {
    parse("1_foo").unwrap_err();
    parse("const").unwrap_err();
}

#[test]
fn variable_assignment() {
    parse("foo = 1").unwrap();
    parse("constfoo=1").unwrap();
    parse("global  foo  =  1").unwrap();
}

#[test]
fn array_declaration() {
    parse("array foo[1]").unwrap();
    parse("array foo[1,2,3]").unwrap();
    parse("array foo[1] = []").unwrap();
    parse("array foo[bar] = [1,2,baz]").unwrap();
    parse("array foo[1,2,3] = [[1,2,3],[bar,baz,xyz],[]]").unwrap();
}

#[test]
fn array_assignments() {
    parse("foo[1] = bar").unwrap();
    parse("foo[0,1,bar] = baz").unwrap();
}

#[test]
fn array_expressions() {
    parse("[]").unwrap();
    parse("[foo,bar,1,2,3]").unwrap();
    parse("[[foo,bar],[1,2,3,[baz]]]").unwrap();
    parse("foo[1]").unwrap();
}

#[test]
fn invalid_arrays() {
    parse("array foo[]").unwrap_err();
    parse("array foo[] = [1,2,3]").unwrap_err();
    parse("array const[]").unwrap_err();
    parse("foo[] = 1").unwrap_err();
}

#[test]
fn function_call_expressions() {
    parse("foo()").unwrap();
    parse("foo(bar)").unwrap();
    parse("foo(bar, baz)").unwrap();
    parse("foo(1, [bar, baz])").unwrap();
}

#[test]
fn strings() {
    parse(r#" "foo" "#).unwrap();
    parse(r#" "foo\"bar" "#).unwrap();
    parse(r#" "foo\nbar" "#).unwrap();
}

#[test]
fn numbers() {
    parse("123").unwrap();
    parse("123.456").unwrap();
    parse("+123").unwrap();
    parse("-123").unwrap();
}

#[test]
fn operator_expressions() {
    parse("123 + 456").unwrap();
    parse("123.456-foo").unwrap();
    parse("(123 * -123) + 33").unwrap();
    parse("5(33) - (foo + (6))").unwrap();
    parse("foo ^ bar(123 MOD 456) / 789").unwrap(); // bugged
}

#[test]
#[allow(dead_code, unused_must_use)]
fn test() {
    let program = "5 5";
    dbg!(parse(program));
    panic!();
}
