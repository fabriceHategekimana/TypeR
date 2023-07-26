use base_language::{Language, Type};
use base_parser::{parse_single_or, parse_symbol, parse_type};
use nom::multi::many1;
use nom::branch::alt;
use nom::sequence::preceded;
use nom::IResult;

fn parse_symbol_or_type(s: &str) -> IResult<&str,Language> {
    alt((
            parse_type,
            parse_symbol
        ))(s)
}

fn parse_single_or_and_symbol_or_type(s: &str) -> IResult<&str,Language> {
    preceded(parse_single_or, parse_symbol_or_type)(s)
}

pub fn parse_union_type(s: &str) -> IResult<&str,Language> {
    let res = many1(
        alt((
            parse_symbol_or_type,
            parse_single_or_and_symbol_or_type
            ))
        )(s);
    match res {
        Ok((s, v)) => Ok((s, Language::UnionArguments(v, Type::Type))),
        Err(r) => Err(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_types(){
        assert_eq!(
            parse_union_type("lgl | chr").unwrap().1,
            Language::UnionArguments(
                vec![
                    Language::Reserved("lgl".to_string(), Type::Type),
                    Language::Reserved("chr".to_string(), Type::Type),
                ]
                , Type::Type)
                  );
    }

    #[test]
    fn test_multiple_base_types(){
        assert_eq!(
            parse_union_type("lgl | chr | clx").unwrap().1,
            Language::UnionArguments(
                vec![
                    Language::Reserved("lgl".to_string(), Type::Type),
                    Language::Reserved("chr".to_string(), Type::Type),
                    Language::Reserved("clx".to_string(), Type::Type),
                ]
                , Type::Type)
                  );
    }

    #[test]
    fn test_multiple_vector_types(){
        assert_eq!(
            parse_union_type("lgl[] | chr[]").unwrap().1,
            Language::UnionArguments(
                vec![
                    Language::Reserved("lgl[]".to_string(), Type::Type),
                    Language::Reserved("chr[]".to_string(), Type::Type),
                ]
                , Type::Type)
                  );
    }

    #[test]
    fn test_multiple_nullable_types(){
        assert_eq!(
            parse_union_type("?lgl | ?chr[]").unwrap().1,
            Language::UnionArguments(
                vec![
                    Language::Reserved("?lgl".to_string(), Type::Type),
                    Language::Reserved("?chr[]".to_string(), Type::Type),
                ]
                , Type::Type)
                  );
    }

}
