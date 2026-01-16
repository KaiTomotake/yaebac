use yaebac::prelude::*;
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
