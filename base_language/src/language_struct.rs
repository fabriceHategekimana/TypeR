use crate::r#type::Type;

pub trait LanguageStruct {
    const TYPE: Type = Type::Any;
    fn get_type(&self) -> Type {
        Self::TYPE
    }
    fn get_term(&self) -> String;
}
