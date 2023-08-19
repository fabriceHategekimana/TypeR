// The disjuctive normal form is there to
// help the checker to infer types
// each tpe from the type struct can be translated to this form
// Like that we can infer the subtypes and exclusions without needing and exhaustive value check
// (one by one// The disjuctive normal form is there to

use std::collections::HashSet;
use base_language::r#type::{Type, BaseType};

#[derive(Debug, Clone)]
pub struct NormalDisjunctiveForm {
    root: Vec<UnionRoot>,
    disjunctions: Vec<Disjunction>,
    remaining_type: Vec<Type>
}

pub trait NDF {
    fn new(root: Vec<UnionRoot>, disjunction: Vec<Disjunction>) -> Self;
    fn default() -> Self;
    fn from_type(t: Type) -> Self;
    fn set_type(&self, t: Type) -> Self;
    fn to_vector(&self) -> Self;
    fn add_na(&self, na: bool) -> Self;
    fn scalar_to_ndf(t: BaseType) -> Self;
    fn vector_to_ndf(t: BaseType, na: bool) -> Self ;
    fn list_to_ndf(t: Vec<Type>) -> Self;
    fn union_to_ndf(t: Vec<Type>) -> Self;
    fn function_to_ndf(args: Vec<Type>, ret: Box<Type>) -> Self;
    fn type_to_ndf(t: &str) -> Self;
    fn combine(&self, ndf: NormalDisjunctiveForm) -> Self;
    fn get_englobing_ndf(&self, ndf: Result<NormalDisjunctiveForm, (Type, Type)>) -> Result<Self, (Type, Type)> where Self: Sized;
    fn get_type(&self) -> Type;
}

impl NDF for NormalDisjunctiveForm {

    fn new(root: Vec<UnionRoot>, disjunctions: Vec<Disjunction>) -> NormalDisjunctiveForm {
        NormalDisjunctiveForm {
            root: root,
            disjunctions: disjunctions,
            remaining_type: vec![Type::Any]
        }
    }

    fn default() -> NormalDisjunctiveForm {
        NormalDisjunctiveForm { root: vec![], disjunctions: vec![], remaining_type: vec![] }
    }

    fn get_type(&self) -> Type {
        self.remaining_type[0].clone()
    }

    fn set_type(&self, t: Type) -> NormalDisjunctiveForm {
        NormalDisjunctiveForm { 
            root: self.root.clone(),
            disjunctions: self.disjunctions.clone(),
            remaining_type: vec![t]
        }
    }

    fn from_type(t: Type) -> NormalDisjunctiveForm {
       match t {
           Type::Scalar(s) => Self::scalar_to_ndf(s.clone()).set_type(Type::Scalar(s.clone())),
           Type::Vector(v, na) => Self::vector_to_ndf(v.clone(), na.clone()).set_type(Type::Vector(v.clone(), na.clone())),
           Type::List(vt) => Self::list_to_ndf(vt.clone()),//.set_type(Type::List(vt.clone())),
           Type::Union(vt) =>Self::union_to_ndf(vt.clone()).set_type(Type::Union(vt.clone())),
           Type::Function(args, ret) => Self::function_to_ndf(args.clone(), ret.clone()).set_type(Type::Function(args.clone(), ret.clone())),
           Type::Any =>  Self::default().set_type(Type::Any),
           Type::Null => Self::default().set_type(Type::Null),
           Type::Type(s) => Self::type_to_ndf(&s).set_type(Type::Type(s.clone())),
           Type::Nullable(t) => Self::default().set_type(Type::Null).combine(NormalDisjunctiveForm::from_type(*t))
       } 
    }

    fn type_to_ndf(_s: &str) -> NormalDisjunctiveForm {
        NormalDisjunctiveForm { root: vec![], disjunctions: vec![], remaining_type: vec![Type::Any] }
    }

    fn function_to_ndf(args: Vec<Type>, ret: Box<Type>) -> NormalDisjunctiveForm {
        NormalDisjunctiveForm {
           root: vec![UnionRoot::Function],
           disjunctions: vec![],
           remaining_type: combine_vec(args, vec![*ret]),
        }
    }

    fn union_to_ndf(t: Vec<Type>) -> NormalDisjunctiveForm {
       t.iter()
           .map(|t| <NormalDisjunctiveForm as NDF>::from_type(t.clone()))
           .fold(
               NormalDisjunctiveForm::default(),
               |ndf, ty| ndf.combine(ty))
    }

    fn combine(&self, ndf: NormalDisjunctiveForm) -> NormalDisjunctiveForm {
       NormalDisjunctiveForm { 
           root: combine_vec(self.root.clone(), ndf.root),
           disjunctions: combine_vec(self.disjunctions.clone(), ndf.disjunctions),
           remaining_type: vec![Type::Any]
       } 
    }

    fn get_englobing_ndf(&self, ndf: Result<NormalDisjunctiveForm, (Type, Type)>) -> Result<NormalDisjunctiveForm, (Type, Type)> {
        match ndf {
            Ok(ndf) => get_englobing_ndf_helper(self.clone(), ndf),
            Err(e) => Err(e)
        }
    }

    fn list_to_ndf(t: Vec<Type>) -> NormalDisjunctiveForm {
        NormalDisjunctiveForm {
            root: vec![UnionRoot::List],
            disjunctions: vec![],
            remaining_type: t
        }
    }

    fn vector_to_ndf(t: BaseType, na: bool) -> NormalDisjunctiveForm {
        Self::scalar_to_ndf(t).to_vector().add_na(na)
    }

    fn to_vector(&self) -> NormalDisjunctiveForm {
        NormalDisjunctiveForm { 
            root: self.root.clone(),
            disjunctions: combine_vec(self.disjunctions.clone(), vec![Disjunction::Vector]),
            remaining_type: self.remaining_type.clone()
        }
    }

    fn add_na(&self, na: bool) -> NormalDisjunctiveForm {
        let res = match na {
            true => self.disjunctions.iter()
                    .chain([Disjunction::Na].iter())
                    .map(|x| x.clone())
                    .collect::<Vec<Disjunction>>(),
            false => self.disjunctions.clone()
        };

        NormalDisjunctiveForm { 
            root: self.root.clone(), 
            disjunctions: res.clone(),
            remaining_type: self.remaining_type.clone()
        } 
    }

    fn scalar_to_ndf(t: BaseType) -> NormalDisjunctiveForm {
        match t {
            BaseType::Logical => NormalDisjunctiveForm::new(vec![UnionRoot::Number], vec![
                                        Disjunction::Logical,
            ]),
            BaseType::Integer => NormalDisjunctiveForm::new(vec![UnionRoot::Number], vec![
                                        Disjunction::Logical,
                                        Disjunction::Integer,
            ]),
            BaseType::Double => NormalDisjunctiveForm::new(vec![UnionRoot::Number], vec![
                                        Disjunction::Logical,
                                        Disjunction::Integer,
                                        Disjunction::Double,
            ]),
            BaseType::Complex => NormalDisjunctiveForm::new(vec![UnionRoot::Number], vec![
                                        Disjunction::Logical,
                                        Disjunction::Integer,
                                        Disjunction::Double,
                                        Disjunction::Complex,
            ]),
            BaseType::Character => NormalDisjunctiveForm::new(vec![UnionRoot::Char], vec![Disjunction::Character]),
            BaseType::Raw => NormalDisjunctiveForm::new(vec![UnionRoot::Raw], vec![Disjunction::Raw]),
        }
    }
}

fn get_englobing_ndf_helper(ndf1: NormalDisjunctiveForm, ndf2: NormalDisjunctiveForm) -> Result<NormalDisjunctiveForm, (Type, Type)> {
    match is_there_an_inclusion(&ndf1.root, &ndf2.root){
        true => return_englobing_ndf(ndf1, ndf2),
        false => Err((ndf1.remaining_type[0].clone(), ndf2.remaining_type[0].clone()))
    }
}

fn return_englobing_ndf(ndf1: NormalDisjunctiveForm, ndf2: NormalDisjunctiveForm) -> Result<NormalDisjunctiveForm, (Type, Type)> {
    let mut group = vec![ndf1.clone(), ndf2.clone()];
    group.sort_by(|x, y| x.disjunctions.len().partial_cmp(&y.disjunctions.len()).unwrap());
    match group[0].disjunctions.iter().all(|x| group[1].disjunctions.iter().any(|y| y == x)) {
        true => Ok(group[1].clone()),
        _ => Err((ndf1.remaining_type[0].clone(), ndf2.remaining_type[0].clone()))
    }
}

fn is_there_an_inclusion<T: PartialEq>(v1: &Vec<T>, v2: &Vec<T>) -> bool {
    let mut group = vec![v1, v2];
    group.sort_by(|x, y| x.len().partial_cmp(&y.len()).unwrap());
    group[0].iter().all(|x| group[1].iter().any(|y| y == x)) 
}

fn combine_vec<T: Eq + Clone>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
   let mut s: Vec<T> = vec![];
   for e in v1.iter().chain(v2.iter()) {
       if !s.iter().any(|x| x == e) {
            s.push(e.clone()) 
       }
   }
   s
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Disjunction {
   Logical,
   Integer,
   Character,
   Double,
   Complex,
   Raw,
   Null,
   Na,
   Vector,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum UnionRoot {
   Number,
   Char,
   Raw,
   List,
   Function,
}


