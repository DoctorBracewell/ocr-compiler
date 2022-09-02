use compiler::{parser::parse, transpiler::transpile};

#[test]
#[allow(dead_code, unused_must_use)]
fn test() {
    let program = "test = 1
                   test.upper";
    let parsed = parse(program);

    dbg!(&parsed);
    dbg!(transpile(parsed));
    panic!();
}
