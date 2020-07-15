use ordered_float::OrderedFloat;
use std::cmp::{Ord, PartialOrd};
use std::collections::{BTreeMap, BTreeSet};
use utils::index::Index;
#[doc(hidden)]
pub mod utils;

/// `EdnType` is an Enum with possible values for an EDN type
/// Symbol and Char are not yet implemented
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
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
    Double(Double),
    Rational(String),
    Char(char),
    Bool(bool),
    Nil,
    Empty,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Vector(Vec<Edn>);
impl Vector {
    pub fn new(v: Vec<Edn>) -> Vector {
        Vector(v)
    }

    pub fn empty() -> Vector {
        Vector(Vec::new())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct List(Vec<Edn>);
impl List {
    pub fn new(v: Vec<Edn>) -> List {
        List(v)
    }

    pub fn empty() -> List {
        List(Vec::new())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Set(BTreeSet<Edn>);
impl Set {
    pub fn new(v: BTreeSet<Edn>) -> Set {
        Set(v)
    }

    pub fn empty() -> Set {
        Set(BTreeSet::new())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Map(BTreeMap<String, Edn>);
impl Map {
    pub fn new(m: BTreeMap<String, Edn>) -> Map {
        Map(m)
    }

    pub fn empty() -> Map {
        Map(BTreeMap::new())
    }
}

pub type Double = OrderedFloat<f64>;

impl core::fmt::Display for Vector {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|i| format!("{:?}, ", i))
                .fold(String::new(), |mut acc, i| {
                    acc.push_str(&i);
                    acc
                })
        )
    }
}

impl core::fmt::Display for List {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "({})",
            self.0
                .iter()
                .map(|i| format!("{:?}, ", i))
                .fold(String::new(), |mut acc, i| {
                    acc.push_str(&i);
                    acc
                })
        )
    }
}

impl core::fmt::Display for Set {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "#{{{}}}",
            self.0
                .iter()
                .map(|i| format!("{:?}, ", i))
                .fold(String::new(), |mut acc, i| {
                    acc.push_str(&i);
                    acc
                })
        )
    }
}

impl core::fmt::Display for Map {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|(k, v)| format!("{}: {:?}, ", k, v))
                .fold(String::new(), |mut acc, i| {
                    acc.push_str(&i);
                    acc
                })
        )
    }
}

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
            Edn::Char(c) => format!("{}", c),
            Edn::Nil => String::from("nil"),
            Edn::Empty => String::from(""),
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
            Edn::Key(k) => k.parse::<f64>().ok(),
            Edn::Str(s) => s.parse::<f64>().ok(),
            Edn::Int(i) => to_double(i).ok(),
            Edn::UInt(u) => to_double(u).ok(),
            Edn::Double(d) => Some(d.into_inner()),
            Edn::Rational(r) => rational_to_double(&r),
            Edn::Bool(_) => None,
            Edn::Char(_) => None,
            Edn::Nil => None,
            Edn::Empty => None,
        }
    }

    /// `to_int` takes an `Edn` and returns an `Option<isize>` with its value. Most types return None
    /// ```rust
    /// use edn_rs::edn::{Edn, Vector};
    ///
    /// let key = Edn::Key(String::from("1234"));
    /// let q = Edn::Rational(String::from("3/4"));
    /// let f = Edn::Double(12.3f64.into());
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
            Edn::Key(k) => k.parse::<isize>().ok(),
            Edn::Str(s) => s.parse::<isize>().ok(),
            Edn::Int(i) => Some(i.to_owned()),
            Edn::UInt(_) => None,
            Edn::Double(d) => Some(d.to_owned().round() as isize),
            Edn::Rational(r) => Some(rational_to_double(&r).unwrap_or(0f64).round() as isize),
            Edn::Bool(_) => None,
            Edn::Char(_) => None,
            Edn::Nil => None,
            Edn::Empty => None,
        }
    }

    /// Index into a EDN vector, list, set or map. A string index can be used to access a
    /// value in a map, and a usize index can be used to access an element of a
    /// seqs.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is a seq or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the seq.
    ///
    /// ```rust
    /// #[macro_use]
    /// extern crate edn_rs;
    /// use edn_rs::edn::{Edn, Map, Vector};
    ///
    /// fn main() {
    ///     let edn = edn!([ 1 1.2 3 {false :f nil 3/4}]);
    ///
    ///     assert_eq!(edn[1], edn!(1.2));
    ///     assert_eq!(edn.get(1).unwrap(), &edn!(1.2));
    ///     assert_eq!(edn[3]["false"], edn!(:f));
    ///     assert_eq!(edn[3].get("false").unwrap(), &Edn::Key("f".to_string()));
    /// }
    /// ```
    pub fn get<I: Index>(&self, index: I) -> Option<&Edn> {
        index.index_into(self)
    }

    /// Mutably index into a EDN vector, set, list or map. A string index can be used to
    /// access a value in a map, and a usize index can be used to access an
    /// element of a seq.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is a seq or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the seq.
    ///
    /// ```rust
    /// #[macro_use]
    /// extern crate edn_rs;
    /// use edn_rs::edn::{Edn, Map, Vector};
    ///
    /// fn main() {
    ///     let mut edn = edn!([ 1 1.2 3 {false :f nil 3/4}]);
    ///
    ///     assert_eq!(edn[1], edn!(1.2));
    ///     assert_eq!(edn.get_mut(1).unwrap(), &edn!(1.2));
    ///     assert_eq!(edn[3]["false"], edn!(:f));
    ///     assert_eq!(edn[3].get_mut("false").unwrap(), &Edn::Key("f".to_string()));
    /// }
    /// ```
    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Edn> {
        index.index_into_mut(self)
    }

    pub(crate) fn parse_word(word: String) -> Edn {
        match word {
            w if w.starts_with(":") => Edn::Key(w),
            w if w.starts_with("\\") && w.len() == 2 => Edn::Char(w.chars().last().unwrap()),
            w if w.starts_with("\"") && w.ends_with("\"") => Edn::Str(w.replace("\"", "")),
            w if w.parse::<bool>().is_ok() => Edn::Bool(w.parse::<bool>().unwrap()),
            w if w == "nil" || w == "Nil" => Edn::Nil,
            w if w.contains("/") && w.split("/").all(|d| d.parse::<f64>().is_ok()) => {
                Edn::Rational(w)
            }
            w if w.parse::<isize>().is_ok() => Edn::Int(w.parse::<isize>().unwrap()),
            w if w.parse::<usize>().is_ok() => Edn::UInt(w.parse::<usize>().unwrap()),
            w if w.parse::<f64>().is_ok() => Edn::Double(OrderedFloat(w.parse::<f64>().unwrap())),
            w => Edn::Symbol(w),
        }
    }
}

impl std::str::FromStr for Edn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::deserialize::parse_edn(s)
    }
}

fn to_double<T>(i: T) -> Result<f64, std::num::ParseFloatError>
where
    T: std::fmt::Debug,
{
    format!("{:?}", i).parse::<f64>()
}

fn rational_to_double(r: &str) -> Option<f64> {
    if r.split('/').count() == 2 {
        let vals = r
            .split('/')
            .map(ToString::to_string)
            .map(|v| v.parse::<f64>())
            .map(Result::ok)
            .collect::<Option<Vec<f64>>>()?;
        return Some(vals[0] / vals[1]);
    }
    None
}

#[test]
fn parses_rationals() {
    assert_eq!(rational_to_double("3/4").unwrap(), 0.75f64);
    assert_eq!(rational_to_double("25/5").unwrap(), 5f64);
    assert_eq!(rational_to_double("15/4").unwrap(), 3.75f64);
    assert_eq!(rational_to_double("3 4"), None);
    assert_eq!(rational_to_double("3/4/5"), None);
    assert_eq!(rational_to_double("text/moretext"), None);
}
