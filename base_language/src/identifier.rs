use crate::symbol::Symbol;
use crate::type_name::TypeName;
use crate::LanguageStruct;
use crate::Type;

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    symbol: String,
    type_name: String
}

impl Identifier {
    pub fn new(sym: &str, tyna: &str) -> Identifier {
        Identifier { symbol: sym.to_string(), type_name: tyna.to_string() }
    }

    pub fn get_type_name(&self) -> String {
        self.type_name.clone()
    }
}

impl LanguageStruct for Identifier {
    fn get_term(&self) -> String {
        self.symbol.clone()
    }
}
