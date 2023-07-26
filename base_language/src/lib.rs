#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]

pub mod language_struct;
pub mod symbol;
pub mod type_name;
pub mod r#type;

use crate::language_struct::LanguageStruct;
use crate::symbol::Symbol;
use crate::type_name::TypeName;
use r#type::{Type, BaseType};

type Name = String;

#[derive(PartialEq, Debug)]
pub enum Language {
    Value(Name, Type),
    Call(Name, Type),
    Assignement((Box<Language>, Box<Language>), Type),
    Symbol(Symbol),
    Identifier(Symbol, TypeName),
    Reserved(Name, Type),
    VectorArguments(Vec<Language>, Type),
    ListArguments(Vec<Language>, Type),
    ScopeElements(Vec<Language>, Type), // TODO: must only be symbol and reserved
    UnionArguments(Vec<Language>, Type), // TODO: must only be symbol and reserved
    Function(Box<Language>, TypeName, Box<Language>, Type),
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
            Language::Symbol(s) => s.get_term(),
            Language::Reserved(n, t) => n.to_owned(),
            Language::VectorArguments(v, t) => join_arguments(&v),
            Language::ListArguments(v, t) => join_arguments(&v),
            Language::ScopeElements(v, t) => join_arguments(&v),
            Language::UnionArguments(v, t) => join_arguments(&v),
            Language::Empty => "empty".to_string(),
            Language::Identifier(l1, l2) => l1.get_term(),
            Language::Function(a, n, s, t) => "function".to_string(),
            Language::FunctionArguments(v, t) => join_arguments(&v),
        }
    }

    fn get_type(&self) -> Type {
        match self {
            Language::Value(n, t) => t.clone(),
            Language::Call(n, t) => t.clone(),
            Language::Assignement(n, t) => t.clone(),
            Language::Symbol(s) => s.get_type(),
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
