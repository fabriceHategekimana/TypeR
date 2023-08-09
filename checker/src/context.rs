//Un objet context doit pouvoir contenier les informations de types sur les variables et les types
//meme
// Il y a deux tables: les alias, les variables

use base_language::r#type::{Type, BaseType};

#[derive(PartialEq, Debug)]
enum Origin {
    BaseType(Type),
    From(String)
}

impl Origin {
    fn get_string(&self) -> String {
        match self {
            Origin::BaseType(s) => s.get_string(),
            Origin::From(s) => s.to_string(),
        }
    }
}

struct Context {
    alias: Vec<(String, Origin)> ,
    variables: Vec<(String, String)> 
}

#[derive(PartialEq, Debug, Copy)]
enum Identity {
    Variable,
    Type,
    None,
}

impl Context {
    fn new() -> Context {
        Context {
            alias: vec![
                ("lgl".to_string(), Origin::BaseType(Type::from("lgl"))),
                ("int".to_string(), Origin::BaseType(Type::from("int"))),
                ("dbl".to_string(), Origin::BaseType(Type::from("dbl"))),
                ("clx".to_string(), Origin::BaseType(Type::from("clx"))),
                ("chr".to_string(), Origin::BaseType(Type::from("chr"))),
                ("raw".to_string(), Origin::BaseType(Type::from("raw"))),
                ("lgl[]".to_string(), Origin::BaseType(Type::from("lgl[]"))),
                ("int[]".to_string(), Origin::BaseType(Type::from("lgl[]"))),
                ("dbl[]".to_string(), Origin::BaseType(Type::from("dbl[]"))),
                ("clx[]".to_string(), Origin::BaseType(Type::from("clx[]"))),
                ("chr[]".to_string(), Origin::BaseType(Type::from("chr[]"))),
                ("raw[]".to_string(), Origin::BaseType(Type::from("raw[]"))),
                ("^lgl[]".to_string(), Origin::BaseType(Type::from("^lgl[]"))),
                ("^int[]".to_string(), Origin::BaseType(Type::from("^int[]"))),
                ("^dbl[]".to_string(), Origin::BaseType(Type::from("^dbl[]"))),
                ("^clx[]".to_string(), Origin::BaseType(Type::from("^clx[]"))),
                ("^chr[]".to_string(), Origin::BaseType(Type::from("^chr[]"))),
                ("^raw[]".to_string(), Origin::BaseType(Type::from("^raw[]"))),
            ],
            variables: vec![]
        }
    }

    fn get_type_of(&self, s: &str) -> Result<Type, String> {
        match self.identify(s) {
            Identity::Variable => Ok(self.get_variable_type(s)),
            Identity::Type => Ok(self.get_type_base_type(s)),
            Identity::None => Err(format!("'{}' is not a valide symbol", s)),
        }
    }
    
    fn identify(&self, s: &str) -> Identity {
        if self.is_a_variable(s) {
            return Identity::Variable;
        } else if self.is_a_type(s) {
            return Identity::Type;
        } else {
            return Identity::None;
        }
    }

    fn is_a_variable(&self, s: &str) -> bool {
        self.variables.iter().any(|(variable, _)| variable == s)
    }

    fn is_a_type(&self, s: &str) -> bool {
        self.alias.iter().any(|(typ, var)| typ == s || var.get_string() == s)
    }

    fn get_variable_type(&self, s: &str) -> Type {
        todo!();
    }

    fn get_type_base_type(&self) -> Type {
        todo!();
    }

}
