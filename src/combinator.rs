use crate::parser::{Output, ParseError, Parser};

pub(crate) fn skipper<S: Parser>(src: &str, locate: usize, skip: &Option<S>) -> usize {
    if let Some(s) = skip
        && let Ok(l) = s.parse_with_locate::<S>(src, locate, &None)
    {
        return l.locate;
    }
    locate
}

#[derive(Debug)]
pub struct Lit {
    pub(crate) text: String,
}

impl Parser for Lit {
    fn parse_with_locate<S: Parser>(
        &self,
        src: &str,
        mut locate: usize,
        skip: &Option<S>,
    ) -> Result<Output, ParseError> {
        if src[src
            .char_indices()
            .nth(locate)
            .map(|(i, _)| i)
            .ok_or(ParseError {
                rule: "lit".to_string(),
                locate,
            })?..]
            .starts_with(self.text.as_str())
        {
            locate += self.text.len();
        } else {
            return Err(ParseError {
                rule: "lit".to_string(),
                locate,
            });
        }
        Ok(Output {
            parsed: vec![self.text.to_string()],
            locate: skipper(src, locate, skip),
        })
    }
}

#[derive(Debug)]
pub struct Then<A: Parser, B: Parser> {
    pub(crate) parser_a: A,
    pub(crate) parser_b: B,
}

impl<A: Parser, B: Parser> Parser for Then<A, B> {
    fn parse_with_locate<S: Parser>(
        &self,
        src: &str,
        locate: usize,
        skip: &Option<S>,
    ) -> Result<Output, ParseError> {
        let output_a = self.parser_a.parse_with_locate(src, locate, skip)?;
        let output_b = self
            .parser_b
            .parse_with_locate(src, output_a.locate, skip)?;
        Ok(Output {
            parsed: output_a.parsed.into_iter().chain(output_b.parsed).collect(),
            locate: skipper(src, output_b.locate, skip),
        })
    }
}

#[derive(Debug)]
pub struct Re {
    pub(crate) pattern: regex::Regex,
}

impl Parser for Re {
    fn parse_with_locate<S: Parser>(
        &self,
        src: &str,
        locate: usize,
        skip: &Option<S>,
    ) -> Result<Output, ParseError> {
        if let Some(mat) = self.pattern.find(
            &src[src
                .char_indices()
                .nth(locate)
                .map(|(i, _)| i)
                .ok_or(ParseError {
                    rule: "lit".to_string(),
                    locate,
                })?..],
        ) {
            let matched = mat.as_str();
            return Ok(Output {
                parsed: vec![matched.to_string()],
                locate: skipper(src, locate + matched.len(), skip),
            });
        }
        Err(ParseError {
            rule: "re".to_string(),
            locate,
        })
    }
}

#[derive(Debug)]
pub struct NoSkip {}

impl Parser for NoSkip {
    fn parse_with_locate<S: Parser>(
        &self,
        _src: &str,
        locate: usize,
        _skip: &Option<S>,
    ) -> Result<Output, ParseError> {
        Err(ParseError {
            rule: "noskip".to_string(),
            locate,
        })
    }
}

#[derive(Debug)]
pub struct Rule<P: Parser> {
    pub(crate) name: String,
    pub(crate) parser: P,
}

impl<P: Parser> Parser for Rule<P> {
    fn parse_with_locate<S: Parser>(
        &self,
        src: &str,
        locate: usize,
        skip: &Option<S>,
    ) -> Result<Output, ParseError> {
        self.parser
            .parse_with_locate(src, locate, skip)
            .map_err(|_| ParseError {
                rule: self.name.clone(),
                locate,
            })
            .map(|out| Output {
                parsed: out.parsed,
                locate: skipper(src, out.locate, skip),
            })
    }
}

#[derive(Debug)]
pub struct Repeat<P: Parser> {
    pub(crate) parser: P,
}

impl<P: Parser> Parser for Repeat<P> {
    fn parse_with_locate<S: Parser>(
        &self,
        src: &str,
        mut locate: usize,
        skip: &Option<S>,
    ) -> Result<Output, ParseError> {
        let mut outs = Vec::new();
        while let Ok(o) = self.parser.parse_with_locate(src, locate, skip) {
            outs.extend(o.parsed);
            locate = o.locate;
        }
        Ok(Output {
            parsed: outs,
            locate: skipper(src, locate, skip),
        })
    }
}
