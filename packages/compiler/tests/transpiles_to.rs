use compiler::{transpiler::transpile, parser::parse};

#[test]
#[allow(dead_code, unused_must_use)]
fn test() {
    let program = "test = 1\ntest = 2";
    let parsed = parse(program);

    dbg!(transpile(parsed));
    panic!();
}