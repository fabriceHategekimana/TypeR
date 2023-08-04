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

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Type {
    Scalar(BaseType),
    Vector(BaseType, Na),
    List(Vec<Type>),
    Union(Vec<Type>),
    Function(Vec<Type>, Box<Type>),
    Any,
    Null,
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

    fn get_string(&self) -> String {
        match element {
            Scalar(BaseType) => BaseType.get_string(),
            Vector(BaseType, Na) => format!("{}{}", boolean_to_na(Na), BaseType.get_string()),
            List(Vec<Type>),
            Union(Vec<Type>),
            Function(Vec<Type>, Box<Type>),
            Any,
            Null,
            Type(String),
            case => body,
            _ => default_body
        }
    }
}
