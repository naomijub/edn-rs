#[derive(Debug, PartialEq)]
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
    Nil,
    Err,
}

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
