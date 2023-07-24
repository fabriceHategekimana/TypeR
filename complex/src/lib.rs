use base_language::{Language, BaseType, Type};
use nom::sequence::tuple;
use nom::character::complete::digit1;
use nom::combinator::opt;
use nom::bytes::complete::tag;
use nom::IResult;

pub fn floating_point(s: &str) -> IResult<&str, String> {
    let res = tuple((tag("."), digit1))(s);
    match res {
        Ok((s, (p, d))) => Ok((s, format!("{}{}", p, d))),
        Err(e) => Err(e)
    }
}

pub fn parse_real(s: &str) -> IResult<&str, String> {
    let res = tuple((
               opt(tag("-")),
               digit1,
               opt(floating_point)
                  ))(s);
    match res {
        Ok((s, (Some(m), v, Some(r)))) => Ok((s, format!("{}{}{}", m, v, r))),
        Ok((s, (Some(m), v, None))) => Ok((s, format!("{}{}", m, v))),
        Ok((s, (None, v, Some(r)))) => Ok((s, format!("{}{}", v, r))),
        Ok((s, (None, v, None))) => Ok((s, format!("{}", v))),
        Err(e) => Err(e)
    }
}

pub fn parse_complex(s: &str) -> IResult<&str, Language> {
    let res = tuple((
               parse_real,
               tag("i")
                  ))(s);
    match res {
        Ok((s, (v, i))) => Ok((s, Language::Value(format!("{}{}", v, i), Type::Scalar(BaseType::Complex)))),
        Err(e) => Err(e)
    }
}


#[cfg(test)]
mod tests {
    use base_language::{Language, BaseType, Type};
    use super::{parse_complex, parse_real};

    #[test]
    fn test_complex_1() {
        assert_eq!(
            parse_complex("3i").unwrap().1,
            Language::Value("3i".to_string(), Type::Scalar(BaseType::Complex)));
    }

    #[test]
    fn test_complex_2() {
        assert_eq!(
            parse_complex("251i").unwrap().1,
            Language::Value("251i".to_string(), Type::Scalar(BaseType::Complex)));
    }
    
    #[test]
    fn test_parse_real() {
        assert_eq!(
            parse_real("251").unwrap().1,
            "251".to_string());
    }
    
}
