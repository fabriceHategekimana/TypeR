use nom::sequence::tuple;
use nom::combinator::opt;
use nom::character::complete::digit1;
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::IResult;
use base_language::r#type::{Type, BaseType};
use base_language::value::Value;

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

pub fn parse_integer(s: &str) -> IResult<&str, Value> {
    let res = alt((
            tuple((opt(tag("-")), parse_digit_l)),
            tuple((opt(tag("-")), parse_digit_1))
        ))(s);
    match res {
        Ok((s, (Some(m), v))) => Ok((s, Value::new(&format!("{}{}", m, v), Type::Scalar(BaseType::Integer)))),
        Ok((s, (None, v))) => Ok((s, Value::new(&format!("{}", v), Type::Scalar(BaseType::Integer)))),
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use base_language::r#type::{Type, BaseType};
    use super::{parse_integer, parse_digit_l, parse_digit_1};
    use base_language::value::Value;

    #[test]
    fn test_integer1() {
        assert_eq!(
            parse_integer("7").unwrap().1,
            Value::new("7", Type::Scalar(BaseType::Integer)));
        assert_eq!(
            parse_integer("3L").unwrap().1,
            Value::new("3L", Type::Scalar(BaseType::Integer)));
        assert_eq!(
            parse_integer("-8").unwrap().1,
            Value::new("-8", Type::Scalar(BaseType::Integer)));
        assert_eq!(
            parse_integer("-8L").unwrap().1,
            Value::new("-8L", Type::Scalar(BaseType::Integer)));
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
