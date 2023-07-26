#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]

type Name = String;
type Na = bool;

#[derive(PartialEq, Debug)]
pub enum Language {
    Value(Name, Type),
    Call(Name, Type),
    Assignement((Box<Language>, Box<Language>), Type),
    Symbol(Name, Type),
    Identifier(Box<Language>, Box<Language>),
    Reserved(Name, Type),
    VectorArguments(Vec<Language>, Type),
    ListArguments(Vec<Language>, Type),
    ScopeElements(Vec<Language>, Type), // TODO: must only be symbol and reserved
    UnionArguments(Vec<Language>, Type), // TODO: must only be symbol and reserved
    Function(Box<Language>, Box<Language>, Box<Language>, Type),
    FunctionArguments(Vec<Language>, Type),
    Empty,
}

fn join_arguments(v: &Vec<Language>) -> String {
    v.iter().map(|x| x.get_name()).collect::<Vec<String>>().join(",")
}

impl Language {
    pub fn get_name(&self) -> String {
        match self {
            Language::Value(n, t) => n.to_owned(),
            Language::Call(n, t) => n.to_owned(),
            Language::Assignement(n, t) => "Todo".to_string(),
            Language::Symbol(n, t) => n.to_owned(),
            Language::Reserved(n, t) => n.to_owned(),
            Language::VectorArguments(v, t) => join_arguments(&v),
            Language::ListArguments(v, t) => join_arguments(&v),
            Language::ScopeElements(v, t) => join_arguments(&v),
            Language::UnionArguments(v, t) => join_arguments(&v),
            Language::Empty => "empty".to_string(),
            Language::Identifier(l1, l2) => l1.get_name(),
            Language::Function(a, n, s, t) => "function".to_string(),
            Language::FunctionArguments(v, t) => join_arguments(&v),
        }
    }

    fn get_type(&self) -> Type {
        match self {
            Language::Value(n, t) => t.clone(),
            Language::Call(n, t) => t.clone(),
            Language::Assignement(n, t) => t.clone(),
            Language::Symbol(n, t) => t.clone(),
            Language::Reserved(n, t) => t.clone(),
            Language::VectorArguments(v, t) => t.clone(),
            Language::ListArguments(v, t) => t.clone(),
            Language::ScopeElements(v, t) => t.clone(),
            Language::UnionArguments(v, t) => t.clone(),
            Language::Empty => Type::Null,
            Language::Identifier(l1, l2) => Type::Any,
            Language::Function(a, n, s, t) => t.clone(),
            Language::FunctionArguments(v, t) => t.clone(),
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
pub enum BaseType {
    Logical,
    Integer,
    Double,
    Character,
    Complex,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Scalar(BaseType),
    Vector(BaseType, Na),
    List(Box<Type>),
    Union(Box<Type>),
    Function(Vec<Type>, Box<Type>),
    Any,
    Null,
    Type
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test(){
        assert_eq!(
            Language::Value("hey".to_string(), Type::Scalar(BaseType::Logical)).get_name(),
            "hey".to_string()
            );
    }
}
