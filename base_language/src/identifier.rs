use crate::symbol::Symbol;
use crate::type_name::TypeName;
use crate::LanguageStruct;

#[derive(Debug, PartialEq)]
pub struct Identifier {
    symbol: String,
    type_name: String
}

impl Identifier {
    pub fn new(sym: &str, tyna: &str) -> Identifier {
        Identifier { symbol: sym.to_string(), type_name: tyna.to_string() }
    }
}

impl LanguageStruct for Identifier {
    fn get_term(&self) -> String {
        format!("{}: {}", self.symbol, self.type_name)
    }
}
