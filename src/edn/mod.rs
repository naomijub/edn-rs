use std::collections::{HashMap};

pub mod utils;

/// `EdnType` is an Enum with possible values for an EDN type
#[derive(Debug, PartialEq, Clone)]
pub enum Edn {
    Vector(Vector),
    Set(Set),
    Map(Map),
    List(List),
    Int(i64),
    Key(String),
    Symbol(String),
    Str(String),
    Double(f64),
    Rational(String),
    Char(char),
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

impl Edn {
    fn to_string(self) -> String {
        match self {
            Edn::Nil => String::from("nil"),
            Edn::Bool(b) => format!("{}",b),
            Edn::Char(c) => format!("{}",c),
            Edn::Rational(r) => r, 
            Edn::Double(d) => format!("{}",d),
            Edn::Str(s) => s,
            Edn::Symbol(sy) => sy,
            Edn::Key(k) => k,
            Edn::Int(i) => format!("{}",i),
            Edn::Vector(v) => format!("{}",v),
            Edn::Set(set) => format!("{}",set),
            Edn::Map(m) => format!("{}",m),
            Edn::List(l) => format!("{}",l),
        }
    }
}

// pub fn from_edn(edn: String) -> Edn {
//     to_edn(edn)
// }

fn comma_to_dot(s: String) -> String {
    s.replace(",", ".")
}

pub fn to_edn(s: String) -> Edn {
    use regex::Regex;
    let keyword_regex = Regex::new(r":+[a-zA-Z0-9_]+[-[a-zA-Z0-9_]+]*").unwrap();
    let str_regex = Regex::new(r#"".+""#).unwrap();
    // let float_regex = Regex::new(r#"\d+,\d+"#).unwrap();
    let rational_regex = Regex::new(r#"\d+/\d+"#).unwrap();
    // let char_regex = Regex::new(r#"\\."#).unwrap();
    // let list_regex = Regex::new(r"\(.+\)").unwrap();
    // let vec_regex = Regex::new(r"\[.+\]").unwrap();
    // let set_regex = Regex::new(r"\#\{.+\}").unwrap();
    // let map_regex = Regex::new(r"\{.+\}").unwrap();

    match &s {
        element if element.is_empty() => Edn::Nil,
        // element if list_regex.is_match(element) => {
        //     let mut aux = s;
        //     Edn::List(List(to_seq(&mut aux)))
        // },
        // element if vec_regex.is_match(element) => {
        //     let mut aux = s;
        //     Edn::Vector(Vector(to_seq(&mut aux)))
        // },
        // element if set_regex.is_match(element) => {
        //     let mut aux = s;
        //     Edn::Set(Set(to_seq(&mut aux)))
        // },
        // element if map_regex.is_match(element) => {
        //     let mut aux = s;
        //     Edn::Map(Map(to_map(&mut aux)))
        // },
        element if element == "nil" || element == "null" => Edn::Nil,
        // element if element.parse::<bool>().is_ok() => Edn::Bool(element.parse::<bool>().unwrap()),
        element if element.parse::<i64>().is_ok() => Edn::Int(element.parse::<i64>().unwrap()),
        element if element.parse::<f64>().is_ok() => Edn::Double(element.parse::<f64>().unwrap()),
        // element if element == "[]" => Edn::Vector(Vector(Vec::new())),
        // element if element == "()" => Edn::List(List(Vec::new())),
        // element if element == "#{}" => Edn::Set(Set(Vec::new())),
        // element if element == "{}" => Edn::Map(Map(HashMap::new())),
        element if keyword_regex.is_match(element) => Edn::Key(element.to_string()),
        // element if char_regex.is_match(element) => Edn::Char(element.chars().last().unwrap()),
        element if rational_regex.is_match(element) => Edn::Rational(element.to_string()),
        element if str_regex.is_match(element) => Edn::Str(element.to_string()),
        _ => Edn::Symbol(s)
    }
}

fn to_seq(list: &mut String) -> Vec<Edn> {
    if list.contains("#{") {
        list.remove(0);
    }
    list.remove(0);
    list.remove(list.len() - 1);

    let tokens = tokenize_edn(list.to_owned());

    tokens.into_iter()
        .fold(Vec::new(),|mut acc, t|{
            acc.push(to_edn(t.to_string()));
            acc
        })
}

fn to_map(map: &mut String) -> HashMap<String, Edn> {
    map.remove(0);
    map.remove(map.len() - 1);

    let tokens = tokenize_edn(map.to_owned());

    let map_token = tokens.into_iter()
        .fold(Vec::new(),|mut acc, t|{
            acc.push(to_edn(t.to_string()));
            acc
        });
    let map_inputs = map_token.chunks_exact(2).collect::<Vec<&[Edn]>>();
    let mut hm = HashMap::new();
    for pair in map_inputs.iter() {
        hm.insert(pair[0].clone().to_string(), pair[1].clone());
    }
    hm
}

pub fn tokenize_edn(edn: String) -> Vec<String> {
    let edn0 = edn.replace("'(", "(");
    let edn1 = edn0.replace("(", " ( ");
    let edn2 = edn1.replace(")", " ) ");
    let edn3 = edn2.replace("]", " ] ");
    let edn4 = edn3.replace("[", " [ ");
    let edn5 = edn4.replace("#{", " #{ ");
    let edn6 = edn5.replace("}", " } ");
    let edn7 = edn6.replace("{", "{ ");
    let edn8 = edn7.trim();

    edn8.split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| String::from(*s))
        .collect::<Vec<String>>()
}