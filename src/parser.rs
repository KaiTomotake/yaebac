use crate::combinator::{Repeat, Rule, Then, Eoi, Opt, Or, skipper};

pub trait Parser: Sized {
    fn parse<S: Parser>(&self, src: &str, skip: Option<S>) -> Result<Vec<String>, ParseError> {
        Ok(self
            .parse_with_locate(src, skipper(src, 0, &skip), &skip)?
            .parsed)
    }

    fn parse_with_locate<S: Parser>(
        &self,
        src: &str,
        locate: usize,
        skip: &Option<S>,
    ) -> Result<Output, ParseError>;

    fn then<P: Parser>(self, parser: P) -> Then<Self, P> {
        Then {
            parser_a: self,
            parser_b: parser,
        }
    }

    fn into_rule(self, name: &str) -> Rule<Self> {
        Rule {
            name: name.to_string(),
            parser: self,
        }
    }

    fn repeated(self) -> Repeat<Self> {
        Repeat { parser: self }
    }

    fn eoi(self) -> Eoi<Self> {
        Eoi {
            parser: self
        }
    }

    fn opt(self) -> Opt<Self> {
        Opt {
            parser: self
        }
    }

    fn or<P: Parser>(self, parser: P) -> Or<Self, P> {
        Or {
            parser_a: self,
            parser_b: parser
        }
    }
}

#[derive(Debug)]
pub struct Output {
    pub parsed: Vec<String>,
    pub locate: usize,
}

#[derive(Debug)]
pub struct ParseError {
    pub rule: String,
    pub locate: usize,
}
