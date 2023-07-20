#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]

type Name = String;

#[derive(PartialEq, Debug)]
pub enum Language {
    Value(Name, Type),
    Call(Name, Type),
    Assignement(Name, Type)
}

impl Language {
    fn get_name(&self) -> String {
        match self {
            Language::Value(n, t) => n.to_owned(),
            Language::Call(n, t) => n.to_owned(),
            Language::Assignement(n, t) => n.to_owned(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Type {
    Logical,
    Integer,
    Double,
    Character,
    Complex,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test(){
        assert_eq!(
            Language::Value("hey".to_string(), Type::Logical).get_name(),
            "hey".to_string()
            );
    }
}
