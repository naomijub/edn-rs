#![recursion_limit="512"]
#[macro_use] extern crate edn_rs;

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::edn_rs::serialize::Serialize;

    #[test]
    fn serializes_a_complex_structure() {
        ser_struct!{
            #[derive(Debug, Clone)]
            struct Edn {
                map: HashMap<String, Vec<String>>,
                set: HashSet<i64>,
                tuples: (i32, bool, char),
            }
        };
        let edn = Edn {
            map: map!{"this is a key".to_string() => vec!["with".to_string(), "many".to_string(), "keys".to_string()]},
            set: set!{3i64, 4i64, 5i64},
            tuples: (3i32, true, 'd')
        };

        assert!(edn.clone().serialize().contains(":map {:this-is-a-key [\"with\", \"many\", \"keys\"]}")
            && edn.clone().serialize().contains(":tuples (3, true, \\d),"));
    }
}