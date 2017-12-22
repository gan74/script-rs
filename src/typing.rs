
use std::fmt::Debug;
use std::rc::Rc;
use std::cmp::PartialEq;

trait Type: Debug {
}



#[derive(Debug, Clone)]
pub struct TypeRef {
    tpe: Option<Rc<Type>>
}

impl TypeRef {
    pub fn untyped() -> TypeRef {
        TypeRef {
            tpe: None
        }
    }

    pub fn is_typed(&self) -> bool {
        self.tpe.is_some()
    }

    pub fn as_type(&self) -> Option<&Type> {
        self.tpe.as_ref().map(|t| t.as_ref())
    }
}


impl PartialEq for TypeRef {
    fn eq(&self, rhs: &TypeRef) -> bool {
        match (&self.tpe, &rhs.tpe) {
            (&Some(ref a), &Some(ref b)) => Rc::ptr_eq(a, b),
            (&None, &None) => true,
            _ => false
        }
    }
}