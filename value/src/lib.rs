use nom::branch::alt;
use nom::IResult;
use base_language::Language;
use logical::parse_logical;
use integer::parse_integer;
use double::parse_double;
use complex::parse_complex;
use character::parse_character;

pub fn parse_value(s: &str) -> IResult<&str, Language> {
    alt((
            parse_complex,
            parse_double,
            parse_integer,
            parse_logical,
            parse_character
        ))(s)
}

#[cfg(test)]


mod tests {
    use super::*;
    use base_language::r#type::{Type, BaseType};
    use base_language::value::Value;

    #[test]
    fn test_character() {
        assert_eq!(
            parse_value("\"Hello\"").unwrap().1,
            Language::Value(Value::new("\"Hello\"", Type::Scalar(BaseType::Character))));
    }

    #[test]
    fn test_complex() {
        assert_eq!(
            parse_value("3i").unwrap().1,
            Language::Value(Value::new("3i", Type::Scalar(BaseType::Complex)))
                  );
    }

    #[test]
    fn test_double() {
        assert_eq!(
            parse_value("3.2").unwrap().1,
            Language::Value(Value::new("3.2", Type::Scalar(BaseType::Double))));
        assert_eq!(
            parse_value("-8.9").unwrap().1,
            Language::Value(Value::new("-8.9", Type::Scalar(BaseType::Double))));
    }


    #[test]
    fn test_integer() {
        assert_eq!(
            parse_value("7").unwrap().1,
            Language::Value(Value::new("7", Type::Scalar(BaseType::Integer))));
        assert_eq!(
            parse_value("3L").unwrap().1,
            Language::Value(Value::new("3L", Type::Scalar(BaseType::Integer))));
        assert_eq!(
            parse_value("-8").unwrap().1,
            Language::Value(Value::new("-8", Type::Scalar(BaseType::Integer))));
        assert_eq!(
            parse_value("-8L").unwrap().1,
            Language::Value(Value::new("-8L", Type::Scalar(BaseType::Integer))));
    }

    #[test]
    fn test_values(){
        assert_eq!(
            parse_value("TRUE").unwrap().1,
            Language::Value(Value::new("TRUE", Type::Scalar(BaseType::Logical)))
           ); 
        assert_eq!(
            parse_value("FALSE").unwrap().1,
            Language::Value(Value::new("FALSE", Type::Scalar(BaseType::Logical)))
           ); 
    }

}
