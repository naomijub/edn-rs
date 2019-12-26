pub mod utils;

/// `EdnType` is an Enum with possible values for an EDN type
#[derive(Debug, PartialEq, Clone)]
pub enum EdnType {
    Vector,
    VectorClose,
    Set,
    Map,
    MapSetClose,
    List,
    ListClose,
    Int,
    Key,
    Symbol,
    Str,
    Double,
    Rational,
    Nil,
    Err,
}

/// `EdnNode` is structured as follows: 
///  - `value` contains a string with the value of the corresponding EDN token. `"3"`, `"4/5"`, `"\"str\""`, `":a"`, `"["`...
///  - `edntype` contains the type that value represents
///  - `internal` is an `Option<Vec<_>>` contsining all `EdnNodes` inside this node. 
#[derive(Debug, PartialEq)]
pub struct EdnNode {
    pub value: String,
    pub edntype: EdnType,
    pub internal: Option<Vec<EdnNode>>,
}

impl EdnNode {
    pub fn nil() -> EdnNode {
        EdnNode {
            value: String::from("nil"),
            edntype: EdnType::Nil,
            internal: None,
        }
    }
}
