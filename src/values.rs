enum Value {
    Atom(f64),
    Ident(String),
    Builtin(String, Box<Value>),
    // compile time known sets
    Set(Vec<Value>),
    // TODO: represent set builder notation
    // LazySet(),
}
