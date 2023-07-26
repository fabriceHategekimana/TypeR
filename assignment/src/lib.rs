use nom::IResult;
use nom::sequence::tuple;
use base_language::{Language, Type};
use expression::parse_expression;
use base_parser::{parse_assignement_symbol, parse_symbol, parse_colon, parse_type};
use nom::branch::alt;

fn parse_identifier(s: &str) -> IResult<&str,Language> {
    let res = tuple((
            parse_symbol,
            parse_colon,
            alt((parse_type, parse_symbol))
          ))(s);
    match res {
        Ok((s, (i, _, t))) => Ok((s, Language::Identifier(Box::new(i), Box::new(t)))),
        Err(r) => Err(r)
    }
}

pub fn parse_assignment(s: &str) -> IResult<&str,Language> {
    let res = tuple((
            parse_identifier,
            parse_assignement_symbol,
            parse_expression,
          ))(s);
    match res {
        Ok((s, (i, _, v))) => Ok((s, Language::Assignement((Box::new(i), Box::new(v)), Type::Null))),
        Err(r) => Err(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_type(){
        assert_eq!(
            parse_assignment("a: int <- 7").unwrap().1,
            Language::Assignement(
                (
                Box::new(
                    Language::Identifier(
                        Box::new(Language::Symbol("a".to_string(), Type::Any)),
                        Box::new(Language::Reserved("int".to_string(), Type::Type)))), 
                Box::new(
                    Language::Value("7".to_string(), Type::Scalar(BaseType::Integer)))
                ), Type::Null)
            );
    }

    #[test]
    fn test_custom_type(){
        assert_eq!(
            parse_assignment("a: Type <- 7").unwrap().1,
            Language::Assignement(
                (
                Box::new(
                    Language::Identifier(
                        Box::new(Language::Symbol("a".to_string(), Type::Any)),
                        Box::new(Language::Symbol("Type".to_string(), Type::Any)))), 
                Box::new(
                    Language::Value("7".to_string(), Type::Scalar(BaseType::Integer)))
                ), Type::Null)
            );
    }

    #[test]
    fn test_equal(){
        assert_eq!(
            parse_assignment("a : Type = 7").unwrap().1,
            Language::Assignement(
                (
                Box::new(
                    Language::Identifier(
                        Box::new(Language::Symbol("a".to_string(), Type::Any)),
                        Box::new(Language::Symbol("Type".to_string(), Type::Any)))), 
                Box::new(
                    Language::Value("7".to_string(), Type::Scalar(BaseType::Integer)))
                ), Type::Null)
            );
    }

    #[test]
    fn test_vector(){
        assert_eq!(
            parse_assignment("a : Type = c(1, 2)").unwrap().1,
            Language::Assignement(
                (
                Box::new(
                    Language::Identifier(
                        Box::new(Language::Symbol("a".to_string(), Type::Any)),
                        Box::new(Language::Symbol("Type".to_string(), Type::Any)))), 
                Box::new(
                    Language::VectorArguments(vec![
                                              Language::Value("1".to_string(), Type::Scalar(BaseType::Integer)),
                                              Language::Value("2".to_string(), Type::Scalar(BaseType::Integer)),
                    ], Type::Any))
                ), Type::Null)
            );
    }
}
