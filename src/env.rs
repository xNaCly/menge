use std::collections::HashMap;

use crate::values;

pub struct Env {
    pub variable_table: HashMap<String, values::Value>,
}

impl Default for Env {
    fn default() -> Self {
        let default_sets = vec![
            ("E".into(), values::Value::Set(vec![])),
            //
        ];
        Self {
            variable_table: default_sets.into_iter().collect(),
        }
    }
}
