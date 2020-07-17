# edn-rs
**Near Stable**

Crate to parse and emit EDN [![Build Status](https://travis-ci.org/naomijub/edn-rs.svg?branch=master)](https://travis-ci.org/naomijub/edn-rs)
* **This lib does not make effort to conform the EDN received to EDN Spec.** The lib that generated this EDN should be responsible for this. For more information on Edn Spec please visit: https://github.com/edn-format/edn.
* Current example usage in crate https://crates.io/crates/transistor 

## Usage

`Cargo.toml`
```toml
[dependencies]
edn-rs = "0.9.2"
```

**Parse an EDN token** into a `Edn` with `edn!` macro:
```rust
use edn_rs::{
    edn, Edn, List
};

fn main() {
    let edn = edn!((sym 1.2 3 false :f nil 3/4));
    let expected = Edn::List(
        List::new(
            vec![
                Edn::Symbol("sym".to_string()),
                Edn::Double(1.2.into()),
                Edn::Int(3),
                Edn::Bool(false),
                Edn::Key("f".to_string()),
                Edn::Nil,
                Edn::Rational("3/4".to_string())
            ]
        )
    );

    println!("{:?}", edn);
    assert_eq!(edn, expected);
}
```

**Parse an EDN String** with `parse_edn`:
```rust
use edn_rs::{
    set, map,
    Edn, Map, Vector, Set,
    parse_edn
};
use std::str::FromStr;

fn main() -> Result<(), String> {
    let edn_str = "{:a \"2\" :b [true false] :c #{:A {:a :b} nil}}";
    let edn = Edn::from_str(edn_str);

    assert_eq!(
        edn,
        Ok(Edn::Map(Map::new(
            map!{
                ":a".to_string() => Edn::Str("2".to_string()),
                ":b".to_string() => Edn::Vector(Vector::new(vec![Edn::Bool(true), Edn::Bool(false)])),
                ":c".to_string() => Edn::Set(Set::new(
                    set!{
                        Edn::Map(Map::new(map!{":a".to_string() => Edn::Key(":b".to_string())})),
                        Edn::Key(":A".to_string()),
                        Edn::Nil}))}
        )))
    );

    // OR 

    let edn_resp = parse_edn(edn_str)?;
    assert_eq!(edn_resp[":b"][0], Edn::Bool(true));
    Ok(())
}
```

To navigate through `Edn` data you can just use `get` and `get_mut`:

```rust
use edn_rs::{
    edn,
    Edn, List, Map
};

fn main() {
    let edn = edn!((sym 1.2 3 {false :f nil 3/4}));

    println!("{:?}", edn);
    assert_eq!(edn[1], edn!(1.2));
    assert_eq!(edn[1], Edn::Double(1.2f64.into()));
    assert_eq!(edn[3]["false"], edn!(:f));
    assert_eq!(edn[3]["false"], Edn::Key("f".to_string()));
}
```

**Serializes Rust Types into EDN**
 ```rust
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use edn_rs::{
    Serialize,
    ser_struct, map, set, hmap, hset
};

fn main() {
    ser_struct!{
        #[derive(Debug, Clone)]
        struct Edn {
            btreemap: BTreeMap<String, Vec<String>>,
            btreeset: BTreeSet<i64>,
            hashmap: HashMap<String, Vec<String>>,
            hashset: HashSet<i64>,
            tuples: (i32, bool, char),
        }
    };
    let edn = Edn {
        btreemap: map!{"this is a key".to_string() => vec!["with".to_string(), "many".to_string(), "keys".to_string()]},
        btreeset: set!{3i64, 4i64, 5i64},
        hashmap: hmap!{"this is a key".to_string() => vec!["with".to_string(), "many".to_string(), "keys".to_string()]},
        hashset: hset!{3i64},
        tuples: (3i32, true, 'd')
    };

    println!("{}",edn.serialize());
    // { :btreemap {:this-is-a-key [\"with\", \"many\", \"keys\"]}, :btreeset #{3, 4, 5}, :hashmap {:this-is-a-key [\"with\", \"many\", \"keys\"]}, :hashset #{3}, :tuples (3, true, \\d), }
}
```

**Emits EDN** format from a Json file
 ```rust
use edn_rs::json_to_edn;

fn main() {
    let json = String::from(r#"{"hello": "world"}"#);
    let edn = String::from(r#"{:hello "world"}"#);

    println!("{:?}", json_to_edn(json.clone()));
    assert_eq!(edn, json_to_edn(json));

    let complex_json = String::from(r#"{
            "people": 
            [
                {
                    "name": "otavio",
                    "age": 22
                },
                {
                    "name": "Julia",
                    "age": 32.0
                }
            ],
            "country or origin": "Brazil",
            "queerentener": true,
            "brain": null
        }"#);

    println!("{:?}", json_to_edn(complex_json.clone()).replace("  ", "").replace("\n", " "));
    // "{ :people  [ { :name \"otavio\", :age 22 }, { :name \"Julia\", :age 32.0 } ], :country-or-origin \"Brazil\", :queerentener true, :brain nil }"
}
 ```

## Current Features
- [x] Define `struct` to map EDN info `EdnNode`
- [x] Define EDN types, `EdnType`
 - [ ] Edn Type into primitive: `Edn::Bool(true).into() -> true`
- [x] Parse EDN data [`parse_edn`](https://docs.rs/edn-rs/0.9.2/edn_rs/deserialize/fn.parse_edn.html):
    - [x] nil `""`
    - [x] String `"\"string\""`
    - [x] Numbers `"324352"`, `"3442.234"`, `"3/4"`
    - [x] Keywords `:a`
    - [x] Symbol `sym-bol-s`
    - [x] Vector `"[1 :2 \"d\"]"`
    - [x] List `"(1 :2 \"d\")"`
    - [x] Set `"#{1 2 3}"`
    - [x] Map `"{:a 1 :b 2 }"`
    - [x] Nested structures `"{:a \"2\" :b [true false] :c #{:A {:a :b} nil}}"`
- [ ] Simple data structures in one another [`edn!`](https://docs.rs/edn-rs/0.9.2/edn_rs/macro.edn.html):
    - [x] Vec in Vec `"[1 2 [:3 \"4\"]]"`
    - [ ] Set in _Vec_ `"[1 2 #{:3 \"4\"}]"`
    - [x] List in List `"(1 2 (:3 \"4\"))"`
    - [x] List in Set `"'#{1 2 (:3 \"4\")}"`
    - [x] Maps in general `"{:a 2 :b {:3 \"4\"}}"`, `"{:a 2 :b [:3 \"4\"]}"`
- [x] Multiple simple data structures in one another (Map and Set in a vector)
- [x] Multi deepen data structures (Map in a Set in a List in a  Vec in a Vec)
- [x] Navigate through Edn Data 
    - [ ] Navigate through Sets
- [x] Json to Edn
    - [x] Json String to EDN String
    - [x] macro to process Structs and Enums to EDN
- [ ] trait Deserialize EDN to Struct
- [x] trait Serialize struct to EDN

## Could be done in another project `edn-derive`
- [ ] `derive Serialize`
- [ ] `derive Deserialize`
