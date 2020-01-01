use std::collections::{HashMap};

pub mod utils;

/// `EdnType` is an Enum with possible values for an EDN type
/// Symbol and Char are not yet implemented
#[derive(Debug, PartialEq, Clone)]
pub enum Edn {
    Vector(Vector),
    Set(Set),
    Map(Map),
    List(List),
    Key(String),
    Symbol(String),
    Str(String),
    Int(isize),
    UInt(usize),
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

impl Eq for Map {}

impl core::fmt::Display for Edn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            Edn::Vector(v) => format!("{}", v),
            Edn::Set(s) => format!("{}", s),
            Edn::Map(m) => format!("{}", m),
            Edn::List(l) => format!("{}", l),
            Edn::Symbol(sy) => sy.to_string(),
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

impl Edn {
    /// `to_float` takes an `Edn` and returns an `Option<f64>` with its value. Most types return None
    /// ```rust
    /// use edn_rs::edn::{Edn, Vector};
    /// 
    /// let key = Edn::Key(String::from("1234"));
    /// let q = Edn::Rational(String::from("3/4"));
    /// let i = Edn::Int(12isize);
    /// 
    /// assert_eq!(Edn::Vector(Vector::empty()).to_float(), None);
    /// assert_eq!(key.to_float().unwrap(),1234f64);
    /// assert_eq!(q.to_float().unwrap(), 0.75f64);
    /// assert_eq!(i.to_float().unwrap(), 12f64);
    /// ```
    pub fn to_float(&self) -> Option<f64> {
        match self {
            Edn::Vector(_) => None,
            Edn::Set(_) => None,
            Edn::Map(_) => None,
            Edn::List(_) => None,
            Edn::Symbol(_) => None,
            Edn::Key(k) => match_parse(k.parse::<f64>()),
            Edn::Str(s) => match_parse(s.parse::<f64>()),
            Edn::Int(i) => match_parse(to_double(i)),
            Edn::UInt(u) => match_parse(to_double(u)),
            Edn::Double(d) => Some(d.to_owned()),
            Edn::Rational(r) => rational_to_double(r.to_owned()),
            Edn::Bool(_) => None,
            Edn::Nil => None,
        }
    }

    /// `to_int` takes an `Edn` and returns an `Option<isize>` with its value. Most types return None
    /// ```rust
    /// use edn_rs::edn::{Edn, Vector};
    /// 
    /// let key = Edn::Key(String::from("1234"));
    /// let q = Edn::Rational(String::from("3/4"));
    /// let f = Edn::Double(12.3f64);
    /// 
    /// assert_eq!(Edn::Vector(Vector::empty()).to_float(), None);
    /// assert_eq!(key.to_int().unwrap(),1234isize);
    /// assert_eq!(q.to_int().unwrap(), 1isize);
    /// assert_eq!(f.to_int().unwrap(), 12isize);
    /// ```
    pub fn to_int(&self) -> Option<isize> {
        match self {
            Edn::Vector(_) => None,
            Edn::Set(_) => None,
            Edn::Map(_) => None,
            Edn::List(_) => None,
            Edn::Symbol(_) => None,
            Edn::Key(k) => match_parse(k.parse::<isize>()),
            Edn::Str(s) => match_parse(s.parse::<isize>()),
            Edn::Int(i) => Some(i.to_owned()),
            Edn::UInt(_) => None,
            Edn::Double(d) => Some(d.to_owned().round() as isize),
            Edn::Rational(r) => Some(rational_to_double(r.to_owned()).unwrap_or(0f64).round() as isize),
            Edn::Bool(_) => None,
            Edn::Nil => None,
        }
    }
}

fn to_double<T>(i: T) -> Result<f64,std::num::ParseFloatError> 
    where T : std::fmt::Debug {
    format!("{:?}", i).parse::<f64>()
}

fn rational_to_double(r: String) -> Option<f64> {
    match r {
        s if s.split("/").collect::<Vec<&str>>().len() == 2 => {
            let vals = s.split("/").map(|i| i.to_string()).map(|v| v.parse::<f64>().unwrap()).collect::<Vec<f64>>();
            Some(vals[0] / vals[1])
        },
        _ => None
    }
}

fn match_parse<T,E>(f: Result<T,E>) -> Option<T> {
    match f {
        Ok(val) => Some(val),
        Err(_) => None
    }
}

#[test]
fn parses_rationals() {
    assert_eq!(rational_to_double(String::from("3/4")).unwrap(), 0.75f64);
    assert_eq!(rational_to_double(String::from("25/5")).unwrap(), 5f64);
    assert_eq!(rational_to_double(String::from("15/4")).unwrap(), 3.75f64);
    assert_eq!(rational_to_double(String::from("3 4")), None);
    assert_eq!(rational_to_double(String::from("3/4/5")), None);
}