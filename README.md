# edn-rs
**Near Stable**

Crate to parse and emit EDN [![Build Status](https://travis-ci.org/naomijub/edn-rs.svg?branch=master)](https://travis-ci.org/naomijub/edn-rs)
* **This lib does not make effort to conform the EDN received to EDN Spec.** The lib that generated this EDN should be responsible for this. For more information on Edn Spec please visit: https://github.com/edn-format/edn.

Current example usage in:
* [crate `transistor`](https://github.com/naomijub/transistor);
* [`atm-crux`](https://github.com/naomijub/atm-crux);

## Usage

`Cargo.toml`
```toml
[dependencies]
edn-rs = "0.11.3"
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

**Parse an EDN String** with `from_str`:
```rust
use edn_rs::{
    set, map,
    Edn, Map, Vector, Set,
};
use std::str::FromStr;

fn main() -> Result<(), String> {
    let edn_str = "{:a \"2\" :b [true false] :c #{:A {:a :b} nil}}";
    // std::str::FromStr
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

    let edn_resp = edn_rs::from_str(edn_str)?;
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

**Serializes Rust Types into EDN with `ser_struct!`**
 ```rust
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use edn_rs::{
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

    println!("{}", edn_rs::to_string(edn));
    // { :btreemap {:this-is-a-key [\"with\", \"many\", \"keys\"]}, :btreeset #{3, 4, 5}, :hashmap {:this-is-a-key [\"with\", \"many\", \"keys\"]}, :hashset #{3}, :tuples (3, true, \\d), }
}
```

**Emits EDN** format from a Json:
* This function requires feature `json` to be activated. To enable this feature add to your `Cargo.toml`  dependencies the following line `edn-rs = { version = 0.11.3", features = ["json"] }`.

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

## Using `async/await` with Edn type

Edn supports `futures` by using the feature `async`. To enable this feature add to your `Cargo.toml`  dependencies the following line `edn-rs = { version = 0.11.3", features = ["async"] }` and you can use futures as in the following example.

```rust
use edn_rs::{edn, Double, Edn, Vector};
use futures::prelude::*;
use futures::Future;
use tokio::prelude::*;

async fn foo() -> impl Future<Output = Edn> + Send {
    edn!([1 1.5 "hello" :key])
}

#[tokio::main]
async fn main() {
    let edn = foo().await.await;

    println!("{}", edn.to_string());
    assert_eq!(edn, edn!([1 1.5 "hello" :key]));

    assert_eq!(edn[1].to_float(), Some(1.5f64));
}
```

## Edn-rs Current Features
- [x] Define `struct` to map EDN info `EdnNode`
- [x] Define EDN types, `EdnType`
 - [x] Edn Type into primitive: `Edn::Bool(true).into() -> true`. This was done by `to_float`, `to_bool`, `to_int`, `to_vec`.
 - [x] implement `futures::Future` trait to `Edn`
- [x] Parse EDN data [`from_str`](https://docs.rs/edn-rs/0.11.3/edn_rs/deserialize/fn.from_str.html):
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
- [ ] Simple data structures in one another [`edn!`](https://docs.rs/edn-rs/0.11.3/edn_rs/macro.edn.html):
    - [x] Vec in Vec `"[1 2 [:3 \"4\"]]"`
    - [ ] Set in _Vec_ `"[1 2 #{:3 \"4\"}]"`
    - [x] List in List `"(1 2 (:3 \"4\"))"`
    - [x] List in Set `"'#{1 2 (:3 \"4\")}"`
    - [x] Maps in general `"{:a 2 :b {:3 \"4\"}}"`, `"{:a 2 :b [:3 \"4\"]}"`
- [x] Multiple simple data structures in one another (Map and Set in a vector)
- [x] Multi deepen data structures (Map in a Set in a List in a  Vec in a Vec)
- [x] Navigate through Edn Data 
    - [x] Navigate through Sets. DOne by `set_iter`
- [x] Json to Edn
    - [x] Json String to EDN String
    - [x] macro to process Structs and Enums to EDN
- [x] trait Deserialize EDN to Struct
- [x] trait Serialize struct to EDN

## `edn-derive`
`edn-derive` is a proc-macro crate to (De)serialize Edn values, currently it is **pre-alpha** and it can be found at [`crates.io`](https://crates.io/crates/edn-derive) or at [`github`](https://github.com/otaviopace/edn-derive).

### Usage
Just add to your `Cargo.toml` the following:

```toml
[dependencies]
edn-derive = "<version>"
edn-rs = "0.11.3"
```

### Examples

**Serialize**
```rust
use edn_derive::Serialize;

#[derive(Serialize)]
pub struct Person {
    name: String,
    age: usize,
}

fn main() {
    let person = Person {
        name: "joana".to_string(),
        age: 290000,
    };
    assert_eq!(
        edn_rs::to_string(person),
        "{ :name \"joana\", :age 290000, }"
    );
}
```

### Current Features
- [x] `derive Serialize`
- [x] `edn_rs::to_string`
- [ ] `derive Deserialize`
- [ ] `let val: YourStruct = edn_rs::from_str(&str)`
