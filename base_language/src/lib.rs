#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]

pub mod language_struct;
pub mod symbol;
pub mod type_name;
pub mod r#type;
pub mod identifier;
pub mod value;

use crate::language_struct::LanguageStruct;
use crate::symbol::Symbol;
use crate::type_name::TypeName;
use crate::identifier::Identifier;
use r#type::{Type, BaseType};
use crate::value::Value;

type Name = String;

#[derive(PartialEq, Debug, Clone)]
pub enum Language {
    Symbol(Name), 
    Reserved(Name), 
    Identifier(Identifier), 
    Value(Value),
    VectorArguments(Vec<Value>), 
    UnionArguments(Vec<TypeName>), 
    Assignement(Identifier, Box<Language>), // expression
    ListArguments(Vec<Language>), //list of expression
    ScopeElements(Vec<Language>), //list of command
    Function(Vec<Identifier>, TypeName, Box<Language>), // list of Identifier; TypeName; list of
                                                      // command
    FunctionArguments(Vec<Identifier>), // list of Identifier
    Empty,
}

fn join_arguments<L: LanguageStruct>(v: &Vec<L>) -> String {
    v.iter().map(|x| x.get_term()).collect::<Vec<String>>().join(",")
}

impl LanguageStruct for Language {
    fn get_term(&self) -> String {
        match self {
            Language::Assignement(i, e) => "Todo".to_string(),
            Language::Symbol(s) => s.to_string(),
            Language::Reserved(n) => n.to_owned(),
            Language::VectorArguments(v) => join_arguments(&v),
            Language::ListArguments(v) => join_arguments(&v),
            Language::ScopeElements(v) => join_arguments(&v),
            Language::UnionArguments(v) => join_arguments(&v),
            Language::Empty => "empty".to_string(),
            Language::Identifier(i) => i.get_term(),
            Language::Function(a, n, s) => "function".to_string(),
            Language::FunctionArguments(v) => join_arguments(&v),
            _ => "(not implemented yet)".to_string()
        }
    }

    fn get_type(&self) -> Type {
        match self {
            Language::Assignement(i, e) => Type::Null,
            Language::Symbol(s) => Type::Any,
            Language::Reserved(n) => Type::Any,
            Language::VectorArguments(v) => Type::Any,
            Language::ListArguments(v) => Type::Any,
            Language::ScopeElements(v) => Type::Any,
            Language::UnionArguments(v) => Type::Any,
            Language::Empty => Type::Null,
            Language::Identifier(i) => Type::Any,
            Language::Function(a, n, s) => Type::Any,
            Language::FunctionArguments(v) => Type::Any,
            _ => Type::Any
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test(){
        assert_eq!(
            Value::new("hey", Type::Scalar(BaseType::Logical)).get_term(),
            "hey".to_string()
            );
    }
}
