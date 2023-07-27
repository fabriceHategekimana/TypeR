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

trait NDF {
    fn new(root: Vec<UnionRoot>, disjunction: Vec<Disjunction>) -> Self;
    fn from(t: Type) -> Self;
    fn set_type(&self, t: Type) -> Self;
    fn to_vector(&self) -> Self;
    fn add_na(&self, na: bool) -> Self;
    fn scalar_to_ndf(t: BaseType) -> Self;
    fn vector_to_ndf(t: BaseType, na: bool) -> Self ;
    fn list_to_ndf(t: Vec<Type>) -> Self;
    fn combine(&self, ndf: NormalDisjunctiveForm) -> Self;

impl NDF for NormalDisjunctiveForm {

    fn new(root: Vec<UnionRoot>, disjunctions: Vec<Disjunction>) -> NormalDisjunctiveForm {
        NormalDisjunctiveForm {
            root: root,
            disjunctions: disjunctions,
            remaining_type: vec![Type::Any]
        }
    }

    fn set_type(&self, t: Type) -> NormalDisjunctiveForm {
        NormalDisjunctiveForm { 
            root: self.root.clone(),
            disjunctions: self.disjunctions.clone(),
            remaining_type: vec![t]
        }
    }

    fn from(t: Type) -> NormalDisjunctiveForm {
       match t {
           Type::Scalar(s) => Self::scalar_to_ndf(s),
           Type::Vector(v, na) => Self::vector_to_ndf(v, na),
           Type::List(t) => Self::list_to_ndf(t),
           Type::Union(u) =>Self::union_to_ndf(t),
       } 
    }

    fn union_to_ndf(tn: Vec<Type>) -> NormalDisjunctiveForm {
       tn.iter()
           .map(|t| NormalDisjunctiveForm::from(t))
           .fold(
               NormalDisjunctiveForm::default(),
               |acc, x| acc.combine(x))
    }

    fn combine(&self, ndf: NormalDisjunctiveForm) -> NormalDisjunctiveForm {
       NormalDisjunctiveForm { 
           root: combine(self.root, ndf.root),
           disjunctions: combine_vec(self.disjunctions, ndf.disjunctions),
           remaining_type: vec![Type::Any]
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
            root: vec![UnionRoot::Vector],
            disjunctions: self.disjunctions.clone(),
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

fn combine_vec<T: Eq + Clone>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
   let s: Vec<T> = vec![];
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
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum UnionRoot {
   Number,
   Vector,
   Char,
   Raw,
   List,
}
