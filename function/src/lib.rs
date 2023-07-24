use base_language::{Language, Type, BaseType};
use nom::IResult;

pub fn parse_function(s: &str) -> IResult<&str,Language> {
    Ok((s, Language::Symbol("function".to_string(), Type::Scalar(BaseType::Any))))
}
