pub type Na = bool;

// TODO solve problem with Vector of vector (can't be vec of vec)
#[derive(PartialEq, Debug, Clone)]
pub enum BaseType {
    Logical,
    Integer,
    Double,
    Character,
    Complex,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Scalar(BaseType),
    Vector(BaseType, Na),
    List(Box<Type>),
    Union(Box<Type>),
    Function(Vec<Type>, Box<Type>),
    Any,
    Null,
    Type
}
