use base_language::Language;
use nom::IResult;
use base_parser::{parse_type_annotation, parse_open_parenthesis, parse_close_parenthesis, parse_symbol};
use crate::parse_scope;
use nom::sequence::tuple;
use nom::bytes::complete::tag;
use nom::sequence::delimited;
use nom::multi::many1;
use base_language::type_name::TypeName;
use base_language::language_struct::LanguageStruct;
use base_language::identifier::Identifier;
use base_parser::parse_separator;
use nom::sequence::preceded;
use nom::branch::alt;

fn parse_function_argument_simple(s: &str) -> IResult<&str,Identifier> {
    let res = tuple((
            parse_symbol,
            parse_type_annotation
          ))(s);
    match res {
        Ok((s, (sy, ta))) => Ok((s, Identifier::new(&sy.get_term(), &ta.get_term()))),
        Err(r) => Err(r)
    }
}

fn parse_comma_and_function_argument(s: &str) -> IResult<&str, Identifier> {
    let res = preceded(parse_separator, parse_function_argument_simple)(s);
    match res {
        Ok((s, id)) => Ok((s,id)),
        Err(r) => Err(r)
    }
    
}

fn parse_function_argument(s: &str) -> IResult<&str,Identifier> {
    alt((
            parse_comma_and_function_argument,
            parse_function_argument_simple
        ))(s)
}

fn parse_function_arguments(s: &str) -> IResult<&str,Vec<Identifier>> {
   delimited(
            parse_open_parenthesis,
            many1(parse_function_argument),
            parse_close_parenthesis
          )(s)
}

pub fn parse_function(s: &str) -> IResult<&str,Language> {
    let res = tuple((
            tag("function"),
            parse_function_arguments,
            parse_type_annotation,
            parse_scope,
          ))(s);
    match res {
        Ok((s, (_, v, tyan, sc))) => Ok((s, Language::Function(v, TypeName::new(&tyan.get_term()), Box::new(sc)))),
        Err(r) => Err(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base_language::type_name::TypeName;
    use base_language::r#type::BaseType;
    use base_language::r#type::Type;
    use base_language::value::Value;

    #[test]
    fn test(){
        assert_eq!(
            parse_function("function(a: int) : chr { \"Hello world\" }").unwrap().1,
            Language::Function(
                vec![Identifier::new("a", "int")],
                TypeName::new("chr"),
                Box::new(Language::ScopeElements(vec![
                         Language::Value(Value::new("\"Hello world\"", Type::Scalar(BaseType::Character)))]))) 
                  );
    }
}
