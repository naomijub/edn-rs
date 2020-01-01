use std::collections::{HashMap};

pub mod utils;

/// `EdnType` is an Enum with possible values for an EDN type
#[derive(Debug, PartialEq, Clone)]
pub enum Edn {
    Vector(Vector),
    Set(Set),
    Map(Map),
    List(List),
    Key(String),
    // Symbol(String),
    Str(String),
    Int(i128),
    UInt(u128),
    Double(f64),
    Rational(String),
    // Char(char),
    Bool(bool),
    Nil,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Vector(Vec<Edn>);
impl Vector {
    pub fn new(v: Vec<Edn>) -> Vector {
        Vector(v)
    }

    pub fn empty() -> Vector {
        Vector(Vec::new())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct List(Vec<Edn>);
impl List {
    pub fn new(v: Vec<Edn>) -> List {
        List(v)
    }

    pub fn empty() -> List {
        List(Vec::new())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Set(Vec<Edn>);
impl Set {
    pub fn new(v: Vec<Edn>) -> Set {
        Set(v)
    }

    pub fn empty() -> Set {
        Set(Vec::new())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Map(HashMap<String,Edn>);
impl Map {
    pub fn new(m: HashMap<String, Edn>) -> Map {
        Map(m)
    }

    pub fn empty() -> Map {
        Map(HashMap::new())
    }
}

impl core::fmt::Display for Vector {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{}]", self.0.iter().map(|i| format!("{:?}, ", i)).fold(String::new(),|mut acc, i| {acc.push_str(&i); acc}))
    }
}

impl core::fmt::Display for List {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({})", self.0.iter().map(|i| format!("{:?}, ", i)).fold(String::new(),|mut acc, i| {acc.push_str(&i); acc}))
    }
}

impl core::fmt::Display for Set {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{{{}}}", self.0.iter().map(|i| format!("{:?}, ", i)).fold(String::new(),|mut acc, i| {acc.push_str(&i); acc}))
    }
}

impl core::fmt::Display for Map {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{{}}}", self.0.iter().map(|(k,v)| format!("{}: {:?}, ", k, v)).fold(String::new(),|mut acc, i| {acc.push_str(&i); acc}))
    }
}

impl core::fmt::Display for Edn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            Edn::Vector(v) => format!("{}", v),
            Edn::Set(s) => format!("{}", s),
            Edn::Map(m) => format!("{}", m),
            Edn::List(l) => format!("{}", l),
            Edn::Key(k) => k.to_string(),
            Edn::Str(s) => s.to_string(),
            Edn::Int(i) => format!("{}", i),
            Edn::UInt(u) => format!("{}", u),
            Edn::Double(d) => format!("{}", d),
            Edn::Rational(r) => r.to_string(),
            Edn::Bool(b) => format!("{}", b),
            Edn::Nil => String::from("nil"),
        };
        write!(f, "{}", text)
    }
}

// pub fn tokenize_edn(edn: String) -> Vec<String> {
//     let edn0 = edn.replace("'(", "(");
//     let edn1 = edn0.replace("(", " ( ");
//     let edn2 = edn1.replace(")", " ) ");
//     let edn3 = edn2.replace("]", " ] ");
//     let edn4 = edn3.replace("[", " [ ");
//     let edn5 = edn4.replace("#{", " #{ ");
//     let edn6 = edn5.replace("}", " } ");
//     let edn7 = edn6.replace("{", "{ ");
//     let edn8 = edn7.trim();

//     edn8.split(' ')
//         .collect::<Vec<&str>>()
//         .iter()
//         .filter(|s| !s.is_empty())
//         .map(|s| String::from(*s))
//         .collect::<Vec<String>>()
// }