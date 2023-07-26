use base_language::Language;
use base_language::r#type::Type;
use nom::IResult;
use base_parser::{parse_type_annotation, parse_open_parenthesis, parse_close_parenthesis, parse_symbol};
use scope::parse_scope;
use nom::sequence::tuple;
use nom::bytes::complete::tag;
use nom::sequence::delimited;
use nom::multi::many1;
use base_language::symbol::Symbol;
use base_language::type_name::TypeName;

fn parse_function_argument(s: &str) -> IResult<&str,Language> {
    let res = tuple((
            parse_symbol,
            parse_type_annotation
          ))(s);
    match res {
        Ok((s, (sy, ta))) => Ok((s, Language::Identifier(
                    Symbol::new(&sy.get_name()),
                    TypeName::new(&ta.get_name())))),
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
        Ok((s, (_, v, tyan, sc))) => Ok((s, Language::Function(Box::new(v), TypeName::new(&tyan.get_name()), Box::new(sc), Type::Any))),
        Err(r) => Err(r)
    }
}

#[cfg(test)]
mod tests {
    use base_language::r#type::BaseType;
    use super::*;
    use base_language::symbol::Symbol;
    use base_language::type_name::TypeName;

  // `Function(FunctionArguments([Identifier(Symbol("a", Any), Reserved("int", Type))], Any), Reserved("chr", Type), ScopeElements
//([Value("\"Hello world\"", Scalar(Character))], Any), Any)`,

    #[test]
    fn test(){
        assert_eq!(
            parse_function("function(a: int) : chr { \"Hello world\" }").unwrap().1,
            Language::Function(
                Box::new(Language::FunctionArguments(vec![
                         Language::Identifier(
                                Symbol::new("a"),
                                TypeName::new("int"))
                ], Type::Any)),
                TypeName::new("chr"),
                Box::new(Language::ScopeElements(vec![
                         Language::Value("\"Hello world\"".to_string(), Type::Scalar(BaseType::Character))],
                         Type::Any)),
                Type::Any) 
                  );
    }
}
