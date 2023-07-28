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
            _ => Type::Any
        }
    }
}
