use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::IResult;
use base_language::r#type::{Type, BaseType};
use base_language::value::Value;

pub fn parse_logical(s: &str) -> IResult<&str, Value> {
    let res = alt((
        tag("TRUE"),
        tag("FALSE"),
        ))(s);
    match res {
        Ok((s, c)) => Ok((s, Value::new(c, Type::Scalar(BaseType::Logical)))),
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use base_language::r#type::{Type, BaseType};
    use base_language::value::Value;
    use super::parse_logical;
    use nom;

    #[test]
    fn test_values(){
        assert_eq!(
            parse_logical("TRUE").unwrap().1,
            Value::new("TRUE", Type::Scalar(BaseType::Logical))
           ); 
        assert_eq!(
            parse_logical("FALSE").unwrap().1,
            Value::new("FALSE", Type::Scalar(BaseType::Logical))
           ); 
    }

    #[test]
    fn test_wrong_value(){
        let res = parse_logical("T");
        assert_eq!(
            res,
            Err(nom::Err::Error(nom::error::Error{
                    input: "T",
                    code: nom::error::ErrorKind::Tag,
                }
                 )));
    }

}
