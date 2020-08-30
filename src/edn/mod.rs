use crate::deserialize;
use std::cmp::{Ord, PartialOrd};
use std::collections::{BTreeMap, BTreeSet};
use utils::index::Index;

#[cfg(feature = "async")]
use core::pin::Pin;
#[cfg(feature = "async")]
use futures::prelude::*;
#[cfg(feature = "async")]
use futures::task;
#[cfg(feature = "async")]
use futures::task::Poll;

#[doc(hidden)]
pub mod utils;

/// `EdnType` is an Enum with possible values for an EDN type
/// Symbol and Char are not yet implemented
/// String implementation of Edn can be obtained with `.to_string()`
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

#[cfg(feature = "async")]
impl futures::future::Future for Edn {
    type Output = Edn;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<Self::Output> {
        if !self.to_string().is_empty() {
            let pinned = self.to_owned();
            Poll::Ready(pinned)
        } else {
            Poll::Pending
        }
    }
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

#[cfg(feature = "async")]
impl futures::future::Future for Vector {
    type Output = Vector;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<Self::Output> {
        if self.0.len() >= 0 {
            let pinned = self.to_owned();
            Poll::Ready(pinned)
        } else {
            Poll::Pending
        }
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

#[cfg(feature = "async")]
impl futures::future::Future for List {
    type Output = List;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<Self::Output> {
        if self.0.len() >= 0 {
            let pinned = self.to_owned();
            Poll::Ready(pinned)
        } else {
            Poll::Pending
        }
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

#[cfg(feature = "async")]
impl futures::future::Future for Set {
    type Output = Set;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<Self::Output> {
        if self.0.len() >= 0 {
            let pinned = self.to_owned();
            Poll::Ready(pinned)
        } else {
            Poll::Pending
        }
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

#[cfg(feature = "async")]
impl futures::future::Future for Map {
    type Output = Map;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<Self::Output> {
        if self.0.len() >= 0 {
            let pinned = self.to_owned();
            Poll::Ready(pinned)
        } else {
            Poll::Pending
        }
    }
}

#[derive(Clone, Ord, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct Double(i64, u64);

impl From<f64> for Double {
    fn from(f: f64) -> Double {
        let f_as_str = format!("{}", f);
        let f_split = f_as_str.split(".").collect::<Vec<&str>>();
        Double(
            f_split[0].parse::<i64>().unwrap(),
            f_split
                .get(1)
                .unwrap_or(&"0")
                .chars()
                .rev()
                .collect::<String>()
                .parse::<u64>()
                .unwrap(),
        )
    }
}

#[cfg(feature = "async")]
impl futures::future::Future for Double {
    type Output = Double;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<Self::Output> {
        if !self.to_string().is_empty() {
            let pinned = self.to_owned();
            Poll::Ready(pinned)
        } else {
            Poll::Pending
        }
    }
}

impl std::fmt::Display for Double {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}",
            self.0,
            self.1.to_string().chars().rev().collect::<String>()
        )
    }
}

impl Double {
    fn to_float(&self) -> f64 {
        format!(
            "{}.{}",
            self.0,
            self.1.to_string().chars().rev().collect::<String>()
        )
        .parse::<f64>()
        .unwrap()
    }
}

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
            Edn::Key(k) => k.parse::<f64>().ok(),
            Edn::Str(s) => s.parse::<f64>().ok(),
            Edn::Int(i) => to_double(i).ok(),
            Edn::UInt(u) => to_double(u).ok(),
            Edn::Double(d) => Some(d.to_float()),
            Edn::Rational(r) => rational_to_double(&r),
            _ => None,
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
            Edn::Key(k) => k.parse::<isize>().ok(),
            Edn::Str(s) => s.parse::<isize>().ok(),
            Edn::Int(i) => Some(i.to_owned() as isize),
            Edn::Double(d) => Some(d.to_owned().to_float().round() as isize),
            Edn::Rational(r) => Some(rational_to_double(&r).unwrap_or(0f64).round() as isize),
            _ => None,
        }
    }

    /// Similar to `to_int` but returns an `Option<usize>`
    pub fn to_uint(&self) -> Option<usize> {
        match self {
            Edn::Str(s) => s.parse::<usize>().ok(),
            Edn::Int(i) => Some(i.to_owned() as usize),
            Edn::UInt(i) => Some(i.to_owned()),
            Edn::Double(d) => Some(d.to_owned().to_float().round() as usize),
            Edn::Rational(r) => Some(rational_to_double(&r).unwrap_or(0f64).round() as usize),
            _ => None,
        }
    }

    /// `to_bool` takes an `Edn` and returns an `Option<bool>` with its value. Most types return None
    /// ```rust
    /// use edn_rs::edn::{Edn};
    ///
    /// let b = Edn::Bool(true);
    /// let s = Edn::Str("true".to_string());
    /// let symbol = Edn::Symbol("false".to_string());
    ///
    /// assert_eq!(b.to_bool().unwrap(),true);
    /// assert_eq!(s.to_bool().unwrap(),true);
    /// assert_eq!(symbol.to_bool().unwrap(),false);
    /// ```
    pub fn to_bool(&self) -> Option<bool> {
        match self {
            Edn::Bool(b) => Some(*b),
            Edn::Str(s) => s.parse::<bool>().ok(),
            Edn::Symbol(s) => s.parse::<bool>().ok(),
            _ => None,
        }
    }

    /// `to_char` takes an `Edn` and returns an `Option<char>` with its value. Most types return None
    /// ```rust
    /// use edn_rs::edn::{Edn};
    ///
    /// let c = Edn::Char('c');
    /// let symbol = Edn::Symbol("false".to_string());
    ///
    /// assert_eq!(c.to_char().unwrap(),'c');
    /// assert_eq!(symbol.to_char(), None);
    /// ```
    pub fn to_char(&self) -> Option<char> {
        match self {
            Edn::Char(c) => Some(*c),
            _ => None,
        }
    }

    /// `to_vec` converts `Edn` types `Vector`, `List` and `Set` into an `Option<Vec<String>>`.
    /// Type String was selected because it is the current way to mix floats, integers and Strings.
    pub fn to_vec(&self) -> Option<Vec<String>> {
        match self {
            Edn::Vector(_) => Some(
                self.iter()
                    .unwrap()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>(),
            ),
            Edn::List(_) => Some(
                self.iter()
                    .unwrap()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>(),
            ),
            Edn::Set(_) => Some(
                self.iter()
                    .unwrap()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>(),
            ),
            _ => None,
        }
    }

    /// `to_int_vec` converts `Edn` types `Vector` `List` and `Set` into an `Option<Vec<isize>>`.
    /// All elements of this Edn structure should be of the same type
    pub fn to_int_vec(&self) -> Option<Vec<isize>> {
        match self {
            Edn::Vector(_) if !self.iter().unwrap().any(|e| e.to_int().is_none()) => Some(
                self.iter()
                    .unwrap()
                    .map(|e| e.to_int().unwrap())
                    .collect::<Vec<isize>>(),
            ),
            Edn::List(_) if !self.iter().unwrap().any(|e| e.to_int().is_none()) => Some(
                self.iter()
                    .unwrap()
                    .map(|e| e.to_int().unwrap())
                    .collect::<Vec<isize>>(),
            ),
            Edn::Set(_) if !self.iter().unwrap().any(|e| e.to_int().is_none()) => Some(
                self.iter()
                    .unwrap()
                    .map(|e| e.to_int().unwrap())
                    .collect::<Vec<isize>>(),
            ),
            _ => None,
        }
    }

    /// `to_uint_vec` converts `Edn` types `Vector` `List` and `Set` into an `Option<Vec<usize>>`.
    /// All elements of this Edn structure should be of the same type
    pub fn to_uint_vec(&self) -> Option<Vec<usize>> {
        match self {
            Edn::Vector(_) if !self.iter().unwrap().any(|e| e.to_uint().is_none()) => Some(
                self.iter()
                    .unwrap()
                    .map(|e| e.to_uint().unwrap())
                    .collect::<Vec<usize>>(),
            ),
            Edn::List(_) if !self.iter().unwrap().any(|e| e.to_uint().is_none()) => Some(
                self.iter()
                    .unwrap()
                    .map(|e| e.to_uint().unwrap())
                    .collect::<Vec<usize>>(),
            ),
            Edn::Set(_) if !self.iter().unwrap().any(|e| e.to_uint().is_none()) => Some(
                self.iter()
                    .unwrap()
                    .map(|e| e.to_uint().unwrap())
                    .collect::<Vec<usize>>(),
            ),
            _ => None,
        }
    }

    /// `to_float_vec` converts `Edn` types `Vector` `List` and `Set` into an `Option<Vec<f64>>`.
    /// All elements of this Edn structure should be of the same type
    pub fn to_float_vec(&self) -> Option<Vec<f64>> {
        match self {
            Edn::Vector(_) if !self.iter().unwrap().any(|e| e.to_float().is_none()) => Some(
                self.iter()
                    .unwrap()
                    .map(|e| e.to_float().unwrap())
                    .collect::<Vec<f64>>(),
            ),
            Edn::List(_) if !self.iter().unwrap().any(|e| e.to_float().is_none()) => Some(
                self.iter()
                    .unwrap()
                    .map(|e| e.to_float().unwrap())
                    .collect::<Vec<f64>>(),
            ),
            Edn::Set(_) if !self.iter().unwrap().any(|e| e.to_float().is_none()) => Some(
                self.iter()
                    .unwrap()
                    .map(|e| e.to_float().unwrap())
                    .collect::<Vec<f64>>(),
            ),
            _ => None,
        }
    }

    /// `to_bool_vec` converts `Edn` types `Vector` `List` and `Set` into an `Option<Vec<bool>>`.
    /// All elements of this Edn structure should be of the same type
    pub fn to_bool_vec(&self) -> Option<Vec<bool>> {
        match self {
            Edn::Vector(_) if !self.iter().unwrap().any(|e| e.to_bool().is_none()) => Some(
                self.iter()
                    .unwrap()
                    .map(|e| e.to_bool().unwrap())
                    .collect::<Vec<bool>>(),
            ),
            Edn::List(_) if !self.iter().unwrap().any(|e| e.to_bool().is_none()) => Some(
                self.iter()
                    .unwrap()
                    .map(|e| e.to_bool().unwrap())
                    .collect::<Vec<bool>>(),
            ),
            Edn::Set(_) if !self.iter().unwrap().any(|e| e.to_bool().is_none()) => Some(
                self.iter()
                    .unwrap()
                    .map(|e| e.to_bool().unwrap())
                    .collect::<Vec<bool>>(),
            ),
            _ => None,
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

    /// `iter` returns am `Option<Iter<Edn>>` with `Some` for types `Edn::Vector` and `Edn::List`
    /// Other types return `None`
    /// ```
    /// use edn_rs::{Edn, Vector};
    ///
    /// fn main() {
    ///     let v = Edn::Vector(Vector::new(vec![Edn::Int(5), Edn::Int(6), Edn::Int(7)]));
    ///     let sum = v.iter().unwrap().filter(|e| e.to_int().is_some()).map(|e| e.to_int().unwrap()).sum();
    ///
    ///     assert_eq!(18isize, sum);
    /// }
    /// ```
    pub fn iter(&self) -> Option<std::slice::Iter<'_, Edn>> {
        match self {
            Edn::Vector(v) => Some(v.0.iter()),
            Edn::List(l) => Some(l.0.iter()),
            _ => None,
        }
    }

    /// `set_iter` returns am `Option<btree_set::Iter<Edn>>` with `Some` for type `Edn::Set`
    /// Other types return `None`
    pub fn set_iter(&self) -> Option<std::collections::btree_set::Iter<'_, Edn>> {
        match self {
            Edn::Set(s) => Some(s.0.iter()),
            _ => None,
        }
    }

    /// `map_iter` returns am `Option<btree_map::Iter<String, Edn>>` with `Some` for type `Edn::Map`
    /// Other types return `None`
    pub fn map_iter(&self) -> Option<std::collections::btree_map::Iter<'_, String, Edn>> {
        match self {
            Edn::Map(m) => Some(m.0.iter()),
            _ => None,
        }
    }
}

impl std::str::FromStr for Edn {
    type Err = Error;

    /// Parses a `&str` that contains an Edn into `Result<Edn, EdnError>`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = deserialize::tokenize(s);
        let edn = deserialize::parse(tokens.next(), &mut tokens);
        Ok(edn)
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

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseEdn(String),
    Deserialize(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::ParseEdn(s)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::ParseEdn(s) => &s,
            Error::Deserialize(s) => &s,
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        Some(self)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseEdn(s) => write!(f, "{}", &s),
            Error::Deserialize(s) => write!(f, "{}", &s),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parses_rationals() {
        assert_eq!(rational_to_double("3/4").unwrap(), 0.75f64);
        assert_eq!(rational_to_double("25/5").unwrap(), 5f64);
        assert_eq!(rational_to_double("15/4").unwrap(), 3.75f64);
        assert_eq!(rational_to_double("3 4"), None);
        assert_eq!(rational_to_double("3/4/5"), None);
        assert_eq!(rational_to_double("text/moretext"), None);
    }

    #[test]
    fn iterator() {
        let v = Edn::Vector(Vector::new(vec![Edn::Int(5), Edn::Int(6), Edn::Int(7)]));
        let sum = v
            .iter()
            .unwrap()
            .filter(|e| e.to_int().is_some())
            .map(|e| e.to_int().unwrap())
            .sum();

        assert_eq!(18isize, sum);
    }

    #[test]
    fn to_vec() {
        let edn = Edn::Vector(Vector::new(vec![Edn::Int(5), Edn::Int(6), Edn::Int(7)]));
        let v = vec![String::from("5"), String::from("6"), String::from("7")];

        assert_eq!(edn.to_vec().unwrap(), v);
    }

    #[test]
    fn double_deals_with_decimal_zeros() {
        let double = Double::from(-3.0000564f64);

        assert_eq!(double.to_float(), -3.0000564f64);
    }

    #[test]
    fn double_deals_without_decimal_zeros() {
        let double = Double::from(45843.835832564f64);

        assert_eq!(double.to_float(), 45843.835832564f64);
    }

    #[test]
    fn to_char() {
        let c = Edn::Char('c');
        let symbol = Edn::Symbol("d".to_string());

        assert_eq!(c.to_char().unwrap(), 'c');
        assert_eq!(symbol.to_char(), None);
    }

    #[test]
    fn to_int_vec() {
        let edn = Edn::Vector(Vector::new(vec![Edn::Int(5), Edn::Int(6), Edn::Int(7)]));
        let v = vec![5isize, 6isize, 7isize];

        assert_eq!(edn.to_int_vec().unwrap(), v);
    }

    #[test]
    fn to_uint_vec() {
        let edn = Edn::Vector(Vector::new(vec![Edn::UInt(5), Edn::UInt(6), Edn::UInt(7)]));
        let v = vec![5usize, 6usize, 7usize];

        assert_eq!(edn.to_uint_vec().unwrap(), v);
    }

    #[test]
    fn to_float_vec() {
        let edn = Edn::Vector(Vector::new(vec![
            Edn::Double(5.5.into()),
            Edn::Double(6.6.into()),
            Edn::Double(7.7.into()),
        ]));
        let v = vec![5.5f64, 6.6f64, 7.7f64];

        assert_eq!(edn.to_float_vec().unwrap(), v);
    }

    #[test]
    fn to_bool_vec() {
        let edn = Edn::Vector(Vector::new(vec![
            Edn::Bool(true),
            Edn::Bool(true),
            Edn::Bool(false),
        ]));
        let v = vec![true, true, false];

        assert_eq!(edn.to_bool_vec().unwrap(), v);
    }

    #[test]
    fn to_bool_vec_with_non_bool_is_none() {
        let edn = Edn::Vector(Vector::new(vec![
            Edn::Bool(true),
            Edn::Int(5),
            Edn::Bool(false),
        ]));

        assert_eq!(edn.to_bool_vec(), None);
    }
}
