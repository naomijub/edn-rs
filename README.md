# edn-rs
[MAYBE_STABLE] Crate to parse and emit EDN
* **This lib does not make effort to conform the EDN received to EDN Spec.** The lib that generated this EDN should be responsible for this.

## Usage

`Cargo.toml`
```toml
[dependencies]
edn-rs = "0.5.2"
```

**Parse an EDN** into a `Edn` with `edn!` macro:
```rust
#![recursion_limit="512"]
#[macro_use]
extern crate edn_rs;

fn main() {
    let edn = edn!((sym 1.2 3 false :f nil 3/4));
    let expected = Edn::List(
            List::new(
                vec![
                    Edn::Symbol("sym".to_string()),
                    Edn::Double(1.2),
                    Edn::Int(3),
                    Edn::Bool(false),
                    Edn::Key("f".to_string()),
                    Edn::Nil,
                    Edn::Rational("3/4".to_string())
                ]
            )
        );

        assert_eq!(edn, expected);
}
```

To navigate through `Edn` data you can just use `get` and `get_mut`:

```rust
let edn = edn!([ 1 1.2 3 {false :f nil 3/4}]);

assert_eq!(edn[1], edn!(1.2));
assert_eq!(edn[1], Edn::Double(1.2f64));
assert_eq!(edn[3]["false"], edn!(:f));
assert_eq!(edn[3]["false"], Edn::Key("f".to_string()));
```

**Serializes Rust Types into EDN**
 ```rust
 #![recursion_limit="512"]
 #[macro_use] extern crate edn_rs;
 
 use std::collections::{HashMap, HashSet};
 use crate::edn_rs::serialize::Serialize;
 
 fn main() {
     ser_struct!{
         #[derive(Debug)]
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
     println!("{}",edn.serialize());
     // { :map {:this-is-a-key ["with", "many", "keys"]}, :set #{3, 4, 5}, :tuples (3, true, \d), }
 }
```

**Emits EDN** format from a Json file
 ```rust
 use edn_rs::emit_edn;

 fn main() {
     let json = String::from("{\"hello\": \"world\"}");
     let edn = String::from("{:hello \"world\"}");

     assert_eq!(edn, emit_edn(json));
 }
 ```

## Current Features
- [x] Define `struct` to map EDN info `EdnNode`
- [x] Define EDN types, `EdnType`
- [x] Parse simples EDN data:
    - [x] nil `""`
    - [x] String `"\"string\""`
    - [x] Numbers `"324352"`, `"3442.234"`, `"3/4"`
    - [x] Keywords `:a`
    - [x] Symbol `sym-bol-s`
    - [x] Vector `"[1 :2 \"d\"]"`
    - [x] List `"(1 :2 \"d\")"`
    - [x] Set `"#{1 2 3}"` For now the usage of Set is defined as a `Vec<Edn>`, this is due to the fact that the lib should not be necessarily responsible for assuring the Set's unicity. A solution could be changing the implementation to `HashSet`.
    - [x] Map `"{:a 1 :b 2 }"`
- [ ] Simple data structures in one another:
    - [x] Vec in Vec `"[1 2 [:3 \"4\"]]"`
    - [ ] Set in _Vec_ `"[1 2 #{:3 \"4\"}]"`
    - [x] List in List `"(1 2 (:3 \"4\"))"`
    - [x] List in Set `"'#{1 2 (:3 \"4\")}"`
    - [x] Maps in general `"{:a 2 :b {:3 \"4\"}}"`, `"{:a 2 :b [:3 \"4\"]}"`
- [x] Multiple simple data structures in one another (Map and Set in a vector)
- [x] Multi deepen data structures (Map in a Set in a List in a  Vec in a Vec)
- [x] Navigate through Edn Data 
- [x] Json to Edn
    - [x] Json String to EDN String
    - [x] macro to process Structs and Enums to EDN
- [ ] Edn to Json