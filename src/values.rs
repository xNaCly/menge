use crate::ast;

#[derive(Debug, PartialEq)]
pub enum Value {
    // Smallest possible value, floating point number
    Atom(f64),
    // compile time known sets
    Set(Vec<Value>),
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
