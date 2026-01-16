use yaebac::parser::Parser;
use yaebac::primitive::{lit, re};

#[test]
#[allow(unused_must_use)]
fn hello_world() {
    dbg!(
        lit("Hello")
            .then(lit(","))
            .then(lit("World"))
            .then(lit("!"))
            .parse("Hello, World!", Some(re("\\s+").unwrap()))
    );
}

#[test]
#[allow(unused_must_use)]
fn hw_with_rule() {
    let hello_rule = lit("Hello").then(lit(",")).into_rule("hello");
    let world_rule = lit("World").then(lit("!")).into_rule("world");
    dbg!(
        hello_rule
            .then(world_rule)
            .parse("Hello, World!", Some(re("\\s+").unwrap()))
    );
}
