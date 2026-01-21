use yaebac::parser::Parser;
use yaebac::primitive::{lit, re};
use yaebac::combinator::NoSkip;

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

#[test]
#[allow(unused_must_use)]
fn hw_repeated() {
    let hw_rule = lit("Hello, World!").into_rule("hw");
    dbg!(hw_rule.repeated().parse(
        "Hello, World! Hello, World!Hello, World!",
        Some(re("\\s+").unwrap())
    ));
}

#[test]
fn hw_eoi_checked() {
    let hw_rule = lit("Hello, World!").into_rule("hw");
    assert!(
        hw_rule
            .repeated()
            .eoi()
            .parse(
                "Hello, World! Hello, World!Hello, Warld!",
                Some(re("\\s+").unwrap())
            )
            .is_err()
    );
}

#[test]
fn hw_opt() {
    let h_rule = lit("Hello").opt().into_rule("hello");
    assert_eq!(
        h_rule.then(lit("World")).parse("World", Some(re("\\s+").unwrap())).unwrap(),
        &["World".to_string()]
    );
}

#[test]
fn hw_or() {
    assert!(
        lit("Hello").or(lit("World")).parse::<NoSkip>("World", None).is_ok()
    )
}