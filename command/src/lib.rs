use expression::parse_expression;
use nom::branch::alt;
use union::parse_union_type;
use nom::IResult;
use nom::sequence::tuple;
use base_language::Language;
use base_language::r#type::Type;
use base_parser::{parse_type_annotation, parse_assignement_symbol, parse_symbol};
use base_language::symbol::Symbol;
use base_language::type_name::TypeName;

pub fn parse_command(s: &str) -> IResult<&str,Language> {
    alt((
            parse_expression,
            parse_assignment,
            parse_union_type,
        ))(s)
}

fn parse_identifier(s: &str) -> IResult<&str,Language> {
    let res = tuple((
            parse_symbol,
            parse_type_annotation
          ))(s);
    match res {
        Ok((s, (i, t))) => Ok((s, Language::Identifier(Symbol::new(&i.get_name()), TypeName::new(&t.get_name())))),
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
    use base_language::r#type::BaseType;

    #[test]
    fn test_basic_type(){
        assert_eq!(
            parse_assignment("a: int <- 7").unwrap().1,
            Language::Assignement(
                (
                Box::new(
                    Language::Identifier(
                        Symbol::new("a"),
                        TypeName::new("int"))), 
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
                        Symbol::new("a"),
                        TypeName::new("Type"))), 
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
                        Symbol::new("a"),
                        TypeName::new("Type"))), 
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
                        Symbol::new("a"),
                        TypeName::new("Type"))), 
                Box::new(
                    Language::VectorArguments(vec![
                                              Language::Value("1".to_string(), Type::Scalar(BaseType::Integer)),
                                              Language::Value("2".to_string(), Type::Scalar(BaseType::Integer)),
                    ], Type::Any))
                ), Type::Null)
            );
    }
}
