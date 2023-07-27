pub type Na = bool;

// TODO solve problem with Vector of vector (can't be vec of vec)
#[derive(PartialEq, Debug, Clone)]
pub enum BaseType {
    Logical,
    Integer,
    Double,
    Character,
    Complex,
    Raw,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Scalar(BaseType),
    Vector(BaseType, Na),
    List(Vec<Type>),
    Union(Vec<Type>),
    Function(Vec<Type>, Box<Type>),
    Any,
    Null,
    Type
}

impl Type {
    pub fn from(s: &str) -> Type {
        match s {
            "int" => Type::Scalar(BaseType::Integer),
            _ => Type::Any
        }
    }
}
