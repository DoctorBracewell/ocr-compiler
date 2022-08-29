use compiler::{parser::parse, transpiler::transpile};

#[test]
#[allow(dead_code, unused_must_use)]
fn test() {
    let program = "test = NOT test
                    a = NOT b";
    let parsed = parse(program);

    dbg!(&parsed);
    dbg!(transpile(parsed));
    panic!();
}
