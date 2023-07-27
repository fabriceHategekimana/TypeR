use nom::IResult;
use nom::bytes::complete::tag;
use nom::sequence::tuple;
use nom::branch::alt;
use nom::sequence::preceded;
use nom::multi::many1;
use nom::combinator::opt; 
use base_language::Language;
use value::parse_value_to_language;
use vector::parse_vector;
use base_parser::{parse_separator, parse_symbol};


fn parse_list_parameter(s: &str) -> IResult<&str,Language> {
    alt((
            parse_list,
            parse_vector,
            parse_value_to_language,
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
        Ok((s, v)) => Ok((s, Language::ListArguments(v))),
        Err(r) => Err(r)
    }
}

pub fn parse_list(s: &str) -> IResult<&str,Language> {
    let res = tuple((
            tag("list("),
            opt(parse_list_parameters),
            tag(")"),
          ))(s);
    match res {
        Ok((s, (_, Some(p), _))) => Ok((s, p)),
        Ok((s, (_, None, _))) => Ok((s, Language::ListArguments(vec![]))),
        Err(r) => Err(r)
    }
}

#[cfg(test)]
mod tests {
    use super::parse_list;
    use base_language::Language;
    use base_language::r#type::{Type, BaseType};
    use base_language::value::Value;

    #[test]
    fn test1(){
        assert_eq!(
            parse_list("list(\"Red\", \"Green\", c(21,32,11), TRUE, 51.23)").unwrap().1,
            Language::ListArguments(vec![
                Language::Value(Value::new("\"Red\"", Type::Scalar(BaseType::Character))),
                Language::Value(Value::new("\"Green\"", Type::Scalar(BaseType::Character))),
                Language::VectorArguments(vec![
                                Value::new("21", Type::Scalar(BaseType::Integer)),
                                Value::new("32", Type::Scalar(BaseType::Integer)),
                                Value::new("11", Type::Scalar(BaseType::Integer))
                                ]),
                Language::Value(Value::new("TRUE", Type::Scalar(BaseType::Logical))),
                Language::Value(Value::new("51.23", Type::Scalar(BaseType::Double)))
            ]));
    }


    #[test]
    fn test_list_of_list() {
        assert_eq!(
            parse_list("list(list(3, 4), 5)").unwrap().1,
            Language::ListArguments(vec![
                    Language::ListArguments(vec![
                            Language::Value(Value::new("3", Type::Scalar(BaseType::Integer))),
                            Language::Value(Value::new("4", Type::Scalar(BaseType::Integer)))
                    ]),
            Language::Value(Value::new("5", Type::Scalar(BaseType::Integer)))]
            ,
                  ));
    }

    #[test]
    fn test_empty_list() {
        assert_eq!(
            parse_list("list()").unwrap().1,
            Language::ListArguments(vec![])
            );
    }
}
