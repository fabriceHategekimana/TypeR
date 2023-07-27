use crate::Type;
use crate::LanguageStruct;

#[derive(Debug, PartialEq)]
pub struct Value {
    name: String,
    infered_type: Type
}

impl Value {
    pub fn new(name: &str, given_type: Type) -> Value {
        Value { name: name.to_string(), infered_type: given_type }
    }
}

impl LanguageStruct for Value {
    fn get_term(&self) -> String {
        self.name.clone()
    }
}
