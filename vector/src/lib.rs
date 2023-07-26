use nom::bytes::complete::tag;
use nom::sequence::{tuple, preceded};
use nom::IResult;
use base_language::Language;
use base_language::r#type::Type;
use nom::multi::many1;
use base_parser::parse_symbol;
use nom::branch::alt;
use value::parse_value;
use base_parser::parse_separator;

fn parse_vector_argument(s: &str) ->  IResult<&str, Language> {
    alt((
        parse_value,
        parse_symbol))(s)
}

fn parse_comma_vector_argument(s: &str) -> IResult<&str, Language> {
    preceded(parse_separator, parse_vector_argument)(s)
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
    use super::*;
    use base_language::r#type::BaseType;

    #[test]
    fn test1(){
        assert_eq!(
            parse_vector("c(1, 2, 3, 4)").unwrap().1,
            Language::VectorArguments(vec![
                        Language::Value("1".to_string(), Type::Scalar(BaseType::Integer)),
                        Language::Value("2".to_string(), Type::Scalar(BaseType::Integer)),
                        Language::Value("3".to_string(), Type::Scalar(BaseType::Integer)),
                        Language::Value("4".to_string(), Type::Scalar(BaseType::Integer))
            ], Type::Any)
                  );
    }

    #[test]
    fn test2(){
        assert_eq!(
            parse_vector("c(\"char\", 2, FALSE)").unwrap().1,
            Language::VectorArguments(vec![
                        Language::Value("\"char\"".to_string(), Type::Scalar(BaseType::Character)),
                        Language::Value("2".to_string(), Type::Scalar(BaseType::Integer)),
                        Language::Value("FALSE".to_string(), Type::Scalar(BaseType::Logical))
            ], Type::Any)
                  );
    }
}

