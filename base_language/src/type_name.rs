use crate::LanguageStruct;

#[derive(Debug, PartialEq, Clone)]
pub struct TypeName {
    term: String,
}
impl LanguageStruct for TypeName {
    fn get_term(&self) -> String {
        self.term.clone()
    }
}
impl TypeName {
    pub fn new(s: &str) -> TypeName {
        TypeName { term: s.to_string() }
    }
}
