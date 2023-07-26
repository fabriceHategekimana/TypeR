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

    #[test]
    fn test_expression_integer(){
        assert_eq!(
            parse_expression("-8L").unwrap().1,
            Language::Value("-8L".to_string(), Type::Scalar(BaseType::Integer)));
    }

    #[test]
    fn test_expression_vector(){
        assert_eq!(
            parse_expression("c(1, 2, 3, 4)").unwrap().1,
            Language::VectorArguments(vec![
                        Language::Value("1".to_string(), Type::Scalar(BaseType::Integer)),
                        Language::Value("2".to_string(), Type::Scalar(BaseType::Integer)),
                        Language::Value("3".to_string(), Type::Scalar(BaseType::Integer)),
                        Language::Value("4".to_string(), Type::Scalar(BaseType::Integer))
            ], Type::Any)
                  );
    }

    #[test]
    fn test_expression_list(){
        assert_eq!(
            parse_expression("list(\"Red\", \"Green\", c(21,32,11), TRUE, 51.23)").unwrap().1,
            Language::ListArguments(vec![
                Language::Value("\"Red\"".to_string(), Type::Scalar(BaseType::Character)),
                Language::Value("\"Green\"".to_string(), Type::Scalar(BaseType::Character)),
                Language::VectorArguments(vec![
                                Language::Value("21".to_string(), Type::Scalar(BaseType::Integer)),
                                Language::Value("32".to_string(), Type::Scalar(BaseType::Integer)),
                                Language::Value("11".to_string(), Type::Scalar(BaseType::Integer))
                                ], Type::Any),
                Language::Value("TRUE".to_string(), Type::Scalar(BaseType::Logical)),
                Language::Value("51.23".to_string(), Type::Scalar(BaseType::Double))
            ], Type::Any));
    }
}
