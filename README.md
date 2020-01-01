# edn-rs
[Experimental] Crate to parse and emit EDN
* **This lib does not make effort to conform the EDN received to EDN Spec.** The lib that generated this EDN should be responsible for this.

## Usage

`Cargo.toml`
```toml
[dependencies]
edn-rs = "0.4.0"
```

**Parse an EDN** into a `EdnNode`:
```rust
#[macro_use]
extern crate edn_rs;

fn main() {
    let edn = edn!((1 1.2 3 false :f nil 3/4));
    let expected = Edn::List(
            List::new(
                vec![
                    Edn::Int(1),
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
- [ ] Json to Edn
    - [x] Json String to EDN String
    - [ ] macro to process Structs and Enums to EDN
- [ ] Edn to Json