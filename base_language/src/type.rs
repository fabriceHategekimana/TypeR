pub type Na = bool;

// TODO solve problem with Vector of vector (can't be vec of vec)
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum BaseType {
    Logical,
    Integer,
    Double,
    Character,
    Complex,
    Raw,
}

impl BaseType {
    fn get_string(&self) -> String {
        match self {
            BaseType::Logical => "lgl".to_string(),
            BaseType::Integer => "int".to_string(),
            BaseType::Double => "dbl".to_string(),
            BaseType::Character => "chr".to_string(),
            BaseType::Complex => "clx".to_string(),
            BaseType::Raw => "raw".to_string(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Type {
    Scalar(BaseType),
    Vector(BaseType, Na),
    List(Vec<Type>),
    Union(Vec<Type>),
    Function(Vec<Type>, Box<Type>),
    Any,
    Null,
    Nullable(Box<Type>),
    Type(String)
}

impl Type {
    pub fn from(s: &str) -> Type {
        match s {
            "int" => Type::Scalar(BaseType::Integer),
            "dbl" => Type::Scalar(BaseType::Double),
            "lgl" => Type::Scalar(BaseType::Logical),
            "chr" => Type::Scalar(BaseType::Character),
            "clx" => Type::Scalar(BaseType::Complex),
            "raw" => Type::Scalar(BaseType::Raw),
            "int[]" => Type::Vector(BaseType::Integer, false),
            "dbl[]" => Type::Vector(BaseType::Double, false),
            "lgl[]" => Type::Vector(BaseType::Logical, false),
            "chr[]" => Type::Vector(BaseType::Character, false),
            "clx[]" => Type::Vector(BaseType::Complex, false),
            "raw[]" => Type::Vector(BaseType::Raw, false),
            "^int[]" => Type::Vector(BaseType::Integer, true),
            "^dbl[]" => Type::Vector(BaseType::Double, true),
            "^lgl[]" => Type::Vector(BaseType::Logical, true),
            "^chr[]" => Type::Vector(BaseType::Character, true),
            "^clx[]" => Type::Vector(BaseType::Complex, true),
            "^raw[]" => Type::Vector(BaseType::Raw, true),
            _ => Type::Any
        }
    }

    pub fn get_string(&self) -> String {
        match self {
            Type::Scalar(base_type) => base_type.get_string(),
            Type::Vector(base_type, na) => format!("{}{}", na_to_string(*na), base_type.get_string()),
            Type::List(vec_of_type) => format!("List<{}>", vec_to_string(vec_of_type, ", ")) ,
            Type::Union(vec_of_type) => format!("{}", vec_to_string(vec_of_type, " | ")),
            Type::Function(vec_of_type, end_type) => format!("<{}> -> {}", vec_to_string(vec_of_type, ", "), end_type.get_string()),
            Type::Any => "Any".to_string(),
            Type::Null => "Null".to_string(),
            Type::Type(actual_type) => actual_type.clone(),
            Type::Nullable(ty) => format!("{}{}", "?", ty.get_string()),
        }
    }
}

fn vec_to_string(v: &[Type], sep: &str) -> String {
    v.iter().map(Type::get_string).collect::<Vec<String>>().join(sep)
}

fn na_to_string(na: bool) -> String {
    match na {
        true => "^".to_string(),
        false => "".to_string()
    }
}
