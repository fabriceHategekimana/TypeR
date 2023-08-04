#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]
// The goal is to do a type inference of types and spot incoherences

mod normal_disjunctive_form;
mod context;

use base_language::Language;
use base_language::r#type::Type;
use base_language::language_struct::LanguageStruct;
use base_language::value::Value;
use command::parse_command;
use normal_disjunctive_form::*;

fn check(adt: Language) -> (Checker, TypeInfo) {
    match adt {
        Language::Value(v) => (Checker::Correct, TypeInfo::Type(v.get_type())),
        Language::Symbol(_) => return_error(),
        Language::Reserved(_) => return_error(),
        Language::Identifier(i) => (Checker::Correct, TypeInfo::Definition(i.get_term(), i.get_type())),
        Language::VectorArguments(v) => unify_vector_arguments(v),
        _ => return_error()
    }
}

fn to_type(u: Vec<NormalDisjunctiveForm>) -> Type {
    Type::Any
}

fn to_union_type(v: impl LanguageStruct) -> Vec<NormalDisjunctiveForm> {
    // if a primitive value (term, type) -> (term, NormalDisjunctiveFormVariant)
    // if a symbol get type from context
    // if an expression check(expr) -> (Cheker, TypeInfo)
    //      if correct (term, TypeInfo) -> (term, NormalDisjunctiveFormVariant)
    //      else panic!(Checker::Error) 
    todo!();
}

type Ndf = (String, Result<NormalDisjunctiveForm, (Type, Type)>);

trait NdfWrapper {
    fn new(v: &Value) -> Ndf;
    fn default() -> Ndf;
    fn get_englobing_ndf(&self, ndf: Ndf) -> Ndf;
}

impl NdfWrapper for Ndf {
    fn new(v: &Value) -> Ndf {
        (v.get_term(),
        Ok(NormalDisjunctiveForm::from_type(v.get_type())))
    }

    fn get_englobing_ndf(&self, ndf: Ndf) -> Ndf {
        match &self.1 {
            Ok(s) => (self.0.clone(), s.get_englobing_ndf(ndf.1)),
            Err(e) => self.clone()
        }
    }

    fn default() -> Ndf {
        ("".to_string(), Ok(NormalDisjunctiveForm::default()))
    }
}


fn unify_vector_arguments(v: Vec<Value>) -> (Checker, TypeInfo) {
    //develop -> (term, Vec<NormalDisjunctiveForm>)
    let res = v.iter().map(|x| Ndf::new(x))
        .fold(Ndf::default(), |acc, x| x.get_englobing_ndf(acc));
   match res {
       (s, Ok(ndf))  => (Checker::Correct, TypeInfo::Type(ndf.get_type())),
       (s, Err((t1, t2))) => {
                        println!("{}", format!("Error: Mismatched types {:?} and {:?}", t1, t2));
                        (Checker::Error, TypeInfo::Empty)}
   } 
}

fn return_error() -> (Checker, TypeInfo) {
   (Checker::Error, TypeInfo::Empty) 
}

#[derive(PartialEq, Debug)]
enum TypeInfo {
    Type(Type),
    Definition(String, Type),
    Empty
}

#[derive(Debug, PartialEq)]
enum Checker {
    Correct,
    Error,
    Mismatch(Type, Type)
}


fn parse_and_check(s: &str) -> (Checker, TypeInfo) {
    let parsed = parse_command(s).unwrap().1;
    check(parsed)
}


#[cfg(test)]
mod tests {
    use super::*;
    use base_language::r#type::BaseType;
    use command::parse_command;
    use base_language::value::Value;
    use base_language::r#type::Type;
    use base_language::identifier::Identifier;

    #[test]
    fn test_shape() {
        let res = parse_command("a: int <- 7").unwrap().1;
        assert_eq!(res,
           Language::Assignement(
               Identifier::new("a", "int"),
               Box::new(
                    Language::Value(Value::new("7", Type::from("int")))
               )
           )
       );
    }

    #[test]
    fn test_value() {
        assert_eq!(parse_command("84").unwrap().1,
        Language::Value(Value::new("84", Type::from("int"))));
    }

    #[test]
    fn test_check_value(){
        assert_eq!(
            parse_and_check("7"),
            (Checker::Correct, TypeInfo::Type(Type::from("int"))));
    }

    #[test]
    fn test_check_vector() {
        assert_eq!(
            parse_and_check("c(1,2)"),
           (Checker::Correct, TypeInfo::Type(Type::from("int"))));
    }
    #[test]
    fn test_vector_good_coertion() {
        assert_eq!(
            parse_command("c(1,2.0)").unwrap().1,
            Language::VectorArguments(vec![
                Value::new("1", Type::from("int")),
                Value::new("2.0", Type::from("dbl")),
            ]));
    }

    #[test]
    fn test_check_vector_good_type_coertion1() {
        assert_eq!(
            parse_and_check("c(1,2.0)"),
           (Checker::Correct, TypeInfo::Type(Type::from("dbl"))));
    }

    #[test]
    fn test_check_vector_good_type_coertion2() {
        assert_eq!(
            parse_and_check("c(1,2.0,4i)"),
           (Checker::Correct, TypeInfo::Type(Type::from("clx"))));
    }
    #[test]
    fn test_check_vector_bad_type_coertion() {
        assert_eq!(
            parse_and_check("c(1,2.0,\"hey\")"),
           (Checker::Error, TypeInfo::Empty));
    }

    #[test]
    fn test_identifier() {
        assert_eq!(
            parse_command("a : int").unwrap().1,
            Language::Identifier(Identifier::new("a", "int")));
    }

}
