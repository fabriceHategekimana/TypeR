use nom::branch::alt;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use base_language::{Language, Type, BaseType};

pub fn parse_separator(s: &str) -> IResult<&str,&str> {
    alt((
       tag(" , "),
       tag(", "),
       tag(" ,"),
       tag(","),
        ))(s)
}

pub fn parse_symbol(s: &str) -> IResult<&str, Language> {
    let res = alphanumeric1(s);
    match res {
        Ok((s, n)) => Ok((s, Language::Symbol(n.to_string(), Type::Scalar(BaseType::Any)))),
        Err(r) => Err(r)
    }
}

