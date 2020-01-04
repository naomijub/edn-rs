#[macro_use] extern crate edn_rs;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::edn_rs::serialize::Serialize;

    #[test]
    fn serializes_a_complex_structure() {
        ser_struct!{
            #[derive(Debug)]
            struct Edn {
                map: HashMap<String, Vec<String>>,
                set: std::collections::HashSet<i64>,
                tuples: (i32, bool, char),
            }
        };
        let edn = Edn {
            map: map!{"this is a key".to_string() => vec!["with".to_string(), "many".to_string(), "keys".to_string()]},
            set: set!{3i64, 4i64, 5i64},
            tuples: (3i32, true, 'd')
        };

        println!("{}", edn.serialize())
    }

}