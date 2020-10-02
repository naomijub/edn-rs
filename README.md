# edn-rs
**Near Stable** no breaking changes expected.

Crate to parse and emit EDN [![Build Status](https://travis-ci.org/naomijub/edn-rs.svg?branch=master)](https://travis-ci.org/naomijub/edn-rs)
* **This lib does not make effort to conform the EDN received to EDN Spec.** The lib that generated this EDN should be responsible for this. For more information on Edn Spec please visit: https://github.com/edn-format/edn.

Current example usage in:
* [crate `transistor`](https://github.com/naomijub/transistor);
* [`atm-crux`](https://github.com/naomijub/atm-crux);
* [Rust/Clojure FFI. Deserialize Clojure Edn into Rust Struct](https://github.com/naomijub/JVM-rust-ffi/tree/master/clj-rs);

## Usage

`Cargo.toml`
```toml
[dependencies]
edn-rs = "0.16.6"
```

## Simple time-only benchmarks of `edn-rs` agains Clojure Edn
* Link to benchmarks implementation [here](https://github.com/naomijub/edn-duration-benchmark/blob/master/README.md)

| Method\Lang 	| Rust --release 	| Rust --debug 	| Clojure 	|
|-	|-	|-	|-	|
| parse string 	| 77.57µs 	| 266.479µs 	| 4.712235 milis 	|
| get-in/navigate (3 blocks)	| 4.224µs	| 22.861µs 	| 26.333 µs 	|
| Deserialize to struct 	| 110.358µs 	| 357.054µs 	| 4.712235 milis 	|
| parse with criterium | 11.348µs | - | 23.230µs|

## Quick reference

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
                Edn::Key(":f".to_string()),
                Edn::Nil,
                Edn::Rational("3/4".to_string())
            ]
        )
    );

    println!("{:?}", edn);
    assert_eq!(edn, expected);
}
```

**Parse an EDN String** with `Edn::from_str`:
```rust
use edn_rs::{
    set, map,
    Edn, Map, Vector, Set,
};
use std::str::FromStr;

fn main() -> Result<(), String> {
    let edn_str = "{:a \"2\" :b [true false] :c #{:A {:a :b} nil}}";
    // std::str::FromStr
    let edn: Edn = Edn::from_str(edn_str);

    assert_eq!(
        edn,
        Edn::Map(Map::new(
            map!{
                ":a".to_string() => Edn::Str("2".to_string()),
                ":b".to_string() => Edn::Vector(Vector::new(vec![Edn::Bool(true), Edn::Bool(false)])),
                ":c".to_string() => Edn::Set(Set::new(
                    set!{
                        Edn::Map(Map::new(map!{":a".to_string() => Edn::Key(":b".to_string())})),
                        Edn::Key(":A".to_string()),
                        Edn::Nil}))}
        ))
    );

    assert_eq!(edn[":b"][0], Edn::Bool(true));

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
    assert_eq!(edn[3]["false"], Edn::Key(":f".to_string()));
}
```

**Serializes Rust Types into EDN with `edn-derive::Serialize`**
 ```rust
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use edn_rs::{
     map, set, hmap, hset
};
use edn_derive::Serialize;

#[derive(Debug, Clone, Serialize)]
struct Edn {
    btreemap: BTreeMap<String, Vec<String>>,
    btreeset: BTreeSet<i64>,
    hashmap: HashMap<String, Vec<String>>,
    hashset: HashSet<i64>,
    tuples: (i32, bool, char),
    nothing: (),
}

fn main() {
    
    let edn = Edn {
        btreemap: map!{"this is a key".to_string() => vec!["with".to_string(), "many".to_string(), "keys".to_string()]},
        btreeset: set!{3i64, 4i64, 5i64},
        hashmap: hmap!{"this is a key".to_string() => vec!["with".to_string(), "many".to_string(), "keys".to_string()]},
        hashset: hset!{3i64},
        tuples: (3i32, true, 'd'),
        nothing: (),
    };

    println!("{}", edn_rs::to_string(edn));
    // { :btreemap {:this-is-a-key [\"with\", \"many\", \"keys\"]}, :btreeset #{3, 4, 5}, :hashmap {:this-is-a-key [\"with\", \"many\", \"keys\"]}, :hashset #{3}, :tuples (3, true, \\d), :nothing nil, }
}
```

**Deserializes Strings into Rust Types**:

> For now you have to implement the conversion yourself with the `Deserialize` trait. Soon you'll be able to have that implemented for you via `edn-derive` crate.
 ```rust
use edn_rs::{Deserialize, Edn, EdnError};

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

impl Deserialize for Person {
    fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
        Ok(Self {
            name: edn_rs::from_edn(&edn[":name"])?,
            age: edn_rs::from_edn(&edn[":age"])?,
        })
    }
}

fn main() -> Result<(), EdnError> {
    let edn_str = "{:name \"rose\" :age 66}";
    let person: Person = edn_rs::from_str(edn_str)?;

    assert_eq!(
        person,
        Person {
            name: "rose".to_string(),
            age: 66,
        }
    );

    println!("{:?}", person);
    // Person { name: "rose", age: 66 }

    let bad_edn_str = "{:name \"rose\" :age \"some text\"}";
    let person: Result<Person, EdnError> = edn_rs::from_str(bad_edn_str);

    assert_eq!(
        person,
        Err(EdnError::Deserialize(
            "couldn't convert `some text` into `uint`".to_string()
        ))
    );

    Ok(())
}
```

**Deserializes Edn types into Rust Types**:

> For now you have to implement the conversion yourself with the `Deserialize` trait. Soon you'll be able to have that implemented for you via `edn-derive` crate.
 ```rust
use edn_rs::{map, Deserialize, Edn, EdnError, Map};

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

impl Deserialize for Person {
    fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
        Ok(Self {
            name: edn_rs::from_edn(&edn[":name"])?,
            age: edn_rs::from_edn(&edn[":age"])?,
        })
    }
}

fn main() -> Result<(), EdnError> {
    let edn = Edn::Map(Map::new(map! {
        ":name".to_string() => Edn::Str("rose".to_string()),
        ":age".to_string() => Edn::UInt(66)
    }));
    let person: Person = edn_rs::from_edn(&edn)?;

    println!("{:?}", person);
    // Person { name: "rose", age: 66 }

    assert_eq!(
        person,
        Person {
            name: "rose".to_string(),
            age: 66,
        }
    );

    let bad_edn = Edn::Map(Map::new(map! {
        ":name".to_string() => Edn::Str("rose".to_string()),
        ":age".to_string() => Edn::Str("some text".to_string())
    }));
    let person: Result<Person, EdnError> = edn_rs::from_edn(&bad_edn);

    assert_eq!(
        person,
        Err(EdnError::Deserialize(
            "couldn't convert `\"some text\"` into `uint`".to_string()
        ))
    );

    Ok(())
}
```

**Emits EDN** format from a Json:
* This function requires feature `json` to be activated. To enable this feature add to your `Cargo.toml`  dependencies the following line `edn-rs = { version = 0.16.6", features = ["json"] }`.

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

**to_string/to_debug**

`to_debug` emits a Debug version of `Edn` type.
```rust
use edn_rs::edn::{Edn, Vector};

let edn = Edn::Vector(Vector::new(vec![Edn::Int(5), Edn::Int(6), Edn::Int(7)]));
let expected = "Vector(Vector([Int(5), Int(6), Int(7)]))";

assert_eq!(edn.to_debug(), expected);
```

`to_string` emits a valid edn.
```rust
use edn_rs::edn::{Edn, Vector};

let edn = Edn::Vector(Vector::new(vec![Edn::Int(5), Edn::Int(6), Edn::Int(7)]));
let expected = "[5, 6, 7, ]";

assert_eq!(edn.to_string(), expected);
```

Larger `to_string` example:
```rust
fn complex_ok() -> Result<(), EdnError> {
    use std::str::FromStr;
    let edn_str = "{ :list [{:name \"rose\" :age 66 :cool true}, {:name \"josh\" :age 33 :cool false}, {:name \"eva\" :age 296 :cool true}] }";

    let edn = Edn::from_str(edn_str)?;
    println!("{:?}", edn.to_string());
//    "{:list: [{:age 66, :cool true, :name \"rose\", }, {:age 33, :cool false, :name \"josh\", }, {:age 296, :cool true, :name \"eva\", }, ], }"

    Ok(())
}
```

## Using `async/await` with Edn type

Edn supports `futures` by using the feature `async`. To enable this feature add to your `Cargo.toml`  dependencies the following line `edn-rs = { version = 0.16.6", features = ["async"] }` and you can use futures as in the following example.

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

The objective of `foo` is to show that `Edn` can be wrapped with a `Future`. If you want to return an `Edn` from an `async` function just use:

```rust
async fn foo() -> Edn {
    edn!([1 1.5 "hello" :key])
}
```

## Edn-rs Current Features
- [x] Define `struct` to map EDN info `EdnNode`
- [x] Define EDN types, `EdnType`
    - [x] Edn Type into primitive: `Edn::Bool(true).into() -> true`. This was done by `to_float`, `to_bool`, `to_int`, `to_vec`.
    - [x] implement `futures::Future` trait to `Edn`
    - [x] `to_string()` for `Edn`.
    - [x] `to_debug()` for `Edn`.
- [x] Parse EDN data [`from_str`](https://docs.rs/edn-rs/0.16.6/edn_rs/deserialize/fn.from_str.html):
    - [x] nil `""`
    - [x] String `"\"string\""`
    - [x] Numbers `"324352"`, `"3442.234"`, `"3/4"`
    - [x] Keywords `:a`
    - [x] Symbol `sym-bol-s` with a maximum of 200 chars
    - [x] Vector `"[1 :2 \"d\"]"`
    - [x] List `"(1 :2 \"d\")"`
    - [x] Set `"#{1 2 3}"`
    - [x] Map `"{:a 1 :b 2 }"`
    - [x] Inst `#inst \"yyyy-mm-ddTHH:MM:ss\"`
    - [x] Nested structures `"{:a \"2\" :b [true false] :c #{:A {:a :b} nil}}"`
- [ ] Simple data structures in one another [`edn!`](https://docs.rs/edn-rs/0.16.6/edn_rs/macro.edn.html):
    - [x] Vec in Vec `"[1 2 [:3 \"4\"]]"`
    - [ ] Set in _Vec_ `"[1 2 #{:3 \"4\"}]"`
    - [x] List in List `"(1 2 (:3 \"4\"))"`
    - [x] List in Set `"'#{1 2 (:3 \"4\")}"`
    - [x] Maps in general `"{:a 2 :b {:3 \"4\"}}"`, `"{:a 2 :b [:3 \"4\"]}"`
    - [ ] Namespaced Maps `":abc{0 5 1 "hello"}`, unfortunately now this is misinterpreted as a keyword
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
edn-rs = "0.16.6"
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

**Deserialization**
```rust
use edn_derive::Deserialize;
use edn_rs::EdnError;

// The `Debug` and `PartialEq` are only necessary because of `assert_eq`, you don't need them
#[derive(Deserialize, Debug, PartialEq)]
pub struct Person {
    name: String,
    age: usize,
}

fn main() -> Result<(), EdnError> {
    let edn_person = "{ :name \"joana\", :age 290000, }";

    let person: Person = edn_rs::from_str(edn_person)?;

    assert_eq!(
        person,
        Person {
            name: "joana".to_string(),
            age: 290000,
        }
    );

    Ok(())
}
```

### Current Features
- [x] `derive Serialize`
- [x] `edn_rs::to_string`
- [x] `derive Deserialize`
- [x] `let val: YourStruct = edn_rs::from_str(&str)`
