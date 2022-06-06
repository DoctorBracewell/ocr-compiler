use super::*;

#[test]
fn parse_padded_linebreaks() {
    let result = parse("\n\n\n// This is a comment statement!\n\n\n").unwrap();
    dbg!(result);
}

#[test]
fn parse_padded_statement() {
    let result = parse("    \n  // This is a comment statement! \n  ").unwrap();
    dbg!(result);
}

#[test]
fn parse_comment() {
    let result = parse("// This is a comment statement!").unwrap();
    dbg!(result);
}

#[test]
fn parse_assignment() {
    parse("variable_name = test").unwrap();
    parse("const variable_name1=test").unwrap();
    parse("global  variableName  =  test").unwrap();
}

// #[test]
// fn debug() {
//     dbg!(parse("//comment!").unwrap());
//     panic!("");
// }
