use crate::{error, values};

pub enum Node {
    // Atom floating point number, maps to Value::Atom
    Atom(f64),
    // Compile time known sets, maps to Value::Set
    Set(Vec<Node>),
    // Variables and builder names
    Ident(String),
    // A Builtin with name and rest
    Builtin {
        name: String,
        value: Box<Node>,
    },
    // Represents <x>..<y>, inclusive
    Range(Option<f64>, Option<f64>),
    // Represents builder { n[<-<prefeed>] | <predicate> }
    Builder {
        var: String,
        prefeed: Option<Box<Node>>,
        predicate: Box<Node>,
    },
}

impl Node {
    pub fn interpret(self) -> Result<values::Value, error::Error> {
        match self {
            Node::Atom(_) => self.try_into(),
            Node::Set(_) => todo!(),
            Node::Ident(_) => todo!(),
            Node::Builtin { .. } => todo!(),
            Node::Range(_, _) => todo!(),
            Node::Builder { .. } => todo!(),
        }
    }
}

#[cfg(test)]
mod ast {
    use crate::ast::Node;

    #[test]
    fn atoms() {
        Node::Atom(25f64).interpret().unwrap();
    }
}
