use nom::IResult;
use nom::bytes::complete::tag;
use nom::sequence::tuple;
use nom::branch::alt;
use nom::sequence::preceded;
use nom::multi::many1;
use base_language::{Language, BaseType, Type};
use value::{parse_vector, parse_value};
use base_parser::{parse_separator, parse_symbol};

fn parse_list_parameter(s: &str) -> IResult<&str,Language> {
    alt((
            parse_value,
            parse_vector,
            parse_symbol,
        ))(s)
}

fn parse_list_parameters(s: &str) -> IResult<&str,Language> {
    let res = many1(
        alt((
            parse_list_parameter,
            preceded(parse_separator, parse_list_parameter),
            ))
         )(s);
    match res {
        Ok((s, v)) => Ok((s, Language::ListArguments(v, Type::Scalar(BaseType::Any)))),
        Err(r) => Err(r)
    }
}

fn parse_list(s: &str) -> IResult<&str,Language> {
    let res = tuple((
            tag("list("),
            parse_list_parameters,
            tag(")"),
          ))(s);
    match res {
        Ok((s, (_, p, _))) => Ok((s, p)),
        Err(r) => Err(r)
    }
}

#[cfg(test)]
mod tests {
    use super::parse_list;
    use base_language::{Language, Type, BaseType};

    #[test]
    fn test1(){
        assert_eq!(
            parse_list("list(\"Red\", \"Green\", c(21,32,11), TRUE, 51.23)").unwrap().1,
            Language::ListArguments(vec![
                Language::Value("\"Red\"".to_string(), Type::Scalar(BaseType::Character)),
                Language::Value("\"Green\"".to_string(), Type::Scalar(BaseType::Character)),
                Language::VectorArguments(vec![
                                Language::Value("21".to_string(), Type::Scalar(BaseType::Integer)),
                                Language::Value("32".to_string(), Type::Scalar(BaseType::Integer)),
                                Language::Value("11".to_string(), Type::Scalar(BaseType::Integer))
                                ], Type::Scalar(BaseType::Any)),
                Language::Value("TRUE".to_string(), Type::Scalar(BaseType::Logical)),
                Language::Value("51.23".to_string(), Type::Scalar(BaseType::Double))
            ], Type::Scalar(BaseType::Any)));
    }
}
