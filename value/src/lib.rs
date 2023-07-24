use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::{tuple, preceded};
use nom::multi::many1;
use nom::IResult;
use base_language::{Language, BaseType, Type};
use logical::parse_logical;
use integer::parse_integer;
use double::parse_double;
use complex::parse_complex;
use character::parse_character;
use base_parser::parse_symbol;

pub fn parse_value(s: &str) -> IResult<&str, Language> {
    alt((
            parse_complex,
            parse_double,
            parse_integer,
            parse_logical,
            parse_character
        ))(s)
}

fn parse_vector_argument(s: &str) ->  IResult<&str, Language> {
    alt((
        parse_value,
        parse_symbol))(s)
}

fn parse_comma_vector_argument(s: &str) -> IResult<&str, Language> {
    preceded(tag(","), parse_vector_argument)(s)
}

fn parse_vector_arguments(s: &str) -> IResult<&str, Language> {
    let res = many1(alt((
            parse_comma_vector_argument,
            parse_vector_argument
              )))(s);
    match res {
        Ok((s, v)) => Ok((s, Language::VectorArguments(v, Type::Scalar(BaseType::Any)))),
        Err(r) => Err(r)
    }
}

pub fn parse_vector(s: &str) -> IResult<&str, Language> {
    let res = tuple((
            tag("c("),
            parse_vector_arguments,
            tag(")")
          ))(s);
    match res {
        Ok((s, (_, a, _))) => Ok((s, a)),
        Err(r) => Err(r)
    }
}

#[cfg(test)]
mod tests {
    use super::parse_value;
    use base_language::{Language, BaseType};

    #[test]
    fn test_character() {
        assert_eq!(
            parse_value("\"Hello\"").unwrap().1,
            Language::Value("\"Hello\"".to_string(), Type::Scalar(BaseType::Character)));
    }

    #[test]
    fn test_complex() {
        assert_eq!(
            parse_value("3i").unwrap().1,
            Language::Value("3i".to_string(), Type::Scalar(BaseType::Complex))
                  );
    }

    #[test]
    fn test_double() {
        assert_eq!(
            parse_value("3.2").unwrap().1,
            Language::Value("3.2".to_string(), Type::Scalar(BaseType::Double)));
        assert_eq!(
            parse_value("-8.9").unwrap().1,
            Language::Value("-8.9".to_string(), Type::Scalar(BaseType::Double)));
    }


    #[test]
    fn test_integer() {
        assert_eq!(
            parse_value("7").unwrap().1,
            Language::Value("7".to_string(), Type::Scalar(BaseType::Integer)));
        assert_eq!(
            parse_value("3L").unwrap().1,
            Language::Value("3L".to_string(), Type::Scalar(BaseType::Integer)));
        assert_eq!(
            parse_value("-8").unwrap().1,
            Language::Value("-8".to_string(), Type::Scalar(BaseType::Integer)));
        assert_eq!(
            parse_value("-8L").unwrap().1,
            Language::Value("-8L".to_string(), Type::Scalar(BaseType::Integer)));
    }

    #[test]
    fn test_values(){
        assert_eq!(
            parse_value("TRUE").unwrap().1,
            Language::Value("TRUE".to_string(), BaseType::Logical)
           ); 
        assert_eq!(
            parse_value("FALSE").unwrap().1,
            Language::Value("FALSE".to_string(), BaseType::Logical)
           ); 
    }

}
