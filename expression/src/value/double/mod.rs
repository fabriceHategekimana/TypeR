use nom::sequence::tuple;
use nom::combinator::opt;
use nom::character::complete::digit1;
use nom::bytes::complete::tag;
use nom::IResult;
use base_language::{Language, BaseType, Type};

pub fn floating_point(s: &str) -> IResult<&str, String> {
    let res = tuple((tag("."), digit1))(s);
    match res {
        Ok((s, (p, d))) => Ok((s, format!("{}{}", p, d))),
        Err(e) => Err(e)
    }
}

pub fn parse_double(s: &str) -> IResult<&str, Language> {
    let res = tuple((
               opt(tag("-")),
               digit1,
               floating_point
                  ))(s);
    match res {
        Ok((s, (Some(m), v, r))) => Ok((s, Language::Value(format!("{}{}{}", m, v, r), Type::Scalar(BaseType::Double)))),
        Ok((s, (None, v, r))) => Ok((s, Language::Value(format!("{}{}", v, r), Type::Scalar(BaseType::Double)))),
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use base_language::{Language, BaseType, Type};
    use super::parse_double;

    #[test]
    fn test_double() {
        assert_eq!(
            parse_double("3.2").unwrap().1,
            Language::Value("3.2".to_string(), Type::Scalar(BaseType::Double)));
        assert_eq!(
            parse_double("-8.9").unwrap().1,
            Language::Value("-8.9".to_string(), Type::Scalar(BaseType::Double)));
    }

}
