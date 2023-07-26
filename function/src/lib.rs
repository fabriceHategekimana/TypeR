use base_language::{Language, Type};
use nom::IResult;
use base_parser::{parse_type_annotation, parse_open_parenthesis, parse_close_parenthesis, parse_symbol};
use scope::parse_scope;
use nom::sequence::tuple;
use nom::bytes::complete::tag;
use nom::sequence::delimited;
use nom::multi::many1;

fn parse_function_argument(s: &str) -> IResult<&str,Language> {
    let res = tuple((
            parse_symbol,
            parse_type_annotation
          ))(s);
    match res {
        Ok((s, (sy, ta))) => Ok((s, Language::Identifier(
                    Box::new(sy),
                    Box::new(ta)))),
        Err(r) => Err(r)
    }
}

fn parse_function_arguments(s: &str) -> IResult<&str,Language> {
    let res = delimited(
            parse_open_parenthesis,
            many1(parse_function_argument),
            parse_close_parenthesis
          )(s);
    match res {
        Ok((s, v)) => Ok((s, Language::FunctionArguments(v, Type::Any))),
        Err(r) => Err(r)
    }
}

pub fn parse_function(s: &str) -> IResult<&str,Language> {
    let res = tuple((
            tag("function"),
            parse_function_arguments,
            parse_type_annotation,
            parse_scope,
          ))(s);
    match res {
        Ok((s, (_, v, type_annot, sc))) => Ok((s, Language::Function(Box::new(v), Box::new(type_annot), Box::new(sc), Type::Any))),
        Err(r) => Err(r)
    }
}

#[cfg(test)]
mod tests {
    use base_language::BaseType;

    use super::*;

  // `Function(FunctionArguments([Identifier(Symbol("a", Any), Reserved("int", Type))], Any), Reserved("chr", Type), ScopeElements
//([Value("\"Hello world\"", Scalar(Character))], Any), Any)`,

    #[test]
    fn test(){
        assert_eq!(
            parse_function("function(a: int) : chr { \"Hello world\" }").unwrap().1,
            Language::Function(
                Box::new(Language::FunctionArguments(vec![
                         Language::Identifier(
                                Box::new(Language::Symbol("a".to_string(), Type::Any)),
                                Box::new(Language::Reserved("int".to_string(), Type::Type)))
                ], Type::Any)),
                Box::new(Language::Reserved("chr".to_string(), Type::Type)),
                Box::new(Language::ScopeElements(vec![
                         Language::Value("\"Hello world\"".to_string(), Type::Scalar(BaseType::Character))],
                         Type::Any)),
                Type::Any) 
                  );
    }
}
