#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]
// The goal is to do a type inference of types and spot incoherences

mod normal_disjunctive_form;

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

fn unify_vector_arguments(v: Vec<Value>) -> (Checker, TypeInfo) {
    //develop -> (term, Vec<NormalDisjunctiveForm>)
    //fold get_englobing type -> Result<NormalDisjunctiveForm, Err((T1, T2))> with Ok(Bottom)
    //match
    //Ok(v) => (Checker::Correct, TypeInfo::Type(to_type(v)))
    //Err((t1, t2)) => (Checker::Mismatch(t1, t2))
    return_error()
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
    fn test_vector_argument() {
        assert_eq!(
            parse_command("c(1,2)").unwrap().1,
            Language::VectorArguments(vec![
                Value::new("1", Type::from("int")),
                Value::new("2", Type::from("int"))
            ])
                  );
    }

    #[test]
    fn test_check_vector() {
        assert_eq!(
            parse_and_check("c(1,2)"),
           (Checker::Correct, TypeInfo::Type(Type::from("int"))));
    }

}
