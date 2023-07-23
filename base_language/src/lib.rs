#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]

type Name = String;

#[derive(PartialEq, Debug)]
pub enum Language {
    Value(Name, Type),
    Call(Name, Type),
    Assignement(Name, Type),
    Symbol(Name, Type),
    VectorArguments(Vec<Language>, Type),
    Vector(Name, Type),
}

fn join_arguments(v: &Vec<Language>) -> String {
    v.iter().map(|x| x.get_name()).collect::<Vec<String>>().join(",")
}

impl Language {
    pub fn get_name(&self) -> String {
        match self {
            Language::Value(n, t) => n.to_owned(),
            Language::Call(n, t) => n.to_owned(),
            Language::Assignement(n, t) => n.to_owned(),
            Language::Symbol(n, t) => n.to_owned(),
            Language::VectorArguments(v, t) => join_arguments(&v),
            Language::Vector(n, t) => n.to_owned(),
        }
    }

    fn get_type(&self) -> Type {
        match self {
            Language::Value(n, t) => t.clone(),
            Language::Call(n, t) => t.clone(),
            Language::Assignement(n, t) => t.clone(),
            Language::Symbol(n, t) => t.clone(),
            Language::VectorArguments(v, t) => t.clone(),
            Language::Vector(n, t) => t.clone(),
        }
    }

    // TODO find algorithm to infer the real type of the vector
    fn infer_type_helper(v: &Vec<Language>) -> Type {
        Type::Any
    }

    pub fn infer_type(&self) -> Type {
        match self {
            Language::VectorArguments(v, t) => Self::infer_type_helper(&v),
            l => l.get_type()
        }
    }
}

// TODO solve problem with Vector of vector (can't be vec of vec)
#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Logical,
    Integer,
    Double,
    Character,
    Complex,
    Any,
    Vector(Box<Type>)
}

#[derive(PartialEq, Debug)]
enum TypeError {
    field
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
