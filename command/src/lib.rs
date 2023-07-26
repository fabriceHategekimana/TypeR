use expression::parse_expression;
use assignment::parse_assignment;
use nom::branch::alt;
use union::parse_union_type;
use nom::IResult;
use base_language::Language;

pub fn parse_command(s: &str) -> IResult<&str,Language> {
    alt((
            parse_expression,
            parse_assignment,
            parse_union_type,
        ))(s)
}

