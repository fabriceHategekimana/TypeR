use nom::IResult;
use nom::branch::alt;
use nom::multi::many1;
use value::parse_value;
use base_language::Language;
use nom::bytes::complete::tag;
use base_parser::parse_symbol;
use base_language::value::Value;
use base_language::r#type::Type;
use base_parser::parse_separator;
use nom::sequence::{tuple, preceded};

fn parse_symbol_to_value(s: &str) -> IResult<&str, Value> {
   let res = parse_symbol(s);
   match res {
       Ok((s, Language::Symbol(sy))) => Ok((s, Value::new(&sy, Type::Any))),
       Ok((s, _)) => Ok((s, Value::new("NULL", Type::Null))),
       Err(r) => Err(r)
   }
}

fn parse_vector_argument(s: &str) ->  IResult<&str, Value> {
    alt((
        parse_value,
        parse_symbol_to_value))(s)
}

fn parse_comma_vector_argument(s: &str) -> IResult<&str, Value> {
    preceded(parse_separator, parse_vector_argument)(s)
}

fn parse_vector_arguments(s: &str) -> IResult<&str, Language> {
    let res = many1(alt((
            parse_comma_vector_argument,
            parse_vector_argument
              )))(s);
    match res {
        Ok((s, v)) => Ok((s, Language::VectorArguments(v))),
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
    use base_language::r#type::Type;
    use base_language::value::Value;

    #[test]
    fn test1(){
        assert_eq!(
            parse_vector("c(1, 2, 3, 4)").unwrap().1,
            Language::VectorArguments(vec![
                        Value::new("1", Type::Scalar(BaseType::Integer)),
                        Value::new("2", Type::Scalar(BaseType::Integer)),
                        Value::new("3", Type::Scalar(BaseType::Integer)),
                        Value::new("4", Type::Scalar(BaseType::Integer)),
            ])
                  );
    }

    #[test]
    fn test2(){
        assert_eq!(
            parse_vector("c(\"char\", 2, FALSE)").unwrap().1,
            Language::VectorArguments(vec![
                        Value::new("\"char\"", Type::Scalar(BaseType::Character)),
                        Value::new("2", Type::Scalar(BaseType::Integer)),
                        Value::new("FALSE", Type::Scalar(BaseType::Logical))
            ])
                  );
    }
}

