use base_language::Language;
use base_language::r#type::Type;
use base_parser::{parse_open_bracket, parse_close_bracket};
use nom::character::complete::line_ending;
use nom::branch::alt;
use nom::IResult;
use nom::sequence::preceded;
use nom::multi::many1;
use nom::sequence::tuple;
use command::parse_command;


fn parse_new_line_and_command(s: &str) -> IResult<&str,Language> {
    preceded(line_ending, parse_command)(s)
}

fn parse_new_line_and_commands(s: &str) -> IResult<&str,Vec<Language>> {
    many1(alt((
                parse_new_line_and_command,
                parse_command
              )))(s)
}

pub fn parse_scope(s: &str) -> IResult<&str,Language> {
    let res = tuple((
            parse_open_bracket,
            parse_new_line_and_commands,
            parse_close_bracket
          ))(s);
    match res {
        Ok((s, (_, e, _))) => Ok((s, Language::ScopeElements(e, Type::Any))),
        Err(r) => Err(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base_language::r#type::BaseType;
    use base_language::symbol::Symbol;
    use base_language::type_name::TypeName;

    #[test]
    fn test_simple_scope(){
        assert_eq!(
            parse_scope("{ 12 }").unwrap().1,
            Language::ScopeElements(vec![
                Language::Value("12".to_string(), Type::Scalar(BaseType::Integer))
            ], Type::Any)
                  );
    }

    #[test]
    fn test_multiline_scope() {
        assert_eq!(
            parse_scope("{ 5\n7 }").unwrap().1,
            Language::ScopeElements(vec![
                Language::Value("5".to_string(), Type::Scalar(BaseType::Integer)),
                Language::Value("7".to_string(), Type::Scalar(BaseType::Integer)),
                        ], Type::Any)
                  );
    }

    #[test]
    fn test_multiline_scope2() {
        assert_eq!(
            parse_scope("{ a: lgl = TRUE\n8 }").unwrap().1,
            Language::ScopeElements(vec![
                Language::Assignement((
                        Box::new(Language::Identifier(
                                Symbol::new("a"), 
                                TypeName::new("lgl"), 
                                )), 
                        Box::new(Language::Value("TRUE".to_string(), Type::Scalar(BaseType::Logical)))), Type::Null),
                Language::Value("8".to_string(), Type::Scalar(BaseType::Integer)),
                        ], Type::Any)
                  );
    }


}
