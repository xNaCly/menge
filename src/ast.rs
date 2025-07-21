use crate::{
    error,
    values::{self, Value},
};

#[derive(Debug)]
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
            Node::Set(elements) => {
                let values: Result<Vec<_>, _> =
                    elements.into_iter().map(|n| n.interpret()).collect();
                Ok(Value::Set(values?))
            }
            Node::Ident(_) => todo!(),
            Node::Builtin { .. } => todo!(),
            Node::Range(_, _) => todo!(),
            Node::Builder { .. } => todo!(),
        }
    }
}

#[cfg(test)]
mod ast {
    use crate::{ast::Node, values::Value};

    #[test]
    fn atoms() {
        let tests = vec![(Node::Atom(3.1415f64), Value::Atom(3.1415f64))];

        for (input, expected) in tests {
            assert_eq!(expected, input.interpret().unwrap());
        }
    }

    #[test]
    fn sets() {
        let tests = vec![(
            Node::Set(vec![Node::Atom(25f64), Node::Atom(24f64)]),
            Value::Set(vec![Value::Atom(25f64), Value::Atom(24f64)]),
        )];

        for (input, expected) in tests {
            assert_eq!(expected, input.interpret().unwrap());
        }
    }
}
