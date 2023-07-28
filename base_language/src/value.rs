use crate::Language;
use crate::Type;
use crate::LanguageStruct;

#[derive(Debug, PartialEq, Clone)]
pub struct Value {
    name: String,
    infered_type: Type
}

impl Value {
    pub fn new(name: &str, given_type: Type) -> Value {
        Value { name: name.to_string(), infered_type: given_type }
    }

    pub fn language(name: &str, given_type: Type) -> Language {
        Language::Value(Value { name: name.to_string(), infered_type: given_type })
    }
}

impl LanguageStruct for Value {
    fn get_term(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> Type {
        self.infered_type.clone()
    }
}
