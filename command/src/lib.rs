pub mod function;
pub mod scope;

use expression::parse_expression;
use nom::branch::alt;
use union::parse_union_type;
use nom::IResult;
use nom::sequence::tuple;
use base_language::Language;
use base_parser::{parse_type_annotation, parse_assignement_symbol, parse_symbol};
use base_language::identifier::Identifier;
use base_language::language_struct::LanguageStruct;
use scope::parse_scope;
use function::parse_function;

pub fn parse_command(s: &str) -> IResult<&str,Language> {
    alt((
            parse_expression,
            parse_assignment,
            parse_identifier,
            parse_function,
            parse_scope,
            parse_union_type,
        ))(s)
}

fn parse_identifier(s: &str) -> IResult<&str,Language> {
    let res = tuple((
            parse_symbol,
            parse_type_annotation
          ))(s);
    match res {
        Ok((s, (i, t))) => Ok((s, Language::Identifier(Identifier::new(&i.get_term(), &t.get_term())))),
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
        Ok((s, (Language::Identifier(sy), _, v))) => Ok((s, Language::Assignement(sy, Box::new(v)))),
        Ok((_s, _)) => todo!(),
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
    fn test_basic_type(){
        assert_eq!(
            parse_assignment("a: int <- 7").unwrap().1,
            Language::Assignement(
                    Identifier::new("a", "int"),
                    Box::new(Language::Value(Value::new("7", Type::Scalar(BaseType::Integer))))
                    ));
    }

    #[test]
    fn test_custom_type(){
        assert_eq!(
            parse_assignment("a: Type <- 7").unwrap().1,
            Language::Assignement(
                        Identifier::new("a", "Type"),
                        Box::new(Language::Value(Value::new("7", Type::Scalar(BaseType::Integer)))))
            );
    }

    #[test]
    fn test_equal(){
        assert_eq!(
            parse_assignment("a : Type = 7").unwrap().1,
            Language::Assignement(
                Identifier::new("a", "Type"),
                Box::new(
                    Language::Value(Value::new("7", Type::Scalar(BaseType::Integer)))
                    ))
            );
    }

    #[test]
    fn test_vector(){
        assert_eq!(
            parse_assignment("a : Type = c(1, 2)").unwrap().1,
            Language::Assignement(
                Identifier::new("a", "Type"),
                Box::new(
                    Language::VectorArguments(vec![
                              Value::new("1", Type::Scalar(BaseType::Integer)),
                              Value::new("2", Type::Scalar(BaseType::Integer)),
                    ]))
                )
            );
    }

    #[test]
    fn test_simple_vector() {
        assert_eq!(
            parse_command("c(1, 2, 3)").unwrap().1,
            Language::VectorArguments(vec![
                    Value::new("1", Type::Scalar(BaseType::Integer)),
                    Value::new("2", Type::Scalar(BaseType::Integer)),
                    Value::new("3", Type::Scalar(BaseType::Integer)),
                  ])
            );
    }

    #[test]
    fn test_identifier() {
        assert_eq!(
            parse_command("a : int").unwrap().1,
            Language::Identifier(Identifier::new("a", "int")));
    }

}
