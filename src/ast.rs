enum Node {
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
