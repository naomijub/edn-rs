# edn-rs
[Experimental] Crate to parse and emit EDN
* **This lib does not make effort to conform the EDN received to EDN Spec.**

## Usage

`Cargo.toml`
```toml
[dependencies]
edn-rs = "0.2.1"
```

**Parse an EDN** into a `EdnNode`:
```rust
extern crate edn_rs;

use edn_rs::parse_edn;

fn main() {
    ...
    let edn = String::from("[1 2 [:3 \"4\"]]");
    let value = parse_edn(edn);
    ...
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
    - [x] Keywords
    - [x] Vector `"[1 :2 \"d\"]"`
    - [x] List `"(1 :2 \"d\")"`
    - [x] Set `"#{1 2 3}"`
    - [x] Map `"{:a 1 :b 2 }"`
- [x] Simple data structures in one another:
    - [x] Vec in Vec `"[1 2 [:3 \"4\"]]"`
    - [x] Set in Vec `"[1 2 #{:3 \"4\"}]"`
    - [x] List in List
    - [x] Set in List
    - [x] Set in Set (Sets will not be sorted and don't need a `dedup` due to the fact that they need to be compliant with EDN spec)
    - [x] Maps in general
- [ ] Multiple simple data structures in one another (Map and Set in a vector)
- [ ] Multi deepen data structures (Map in a Set in a List in a  Vec in a Vec)
- [ ] Json to Edn
    - [x] Json String to EDN String
    - [ ] macro to process Structs and Enums to EDN
- [ ] Edn to Json