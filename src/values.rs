use std::rc::Rc;

use crate::ast;

#[derive(Clone)]
pub enum Value {
    // Smallest possible value, floating point number
    Atom(f64),
    // compile time known sets
    Set(Vec<Value>),
    // { <var>[<-feed] | <prec> }
    LazySet {
        // TODO: where do i represent ranges? see nodes::Node::Range(Option<f64>, Option<f64>), via
        // predicate, probably an iterator tbh
        var: String,
        feed: Box<Value>,
        pred: Rc<dyn Fn(&Value) -> bool>,
    },
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Atom(a), Value::Atom(b)) => a == b,
            (Value::Set(a), Value::Set(b)) => a == b,
            // Consider all LazySets unequal — or use pointer equality if you prefer
            (Value::LazySet { .. }, Value::LazySet { .. }) => false,
            _ => false,
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Atom(a) => write!(f, "Atom({a})"),
            Value::Set(v) => f.debug_list().entries(v).finish(),
            Value::LazySet { .. } => write!(f, "LazySet(...)"),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Atom(atom) => write!(f, "{atom}"),
            Value::Set(values) => {
                write!(f, "{{")?;
                for (i, val) in values.iter().enumerate() {
                    write!(f, "{val}")?;
                    if i + 1 != values.len() {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
            // TODO: This probably needs some consumption from like the first 10 elements and then
            // just elipsis
            Value::LazySet { .. } => todo!(),
        }
    }
}

impl TryFrom<ast::Node> for Value {
    type Error = crate::error::Error;

    fn try_from(value: ast::Node) -> Result<Self, Self::Error> {
        Ok(match value {
            ast::Node::Atom(atom) => Value::Atom(atom),
            _ => return Err(format!("Cannot convert {:?} to value::Value", value).into()),
        })
    }
}
