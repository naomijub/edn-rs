# edn-rs
[Experimental] Crate to parse and emit EDN

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
    - [x] Unbalanced Map returns `EdnType::Err`, `"{:a 1 :b}"`
- [x] Simple data structures in one another:
    - [x] Vec in Vec `"[1 2 [:3 \"4\"]]"`
    - [x] Set in Vec `"[1 2 #{:3 \"4\"}]"`
    - [ ] List in List
    - [ ] Set in List
    - [ ] Set in Set (Needs to sort set withoug the inner values)
    - [ ] List in Set (Needs to sort set withoug the inner values)
    - [ ] Maps in general
- [ ] Multiple simple data structures in one another (Map and Set in a vector)
- [ ] Multi deepen data structures (Map in a Set in a List in a  Vec in a Vec)
- [ ] Json to Edn
    - [ ] macro to process Structs and Enums to EDN
- [ ] Edn to Json