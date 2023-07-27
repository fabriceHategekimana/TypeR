use nom::IResult;
use base_language::Language;
use base_language::r#type::{Type, BaseType};
use base_language::value::Value;
use nom::bytes::complete::{tag, take_until};
use nom::sequence::tuple;

pub fn parse_character(s: &str) -> IResult<&str, Language> {
    let res = tuple((
            tag("\""),
            take_until("\""),
            tag("\"")
                  ))(s);
    match res {
        Ok((s, (q1, v, q2))) => Ok((s, Language::Value(Value::new(&format!("{}{}{}", q1, v, q2), Type::Scalar(BaseType::Character))))),
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use base_language::Language;
    use base_language::r#type::{Type, BaseType};
    use super::parse_character;
    use base_language::value::Value;
    use nom;

    #[test]
    fn test_character() {
        assert_eq!(
            parse_character("\"Hello\"").unwrap().1,
            Language::Value(Value::new("\"Hello\"", Type::Scalar(BaseType::Character))));
    }

    #[test]
    fn test_character_false() {
        assert_eq!(
            parse_character("7"),
            Err(nom::Err::Error(nom::error::Error{
                input: "7",
                code: nom::error::ErrorKind::Tag    
            })));
    }
}
