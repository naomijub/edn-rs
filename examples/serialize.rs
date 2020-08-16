use edn_rs::{hmap, hset, map, ser_struct, set, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

ser_struct! {
#[derive(Debug, Clone)]
struct Foo {
    value: usize,
}
}

fn main() {
    ser_struct! {
        #[derive(Debug, Clone)]
        struct Edn {
            btreemap: BTreeMap<String, Vec<String>>,
            btreeset: BTreeSet<i64>,
            hashmap: HashMap<String, Vec<String>>,
            hashset: HashSet<i64>,
            tuples: (i32, bool, char),
            foo_vec: Vec<Foo>,
        }
    };
    let edn = Edn {
        btreemap: map! {"this is a key".to_string() => vec!["with".to_string(), "many".to_string(), "keys".to_string()]},
        btreeset: set! {3i64, 4i64, 5i64},
        hashmap: hmap! {"this is a key".to_string() => vec!["with".to_string(), "many".to_string(), "keys".to_string()]},
        hashset: hset! {3i64},
        tuples: (3i32, true, 'd'),
        foo_vec: vec![Foo { value: 2 }, Foo { value: 3 }],
    };

    println!("{}", edn_rs::to_string(edn));
    // { :btreemap {:this-is-a-key [\"with\", \"many\", \"keys\"]}, :btreeset #{3, 4, 5}, :hashmap {:this-is-a-key [\"with\", \"many\", \"keys\"]}, :hashset #{3}, :tuples (3, true, \\d), :foo-vec [{ :value 2, }, { :value 3, }], }
}
