use nom::IResult;
use nom::sequence::tuple;
use base_language::{Language, Type, BaseType};
use value::parse_value;
use base_parser::{parse_assignement_symbol};

fn parse_identifier(s: &str) -> IResult<&str,Language> {
    Ok((s, Language::Empty))
}

fn parse_assignement(s: &str) -> IResult<&str,Language> {
    let res = tuple((
            parse_identifier,
            parse_assignement_symbol,
            parse_value,
          ))(s);
    match res {
        Ok((s, (i, _, v))) => Ok((s, Language::Assignement((i, v), Type::Scalar(BaseType::Null)))),
        Err(r) => Err(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test(){
        assert_eq!(
            parse_assignement("a: Integer <- 7").unwrap().1,
            Language::Assignement((
                Language::Symbol("a".to_string(), Type::Scalar(BaseType::Integer)),
                Language::Value("7".to_string(), Type::Scalar(BaseType::Integer)),
), Type::Scalar(BaseType::Null)
                  ));
    }
}
