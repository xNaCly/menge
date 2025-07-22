use crate::{
    env, error,
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
    // <name> := <value>
    Assign {
        name: String,
        value: Box<Node>,
    },
    Binary {
        name: String,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Unary {
        name: String,
        lhs: Box<Node>,
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
    pub fn interpret(self, env: &mut env::Env) -> Result<values::Value, error::Error> {
        match self {
            Node::Atom(_) => self.try_into(),
            Node::Set(elements) => {
                let values: Result<Vec<_>, _> =
                    elements.into_iter().map(|n| n.interpret(env)).collect();
                Ok(Value::Set(values?))
            }
            Node::Ident(name) => env
                .variable_table
                .get(&name)
                .ok_or_else(|| format!("Unkown variable '{name}'").into())
                .map(Clone::clone),
            Node::Assign { .. } => todo!(),
            Node::Binary { .. } => todo!(),
            Node::Unary { .. } => todo!(),
            Node::Range(_, _) => todo!(),
            Node::Builder { .. } => todo!(),
        }
    }
}

#[cfg(test)]
mod ast {
    use crate::{ast::Node, env, values::Value};

    #[test]
    fn atoms() {
        let tests = vec![(Node::Atom(3.1415f64), Value::Atom(3.1415f64))];
        let mut env = env::Env::default();

        for (input, expected) in tests {
            let interpreted = input.interpret(&mut env).unwrap();
            println!("{interpreted}");
            assert_eq!(expected, interpreted);
        }
    }

    #[test]
    fn sets() {
        let tests = vec![
            (
                Node::Set(vec![Node::Atom(25f64), Node::Atom(24f64)]),
                Value::Set(vec![Value::Atom(25f64), Value::Atom(24f64)]),
            ),
            (
                Node::Set(vec![
                    Node::Set(vec![Node::Atom(25f64), Node::Atom(24f64)]),
                    Node::Set(vec![Node::Atom(25f64), Node::Atom(24f64)]),
                ]),
                Value::Set(vec![
                    Value::Set(vec![Value::Atom(25f64), Value::Atom(24f64)]),
                    Value::Set(vec![Value::Atom(25f64), Value::Atom(24f64)]),
                ]),
            ),
        ];

        let mut env = env::Env::default();

        for (input, expected) in tests {
            let interpreted = input.interpret(&mut env).unwrap();
            println!("{interpreted}");
            assert_eq!(expected, interpreted);
        }
    }
}
