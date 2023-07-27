use nom::branch::alt;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::sequence::preceded;

use base_language::Language;

pub fn parse_open_parenthesis(s: &str) -> IResult<&str,&str> {
    alt((
            tag(" ( "),
            tag("( "),
            tag(" ("),
            tag("("),
        ))(s)
}

pub fn parse_close_parenthesis(s: &str) -> IResult<&str,&str> {
    alt((
            tag(" ) "),
            tag(") "),
            tag(" )"),
            tag(")"),
        ))(s)
}

pub fn parse_close_bracket(s: &str) -> IResult<&str,&str> {
    alt((
            tag(" } "),
            tag("} "),
            tag(" }"),
            tag("}"),
        ))(s)
}

pub fn parse_open_bracket(s: &str) -> IResult<&str,&str> {
    alt((
            tag(" { "),
            tag("{ "),
            tag(" {"),
            tag("{"),
        ))(s)
}

pub fn parse_colon(s: &str) -> IResult<&str,&str> {
    alt((
            tag(" : "),
            tag(": "),
            tag(" :"),
            tag(":"),
        ))(s)
}

pub fn parse_left_arrow(s: &str) -> IResult<&str,&str> {
    alt((
            tag(" <- "),
            tag(" <-"),
            tag("<- "),
            tag("<-"),
        ))(s)
}

pub fn parse_right_arrow(s: &str) -> IResult<&str,&str> {
    alt((
            tag(" -> "),
            tag(" ->"),
            tag("-> "),
            tag("->"),
        ))(s)
}

pub fn parse_equal_assignement(s: &str) -> IResult<&str,&str> {
    alt((
            tag(" = "),
            tag(" ="),
            tag("= "),
            tag("="),
        ))(s)
}

// actually, don't parse left assignement
pub fn parse_assignement_symbol(s: &str) -> IResult<&str,&str> {
    alt((
            parse_left_arrow,
            parse_equal_assignement,
        ))(s)
}

pub fn parse_separator(s: &str) -> IResult<&str,&str> {
    alt((
       tag(" , "),
       tag(", "),
       tag(" ,"),
       tag(","),
        ))(s)
}

pub fn parse_single_or(s: &str) -> IResult<&str,&str> {
    alt((
       tag(" | "),
       tag(" |"),
       tag("| "),
       tag("|"),
        ))(s)
}

fn parse_type_indicator(s: &str) -> IResult<&str,&str> {
    alt((
            parse_colon,
            parse_right_arrow
        ))(s)
}

fn parse_nullable_type(s: &str) -> IResult<&str,&str> {
    alt((
            tag("?^lgl[]"),
            tag("?^int[]"),
            tag("?^dbl[]"),
            tag("?^clx[]"),
            tag("?^chr[]"),
            tag("?^raw[]"),
            tag("?lgl[]"),
            tag("?int[]"),
            tag("?dbl[]"),
            tag("?clx[]"),
            tag("?chr[]"),
            tag("?raw[]"),
            tag("?lgl"),
            tag("?int"),
            tag("?dbl"),
            tag("?clx"),
            tag("?chr"),
            tag("?raw"),
        ))(s)
}

fn parse_normal_type(s: &str) -> IResult<&str,&str> {
    alt((
            tag("^lgl[]"),
            tag("^int[]"),
            tag("^dbl[]"),
            tag("^clx[]"),
            tag("^chr[]"),
            tag("^raw[]"),
            tag("lgl[]"),
            tag("int[]"),
            tag("dbl[]"),
            tag("clx[]"),
            tag("chr[]"),
            tag("raw[]"),
            tag("lgl"),
            tag("int"),
            tag("dbl"),
            tag("clx"),
            tag("chr"),
            tag("raw"),
        ))(s)
}

pub fn parse_type(s: &str) -> IResult<&str,Language> {
    let res = alt((
            parse_nullable_type,
            parse_normal_type,
        ))(s);
    match res {
        Ok((s, r)) => Ok((s, Language::Reserved(r.to_string()))),
        Err(r) => Err(r)
    }
}

pub fn parse_symbol(s: &str) -> IResult<&str, Language> {
    let res = alphanumeric1(s);
    match res {
        Ok((s, n)) => Ok((s, Language::Symbol(n.to_string()))),
        Err(r) => Err(r)
    }
}

fn parse_general_type(s: &str) -> IResult<&str,Language> {
    alt((
            parse_type,
            parse_symbol
        ))(s)
}

pub fn parse_type_annotation(s: &str) -> IResult<&str,Language> {
    preceded(parse_type_indicator, parse_general_type)(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_annotation_with_colon(){
        assert_eq!(
            parse_type_annotation(": int").unwrap().1,
            Language::Reserved("int".to_string()));
    }

    #[test]
    fn test_type_annotation_with_arrow(){
        assert_eq!(
            parse_type_annotation(" -> DataFrame").unwrap().1,
            Language::Symbol("DataFrame".to_string()));
    }

}
