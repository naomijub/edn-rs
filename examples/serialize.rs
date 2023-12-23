use edn_derive::Serialize;
use edn_rs::{hmap, hset, map, set};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[derive(Debug, Clone, Serialize)]
struct Foo {
    value: u64,
}

#[derive(Debug, Clone, Serialize)]
struct Edn {
    btreemap: BTreeMap<String, Vec<String>>,
    btreeset: BTreeSet<i64>,
    hashmap: HashMap<String, Vec<String>>,
    hashset: HashSet<i64>,
    tuples: (i32, bool, char),
    foo_vec: Vec<Foo>,
    nothing: (),
}

fn serialize() -> String {
    let edn = Edn {
        btreemap: map! {"this is a key".to_string() => vec!["with".to_string(), "many".to_string(), "keys".to_string()]},
        btreeset: set! {3i64, 4i64, 5i64},
        hashmap: hmap! {"this is a key".to_string() => vec!["with".to_string(), "many".to_string(), "keys".to_string()]},
        hashset: hset! {3i64},
        tuples: (3i32, true, 'd'),
        foo_vec: vec![Foo { value: 2 }, Foo { value: 3 }],
        nothing: (),
    };

    edn_rs::to_string(&edn)
}

fn main() {
    println!("{}", serialize());
    // { :btreemap {:this-is-a-key [\"with\", \"many\", \"keys\"]}, :btreeset #{3, 4, 5}, :hashmap {:this-is-a-key [\"with\", \"many\", \"keys\"]}, :hashset #{3}, :tuples (3, true, \\d), :foo-vec [{ :value 2, }, { :value 3, }], :nothing nil, }
}

#[test]
fn test_serialize() {
    let edn_str = "{ :btreemap {:this-is-a-key [\"with\", \"many\", \"keys\"]}, :btreeset #{3, 4, 5}, :hashmap {:this-is-a-key [\"with\", \"many\", \"keys\"]}, :hashset #{3}, :tuples (3, true, \\d), :foo-vec [{ :value 2, }, { :value 3, }], :nothing nil, }";
    assert_eq!(serialize(), edn_str)
}
