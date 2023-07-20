use base_language::{Language, Type};
use nom::sequence::tuple;
use nom::character::complete::digit1;
use nom::combinator::opt;
use nom::bytes::complete::tag;
use nom::IResult;

pub fn parse_complex(s: &str) -> IResult<&str, Language> {
    let res = tuple((
               opt(tag("-")),
               digit1,
               tag("i")
                  ))(s);
    match res {
        Ok((s, (Some(m), v, r))) => Ok((s, Language::Value(format!("{}{}{}", m, v, r), Type::Complex))),
        Ok((s, (None, v, r))) => Ok((s, Language::Value(format!("{}{}", v, r), Type::Complex))),
        Err(e) => Err(e)
    }
}


#[cfg(test)]
mod tests {
    use base_language::{Language, Type};
    use nom;
    use super::parse_complex;

    #[test]
    fn test_complex_1() {
        assert_eq!(
            parse_complex("3i").unwrap().1,
            Language::Value("3i".to_string(), Type::Complex));
    }

    #[test]
    fn test_complex_2() {
        assert_eq!(
            parse_complex("251i").unwrap().1,
            Language::Value("251i".to_string(), Type::Complex));
    }
    
}
