use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::{tuple, preceded};
use nom::character::complete::alphanumeric1;
use nom::multi::many1;
use nom::IResult;
use base_language::{Language, Type};
use logical::parse_logical;
use integer::parse_integer;
use double::parse_double;
use complex::parse_complex;
use character::parse_character;

fn parse_value(s: &str) -> IResult<&str, Language> {
    alt((
            parse_complex,
            parse_double,
            parse_integer,
            parse_logical,
            parse_character
        ))(s)
}

fn parse_symbol(s: &str) -> IResult<&str, Language> {
    let res = alphanumeric1(s);
    match res {
        Ok((s, n)) => Ok((s, Language::Symbol(n.to_string(), Type::Any))),
        Err(r) => Err(r)
    }
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
        Ok((s, v)) => Ok((s, Language::VectorArguments(v, Type::Any))),
        Err(r) => Err(r)
    }
}

fn parse_vector(s: &str) -> IResult<&str, Language> {
    let res = tuple((
            tag("c("),
            parse_vector_arguments,
            tag(")")
          ))(s);
    match res {
        Ok((s, (o, a, c))) => Ok((s, Language::Vector(format!("{}{}{}", o, a.get_name(), c), a.infer_type()))),
        Err(r) => Err(r)
    }
}

#[cfg(test)]
mod tests {
    use super::parse_value;
    use base_language::{Language, Type};
    use integer::parse_integer;

    #[test]
    fn test_character() {
        assert_eq!(
            parse_value("\"Hello\"").unwrap().1,
            Language::Value("\"Hello\"".to_string(), Type::Character));
    }

    #[test]
    fn test_complex() {
        assert_eq!(
            parse_value("3i").unwrap().1,
            Language::Value("3i".to_string(), Type::Complex)
                  );
    }

    #[test]
    fn test_double() {
        assert_eq!(
            parse_value("3.2").unwrap().1,
            Language::Value("3.2".to_string(), Type::Double));
        assert_eq!(
            parse_value("-8.9").unwrap().1,
            Language::Value("-8.9".to_string(), Type::Double));
    }


    #[test]
    fn test_integer() {
        assert_eq!(
            parse_value("7").unwrap().1,
            Language::Value("7".to_string(), Type::Integer));
        assert_eq!(
            parse_value("3L").unwrap().1,
            Language::Value("3L".to_string(), Type::Integer));
        assert_eq!(
            parse_value("-8").unwrap().1,
            Language::Value("-8".to_string(), Type::Integer));
        assert_eq!(
            parse_value("-8L").unwrap().1,
            Language::Value("-8L".to_string(), Type::Integer));
    }

    #[test]
    fn test_values(){
        assert_eq!(
            parse_value("TRUE").unwrap().1,
            Language::Value("TRUE".to_string(), Type::Logical)
           ); 
        assert_eq!(
            parse_value("FALSE").unwrap().1,
            Language::Value("FALSE".to_string(), Type::Logical)
           ); 
    }

}
