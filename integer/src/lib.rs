use nom::sequence::tuple;
use nom::combinator::opt;
use nom::character::complete::digit1;
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::IResult;
use base_language::{Language, Type};

// TODO: Test digit L
fn parse_digit_l(s: &str) -> IResult<&str, String> {
    let res = tuple((digit1, tag("L")))(s);
    match res {
        Ok((s, (d, l))) => Ok((s, format!("{}{}", d, l))),
        Err(e) => Err(e)
    }
}

fn parse_digit_1(s: &str) ->IResult<&str, String> {
    let res = digit1(s);
    match res {
        Ok((s, r)) => Ok((s, r.to_string())),
        Err(e) => Err(e)
    }
}

pub fn parse_integer(s: &str) -> IResult<&str, Language> {
    let res = alt((
            tuple((opt(tag("-")), parse_digit_l)),
            tuple((opt(tag("-")), parse_digit_1))
        ))(s);
    match res {
        Ok((s, (Some(m), v))) => Ok((s, Language::Value(format!("{}{}", m, v), Type::Integer))),
        Ok((s, (None, v))) => Ok((s, Language::Value(format!("{}", v), Type::Integer))),
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use base_language::{Language, Type};
    use super::{parse_integer, parse_digit_l, parse_digit_1};

    #[test]
    fn test_integer1() {
        assert_eq!(
            parse_integer("7").unwrap().1,
            Language::Value("7".to_string(), Type::Integer));
        assert_eq!(
            parse_integer("3L").unwrap().1,
            Language::Value("3L".to_string(), Type::Integer));
        assert_eq!(
            parse_integer("-8").unwrap().1,
            Language::Value("-8".to_string(), Type::Integer));
        assert_eq!(
            parse_integer("-8L").unwrap().1,
            Language::Value("-8L".to_string(), Type::Integer));
    }

    #[test]
    fn test_parse_digit_l() {
        assert_eq!(
            parse_digit_l("3L").unwrap().1,
            "3L".to_string());
    }

    #[test]
    fn test_parse_digit_1() {
        assert_eq!(
            parse_digit_1("3").unwrap().1,
            "3".to_string());
    }

}
