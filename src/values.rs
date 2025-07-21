enum Value {
    // Smallest possible value, floating point number
    Atom(f64),
    // compile time known sets
    Set(Vec<Value>),
}
