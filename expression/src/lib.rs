use value::parse_value;
use vector::parse_vector;
use list::parse_list;
use nom::branch::alt;
use nom::IResult;
use base_language::Language;


pub fn parse_expression(s: &str) -> IResult<&str,Language> {
    alt((
            parse_value,
            parse_vector,
            parse_list,
        ))(s)
}


#[cfg(test)]
mod tests {
    use super::*;
    use base_language::r#type::{Type, BaseType};
    use base_language::value::Value;

    #[test]
    fn test_expression_integer(){
        assert_eq!(
            parse_expression("-8L").unwrap().1,
            Language::Value(Value::new("-8L", Type::Scalar(BaseType::Integer))));
    }

    #[test]
    fn test_expression_vector(){
        assert_eq!(
            parse_expression("c(1, 2, 3, 4)").unwrap().1,
            Language::VectorArguments(vec![
                        Value::new("1", Type::Scalar(BaseType::Integer)),
                        Value::new("2", Type::Scalar(BaseType::Integer)),
                        Value::new("3", Type::Scalar(BaseType::Integer)),
                        Value::new("4", Type::Scalar(BaseType::Integer))
            ]));
    }

    #[test]
    fn test_expression_list(){
        assert_eq!(
            parse_expression("list(\"Red\", \"Green\", c(21,32,11), TRUE, 51.23)").unwrap().1,
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
}
