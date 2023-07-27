use base_language::Language;
use base_parser::{parse_single_or, parse_symbol, parse_type};
use nom::multi::many1;
use nom::branch::alt;
use nom::sequence::preceded;
use nom::IResult;
use base_language::type_name::TypeName;

fn parse_symbol_or_type(s: &str) -> IResult<&str,TypeName> {
    let res = alt((
            parse_type,
            parse_symbol
        ))(s);
    match res {
        Ok((s, Language::Symbol(sy))) => Ok((s, TypeName::new(&sy))),
        Ok((s, Language::Reserved(n))) => Ok((s, TypeName::new(&n))),
        Ok((s, _)) => Ok((s, TypeName::new("_"))),
        Err(r) => Err(r)
    }
}

fn parse_single_or_and_symbol_or_type(s: &str) -> IResult<&str,TypeName> {
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
        Ok((s, v)) => Ok((s, Language::UnionArguments(v))),
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
                    TypeName::new("lgl"),
                    TypeName::new("chr"),
                ]));
    }

    #[test]
    fn test_multiple_base_types(){
        assert_eq!(
            parse_union_type("lgl | chr | clx").unwrap().1,
            Language::UnionArguments(
                vec![
                    TypeName::new("lgl"),
                    TypeName::new("chr"),
                    TypeName::new("clx"),
                ]));
    }

    #[test]
    fn test_multiple_vector_types(){
        assert_eq!(
            parse_union_type("lgl[] | chr[]").unwrap().1,
            Language::UnionArguments(
                vec![
                    TypeName::new("lgl[]"),
                    TypeName::new("chr[]"),
                ]));
    }

    #[test]
    fn test_multiple_nullable_types(){
        assert_eq!(
            parse_union_type("?lgl | ?chr[]").unwrap().1,
            Language::UnionArguments(
                vec![
                    TypeName::new("?lgl"),
                    TypeName::new("?chr[]"),
                ]));
    }

}
