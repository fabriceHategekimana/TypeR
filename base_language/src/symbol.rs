use crate::LanguageStruct;

#[derive(Debug, PartialEq)]
pub struct Symbol {
    term: String,
}
impl LanguageStruct for Symbol {
    fn get_term(&self) -> String {
        self.term.clone()
    }
}
impl Symbol {
    pub fn new(s: &str) -> Symbol {
        Symbol { term: s.to_string() }
    }
}

