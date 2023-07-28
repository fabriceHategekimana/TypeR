use base_language::Language;
use base_language::language_struct::LanguageStruct;
use command::parse_command;
use base_language::value::Value;
use base_language::identifier::Identifier;
use base_language::type_name::TypeName;

fn transpile(l: &Language) -> String {
   match l {
       Language::Value(v) => v.get_term(),
       Language::Empty => "".to_string(),
       Language::Symbol(s) => s.to_string(),
       Language::Reserved(r) => r.to_string(),
       Language::Identifier(id) => id.get_term(), 
       Language::VectorArguments(args) => format_vector_arguments(args),
       Language::UnionArguments(_) => "".to_string(),
       Language::Assignement(id, exp) => format!("{} <- {}", id.get_term(), transpile(exp)),
       Language::ListArguments(args) => format_list_arguments(args),
       Language::ScopeElements(lines) => format_scope_elements(lines),
       Language::Function(id, ty, exp) => format_function(id, ty, exp),
       _ => "No transpilation avaliable".to_string()
   } 
}

fn format_function(id: &[Identifier], ty: &TypeName, exp: &Language) -> String {
    let args = id.iter().map(|x| x.get_term()).collect::<Vec<String>>().join(", ");
    format!("function({}) {}", args, transpile(exp))
}

fn format_scope_elements(lines: &[Language]) -> String {
    let res = lines.iter().map(transpile).collect::<Vec<String>>().join("\n");
    format!("{} {} {}", "{", res, "}")
}

fn format_list_arguments(args: &[Language]) -> String {
    let res = args.iter().map(transpile).collect::<Vec<String>>().join(", ");
    format!("list({})", res)
}

fn format_vector_arguments(args: &[Value]) -> String {
    let res = args.iter().map(|x| x.get_term()).collect::<Vec<String>>();
    format!("c({})", res.join(", "))
}

fn parse_and_transpile(s: &str) -> String {
    let adt = parse_command(s).unwrap().1;
    transpile(&adt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base_language::r#type::Type;
    use command::function::parse_function;

    #[test]
    fn test(){
        assert_eq!(
            parse_and_transpile("7"),
            "7".to_string());
    }

    #[test]
    fn test_identifier() {
        assert_eq!(
            parse_and_transpile("a : int"),
            "a".to_string()
        );
    }

    #[test]
    fn test_assignement() {
        assert_eq!(
            parse_and_transpile("a : int <- 9"),
            "a <- 9".to_string()
        );
    }

    #[test]
    fn test_vector() {
        assert_eq!(
            parse_and_transpile("c(1, 2.0, 3)"),
            "c(1, 2.0, 3)".to_string()
        );
    }

    #[test]
    fn test_list() {
        assert_eq!(
            parse_and_transpile("list(1, 2.0, 3)"),
            "list(1, 2.0, 3)".to_string()
        );
    }

    #[test]
    fn test_scope_elements() {
        assert_eq!(
            parse_and_transpile("{ a: int <- 7\nb: dbl <- list(1, 2.0, 3)\nTRUE }"),
            "{ a <- 7\nb <- list(1, 2.0, 3)\nTRUE }".to_string()
        );
    }

    #[test]
    fn test_function0() {
        assert_eq!(
            parse_command("function(a: lgl, b: int) -> int { c(3, 4) }").unwrap().1,
            Language::Function(
                vec![
                    Identifier::new("a", "lgl"),
                    Identifier::new("b", "int")],
                TypeName::new("int"),
                Box::new(
                    Language::ScopeElements(vec![
                        Language::VectorArguments(vec![
                            Value::new("3", Type::from("int")),
                            Value::new("4", Type::from("int"))])
                    ])
                        )
                  ));
    }

    #[test]
    fn test_function() {
        assert_eq!(
            parse_and_transpile("function(a: lgl, b: int) -> int { c(a, b) }"),
            "function(a, b) { c(a, b) }".to_string());
    }

}
