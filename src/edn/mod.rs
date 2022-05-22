use crate::deserialize::parse::{self};
use std::cmp::{Ord, PartialOrd};
use std::collections::{BTreeMap, BTreeSet};
use std::convert::TryFrom;
use utils::index::Index;

#[cfg(feature = "async")]
use core::pin::Pin;
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
    Tagged(String, Box<Edn>),
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
    Inst(String),
    Uuid(String),
    NamespacedMap(String, Map),
    Nil,
    Empty,
}

#[cfg(feature = "async")]
impl futures::future::Future for Edn {
    type Output = Edn;

    fn poll(self: Pin<&mut Self>, _cx: &mut task::Context) -> Poll<Self::Output> {
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
    #[must_use]
    pub fn new(v: Vec<Edn>) -> Vector {
        Vector(v)
    }

    #[must_use]
    pub fn empty() -> Vector {
        Vector(Vec::new())
    }

    #[must_use]
    pub fn to_vec(self) -> Vec<Edn> {
        self.0
    }
}

#[cfg(feature = "async")]
impl futures::future::Future for Vector {
    type Output = Vector;

    #[allow(unused_comparisons)]
    fn poll(self: Pin<&mut Self>, _cx: &mut task::Context) -> Poll<Self::Output> {
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
    #[must_use]
    pub fn new(v: Vec<Edn>) -> List {
        List(v)
    }

    #[must_use]
    pub fn empty() -> List {
        List(Vec::new())
    }

    #[must_use]
    pub fn to_vec(self) -> Vec<Edn> {
        self.0
    }
}

#[cfg(feature = "async")]
impl futures::future::Future for List {
    type Output = List;

    #[allow(unused_comparisons)]
    fn poll(self: Pin<&mut Self>, _cx: &mut task::Context) -> Poll<Self::Output> {
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
    #[must_use]
    pub fn new(v: BTreeSet<Edn>) -> Set {
        Set(v)
    }

    #[must_use]
    pub fn empty() -> Set {
        Set(BTreeSet::new())
    }

    #[must_use]
    pub fn to_set(self) -> BTreeSet<Edn> {
        self.0
    }
}

#[cfg(feature = "async")]
impl futures::future::Future for Set {
    type Output = Set;

    #[allow(unused_comparisons)]
    fn poll(self: Pin<&mut Self>, _cx: &mut task::Context) -> Poll<Self::Output> {
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
    #[must_use]
    pub fn new(m: BTreeMap<String, Edn>) -> Map {
        Map(m)
    }

    #[must_use]
    pub fn empty() -> Map {
        Map(BTreeMap::new())
    }

    #[must_use]
    pub fn to_map(self) -> BTreeMap<String, Edn> {
        self.0
    }
}

#[cfg(feature = "async")]
impl futures::future::Future for Map {
    type Output = Map;

    #[allow(unused_comparisons)]
    fn poll(self: Pin<&mut Self>, _cx: &mut task::Context) -> Poll<Self::Output> {
        if self.0.len() >= 0 {
            let pinned = self.to_owned();
            Poll::Ready(pinned)
        } else {
            Poll::Pending
        }
    }
}

#[derive(Clone, Ord, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct Double(i64, u128);

impl From<f64> for Double {
    fn from(f: f64) -> Double {
        let f_as_str = format!("{}", f);
        let f_split = f_as_str.split('.').collect::<Vec<&str>>();
        Double(
            f_split[0].parse::<i64>().unwrap(),
            f_split
                .get(1)
                .unwrap_or(&"0")
                .chars()
                .rev()
                .collect::<String>()
                .parse::<u128>()
                .unwrap(),
        )
    }
}

#[cfg(feature = "async")]
impl futures::future::Future for Double {
    type Output = Double;

    fn poll(self: Pin<&mut Self>, _cx: &mut task::Context) -> Poll<Self::Output> {
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
        let floating = self.1.to_string();
        if floating.starts_with('0') {
            write!(f, "{}.{}", self.0, floating)
        } else {
            write!(
                f,
                "{}.{}",
                self.0,
                floating.chars().rev().collect::<String>()
            )
        }
    }
}

impl Double {
    fn to_float(&self) -> f64 {
        let floating = self.1.to_string();
        if floating.starts_with('0') {
            format!("{}.{}", self.0, floating).parse::<f64>().unwrap()
        } else {
            format!("{}.{}", self.0, floating.chars().rev().collect::<String>())
                .parse::<f64>()
                .unwrap()
        }
    }
}

impl core::fmt::Display for Vector {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(ToString::to_string)
                .fold(String::new(), |mut acc, i| {
                    acc.push_str(&i);
                    acc.push_str(", ");
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
                .map(ToString::to_string)
                .fold(String::new(), |mut acc, i| {
                    acc.push_str(&i);
                    acc.push_str(", ");
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
                .map(ToString::to_string)
                .fold(String::new(), |mut acc, i| {
                    acc.push_str(&i);
                    acc.push_str(", ");
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
            self.0.iter().map(|(k, v)| format!("{} {}, ", k, v)).fold(
                String::new(),
                |mut acc, i| {
                    acc.push_str(&i);
                    acc
                }
            )
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
            Edn::Str(s) => format!("{:?}", s),
            Edn::Int(i) => format!("{}", i),
            Edn::UInt(u) => format!("{}", u),
            Edn::Double(d) => format!("{}", d),
            Edn::Rational(r) => r.to_string(),
            Edn::Bool(b) => format!("{}", b),
            Edn::Char(c) => format!("{}", c),
            Edn::Inst(t) => format!("#inst \"{}\"", t),
            Edn::Uuid(t) => format!("#uuid \"{}\"", t),
            Edn::NamespacedMap(s, m) => format!(":{}{}", s, m),
            Edn::Nil => String::from("nil"),
            Edn::Empty => String::from(""),
            Edn::Tagged(tag, edn) => format!("#{} {}", tag, edn),
        };
        write!(f, "{}", text)
    }
}

impl Edn {
    /// `to_float` takes an `Edn` and returns an `Option<f64>` with its value. Most types return None
    /// ```rust
    /// use edn_rs::edn::{Edn, Vector};
    ///
    /// let key = Edn::Key(String::from(":1234"));
    /// let q = Edn::Rational(String::from("3/4"));
    /// let i = Edn::Int(12isize);
    ///
    /// assert_eq!(Edn::Vector(Vector::empty()).to_float(), None);
    /// assert_eq!(key.to_float().unwrap(),1234f64);
    /// assert_eq!(q.to_float().unwrap(), 0.75f64);
    /// assert_eq!(i.to_float().unwrap(), 12f64);
    /// ```
    #[must_use]
    pub fn to_float(&self) -> Option<f64> {
        match self {
            Edn::Key(k) => k.replace(':', "").parse::<f64>().ok(),
            Edn::Str(s) => s.parse::<f64>().ok(),
            Edn::Int(i) => to_double(i).ok(),
            Edn::UInt(u) => to_double(u).ok(),
            Edn::Double(d) => Some(d.to_float()),
            Edn::Rational(r) => rational_to_double(r),
            _ => None,
        }
    }

    /// `to_int` takes an `Edn` and returns an `Option<isize>` with its value. Most types return None
    /// ```rust
    /// use edn_rs::edn::{Edn, Vector};
    ///
    /// let key = Edn::Key(String::from(":1234"));
    /// let q = Edn::Rational(String::from("3/4"));
    /// let f = Edn::Double(12.3f64.into());
    ///
    /// assert_eq!(Edn::Vector(Vector::empty()).to_float(), None);
    /// assert_eq!(key.to_int().unwrap(),1234isize);
    /// assert_eq!(q.to_int().unwrap(), 1isize);
    /// assert_eq!(f.to_int().unwrap(), 12isize);
    /// ```
    #[must_use]
    pub fn to_int(&self) -> Option<isize> {
        match self {
            Edn::Key(k) => k.replace(':', "").parse::<isize>().ok(),
            Edn::Str(s) => s.parse::<isize>().ok(),
            Edn::Int(i) => Some(*i as isize),
            #[allow(clippy::cast_possible_wrap)]
            Edn::UInt(u) if isize::try_from(*u).is_ok() => Some(*u as isize),
            #[allow(clippy::cast_possible_truncation)]
            Edn::Double(d) => Some(d.clone().to_float().round() as isize),
            #[allow(clippy::cast_possible_truncation)]
            Edn::Rational(r) => Some(rational_to_double(r).unwrap_or(0f64).round() as isize),
            _ => None,
        }
    }

    /// Similar to `to_int` but returns an `Option<usize>`
    #[must_use]
    pub fn to_uint(&self) -> Option<usize> {
        match self {
            Edn::Str(s) => s.parse::<usize>().ok(),
            #[allow(clippy::cast_sign_loss)]
            Edn::Int(i) if i > &0 => Some(*i as usize),
            Edn::UInt(i) => Some(*i),
            Edn::Double(d) if d.to_float() > 0f64 =>
            {
                #[allow(clippy::cast_sign_loss)]
                #[allow(clippy::cast_possible_truncation)]
                Some(d.clone().to_float().round() as usize)
            }
            Edn::Rational(r) if !r.contains('-') =>
            {
                #[allow(clippy::cast_sign_loss)]
                #[allow(clippy::cast_possible_truncation)]
                Some(rational_to_double(r)?.round() as usize)
            }
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
    #[must_use]
    pub fn to_bool(&self) -> Option<bool> {
        match self {
            Edn::Bool(b) => Some(*b),
            Edn::Str(s) | Edn::Symbol(s) => s.parse::<bool>().ok(),
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
    #[must_use]
    pub fn to_char(&self) -> Option<char> {
        match self {
            Edn::Char(c) => Some(*c),
            _ => None,
        }
    }

    /// `to_vec` converts `Edn` types `Vector`, `List` and `Set` into an `Option<Vec<String>>`.
    /// Type String was selected because it is the current way to mix floats, integers and Strings.
    #[must_use]
    pub fn to_vec(&self) -> Option<Vec<String>> {
        match self {
            Edn::Vector(_) => Some(
                self.iter_some()?
                    .map(|e| match e {
                        Edn::Str(s) => (s.clone()),
                        _ => e.to_string(),
                    })
                    .collect::<Vec<String>>(),
            ),
            Edn::List(_) => Some(
                self.iter_some()?
                    .map(|e| match e {
                        Edn::Str(s) => (s.clone()),
                        _ => e.to_string(),
                    })
                    .collect::<Vec<String>>(),
            ),
            Edn::Set(_) => Some(
                self.iter_some()?
                    .map(|e| match e {
                        Edn::Str(s) => (s.clone()),
                        _ => e.to_string(),
                    })
                    .collect::<Vec<String>>(),
            ),
            _ => None,
        }
    }

    /// `to_int_vec` converts `Edn` types `Vector` `List` and `Set` into an `Option<Vec<isize>>`.
    /// All elements of this Edn structure should be of the same type
    #[must_use]
    pub fn to_int_vec(&self) -> Option<Vec<isize>> {
        match self {
            Edn::Vector(_) if !self.iter_some()?.any(|e| e.to_int().is_none()) => Some(
                self.iter_some()?
                    .map(Edn::to_int)
                    .collect::<Option<Vec<isize>>>()?,
            ),
            Edn::List(_) if !self.iter_some()?.any(|e| e.to_int().is_none()) => Some(
                self.iter_some()?
                    .map(Edn::to_int)
                    .collect::<Option<Vec<isize>>>()?,
            ),
            Edn::Set(_) if !self.iter_some()?.any(|e| e.to_int().is_none()) => Some(
                self.iter_some()?
                    .map(Edn::to_int)
                    .collect::<Option<Vec<isize>>>()?,
            ),
            _ => None,
        }
    }

    /// `to_uint_vec` converts `Edn` types `Vector` `List` and `Set` into an `Option<Vec<usize>>`.
    /// All elements of this Edn structure should be of the same type
    #[must_use]
    pub fn to_uint_vec(&self) -> Option<Vec<usize>> {
        match self {
            Edn::Vector(_) if !self.iter_some()?.any(|e| e.to_uint().is_none()) => Some(
                self.iter_some()?
                    .map(Edn::to_uint)
                    .collect::<Option<Vec<usize>>>()?,
            ),
            Edn::List(_) if !self.iter_some()?.any(|e| e.to_uint().is_none()) => Some(
                self.iter_some()?
                    .map(Edn::to_uint)
                    .collect::<Option<Vec<usize>>>()?,
            ),
            Edn::Set(_) if !self.iter_some()?.any(|e| e.to_uint().is_none()) => Some(
                self.iter_some()?
                    .map(Edn::to_uint)
                    .collect::<Option<Vec<usize>>>()?,
            ),
            _ => None,
        }
    }

    /// `to_float_vec` converts `Edn` types `Vector` `List` and `Set` into an `Option<Vec<f64>>`.
    /// All elements of this Edn structure should be of the same type
    #[must_use]
    pub fn to_float_vec(&self) -> Option<Vec<f64>> {
        match self {
            Edn::Vector(_) if !self.iter_some()?.any(|e| e.to_float().is_none()) => Some(
                self.iter_some()?
                    .map(Edn::to_float)
                    .collect::<Option<Vec<f64>>>()?,
            ),
            Edn::List(_) if !self.iter_some()?.any(|e| e.to_float().is_none()) => Some(
                self.iter_some()?
                    .map(Edn::to_float)
                    .collect::<Option<Vec<f64>>>()?,
            ),
            Edn::Set(_) if !self.iter_some()?.any(|e| e.to_float().is_none()) => Some(
                self.iter_some()?
                    .map(Edn::to_float)
                    .collect::<Option<Vec<f64>>>()?,
            ),
            _ => None,
        }
    }

    /// `to_bool_vec` converts `Edn` types `Vector` `List` and `Set` into an `Option<Vec<bool>>`.
    /// All elements of this Edn structure should be of the same type
    #[must_use]
    pub fn to_bool_vec(&self) -> Option<Vec<bool>> {
        match self {
            Edn::Vector(_) if !self.iter_some()?.any(|e| e.to_bool().is_none()) => Some(
                self.iter_some()?
                    .map(Edn::to_bool)
                    .collect::<Option<Vec<bool>>>()?,
            ),
            Edn::List(_) if !self.iter_some()?.any(|e| e.to_bool().is_none()) => Some(
                self.iter_some()?
                    .map(Edn::to_bool)
                    .collect::<Option<Vec<bool>>>()?,
            ),
            Edn::Set(_) if !self.iter_some()?.any(|e| e.to_bool().is_none()) => Some(
                self.iter_some()?
                    .map(Edn::to_bool)
                    .collect::<Option<Vec<bool>>>()?,
            ),
            _ => None,
        }
    }

    /// **[`std::fmt::Debug`]**
    /// `to_debug` is a wrapper of `format!("{:?}", &self)` for `&Edn`.
    /// ```
    /// use edn_rs::edn::{Edn, Vector};
    ///
    /// let edn = Edn::Vector(Vector::new(vec![Edn::Int(5), Edn::Int(6), Edn::Int(7)]));
    /// let expected = "Vector(Vector([Int(5), Int(6), Int(7)]))";
    ///
    /// assert_eq!(edn.to_debug(), expected);
    /// ```
    ///
    /// While `to_string` returns a valid edn:
    ///
    ///  ```
    /// use edn_rs::edn::{Edn, Vector};
    ///
    /// let edn = Edn::Vector(Vector::new(vec![Edn::Int(5), Edn::Int(6), Edn::Int(7)]));
    /// let expected = "[5, 6, 7, ]";
    ///
    /// assert_eq!(edn.to_string(), expected);
    /// ```
    ///
    #[allow(clippy::must_use_candidate)]
    pub fn to_debug(&self) -> String {
        format!("{:?}", self)
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
    ///     assert_eq!(edn[3].get("false").unwrap(), &Edn::Key(":f".to_string()));
    /// }
    /// ```
    #[must_use]
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
    ///     assert_eq!(edn[3].get_mut("false").unwrap(), &Edn::Key(":f".to_string()));
    /// }
    /// ```
    #[must_use]
    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Edn> {
        index.index_into_mut(self)
    }

    /// `iter_some` returns an `Option<Iter<Edn>>` with `Some` for types `Edn::Vector` and `Edn::List`
    /// Other types return `None`
    /// ```
    /// use edn_rs::{Edn, Vector};
    ///
    /// fn main() {
    ///     let v = Edn::Vector(Vector::new(vec![Edn::Int(5), Edn::Int(6), Edn::Int(7)]));
    ///     let sum = v.iter_some().unwrap().filter(|e| e.to_int().is_some()).map(|e| e.to_int().unwrap()).sum();
    ///
    ///     assert_eq!(18isize, sum);
    /// }
    /// ```
    #[allow(clippy::needless_doctest_main)]
    #[must_use]
    pub fn iter_some(&self) -> Option<std::slice::Iter<'_, Edn>> {
        match self {
            Edn::Vector(v) => Some(v.0.iter()),
            Edn::List(l) => Some(l.0.iter()),
            _ => None,
        }
    }

    /// `set_iter` returns am `Option<btree_set::Iter<Edn>>` with `Some` for type `Edn::Set`
    /// Other types return `None`
    #[must_use]
    pub fn set_iter(&self) -> Option<std::collections::btree_set::Iter<'_, Edn>> {
        match self {
            Edn::Set(s) => Some(s.0.iter()),
            _ => None,
        }
    }

    /// `map_iter` returns am `Option<btree_map::Iter<String, Edn>>` with `Some` for type `Edn::Map`
    /// Other types return `None`
    #[must_use]
    pub fn map_iter(&self) -> Option<std::collections::btree_map::Iter<'_, String, Edn>> {
        match self {
            Edn::Map(m) | Edn::NamespacedMap(_, m) => Some(m.0.iter()),
            _ => None,
        }
    }

    /// `to_str_uuid` returns am `Option<String>` with `Some` containing the string representing the UUID for type `Edn::Uuid`
    /// Other types return `None`
    #[must_use]
    pub fn to_str_uuid(&self) -> Option<String> {
        if let Edn::Uuid(uuid) = self {
            Some(uuid.clone())
        } else {
            None
        }
    }

    /// `to_str_INST` returns am `Option<String>` with `Some` containing the string representing the instant for type `Edn::Inst`
    /// Other types return `None`
    #[must_use]
    pub fn to_str_inst(&self) -> Option<String> {
        if let Edn::Inst(inst) = self {
            Some(inst.clone())
        } else {
            None
        }
    }

    /// Method `to_json` allows you to convert a `edn_rs::Edn` into a JSON string. Type convertions are:
    /// `Edn::Vector(v)` => a vector like `[value1, value2, ..., valueN]`
    /// `Edn::Set(s)` => a vector like `[value1, value2, ..., valueN]`
    /// `Edn::Map(map)` => a map like `{\"key1\": value1, ..., \"keyN\": valueN}`
    /// `Edn::List(l)` => a vector like `[value1, value2, ..., valueN]`
    /// `Edn::Key(key)` => a `camelCase` version of the `:kebab-case` keyword,
    /// `Edn::Symbol(s)` => `\"a-simple-string\"`
    /// `Edn::Str(s)` => `\"a simple string\"`
    /// `Edn::Int(n)` => a number like `5`
    /// `Edn::UInt(n)` => a number like `5`
    /// `Edn::Double(n)` => a number like `3.14`
    /// `Edn::Rational(r)` => a number like `0.25` for `1/4`.
    /// `Edn::Char(c)` => a simple char `\'c\'`
    /// `Edn::Bool(b)` => boolean options, `true` and `false`
    /// `Edn::Inst(inst)` => a DateTime string like `\"2020-10-21T00:00:00.000-00:00\"`
    /// `Edn::Uuid(uuid)` => a UUID string like `\"7a6b6722-0221-4280-865e-ad41060d53b2\"`
    /// `Edn::NamespacedMap(ns, map)` => a namespaced map like `{\"nameSpace\": {\"key1\": value1, ..., \"keyN\": valueN}}`
    /// `Edn::Nil` => `null`
    /// `Edn::Empty` => empty value, ` `
    /// ```
    /// use std::str::FromStr;
    ///
    /// fn complex_json() {
    ///     let edn = "{ :people-list  [ { :first-name \"otavio\", :age 22 }, { :first-name \"Julia\", :age 32.0 } ], :country-or-origin \"Brazil\", :queerentener true, :brain nil }";
    ///     let parsed_edn : edn_rs::Edn = edn_rs::Edn::from_str(edn).unwrap();
    ///     let actual_json = parsed_edn.to_json();
    ///
    ///     let expected = String::from(
    ///         "{\"brain\": null, \"countryOrOrigin\": \"Brazil\", \"peopleList\": [{\"age\": 22, \"firstName\": \"otavio\"}, {\"age\": 32.0, \"firstName\": \"Julia\"}], \"queerentener\": true}",
    ///     );
    ///
    ///     assert_eq!(
    ///         actual_json,
    ///         expected
    ///     );
    /// }
    /// ```
    #[cfg(feature = "json")]
    pub fn to_json(&self) -> String {
        crate::json::display_as_json(self)
    }
}

impl std::str::FromStr for Edn {
    type Err = Error;

    /// Parses a `&str` that contains an Edn into `Result<Edn, EdnError>`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean = String::from(s.trim_start());
        let mut tokens = parse::tokenize(&clean);
        let edn = parse::parse(tokens.next(), &mut tokens)?;
        Ok(edn)
    }
}

fn to_double<T>(i: T) -> Result<f64, std::num::ParseFloatError>
where
    T: std::fmt::Debug,
{
    format!("{:?}", i).parse::<f64>()
}

pub(crate) fn rational_to_double(r: &str) -> Option<f64> {
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
    Iter(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::ParseEdn(s)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(s: std::num::ParseIntError) -> Self {
        Error::ParseEdn(s.to_string())
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(s: std::num::ParseFloatError) -> Self {
        Error::ParseEdn(s.to_string())
    }
}

impl From<std::str::ParseBoolError> for Error {
    fn from(s: std::str::ParseBoolError) -> Self {
        Error::ParseEdn(s.to_string())
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::ParseEdn(s) | Error::Deserialize(s) | Error::Iter(s) => s,
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        Some(self)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseEdn(s) | Error::Deserialize(s) | Error::Iter(s) => write!(f, "{}", &s),
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
            .iter_some()
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

    #[test]
    fn edn_to_string() {
        let edn = Edn::Map(Map::new(
            map! {":a".to_string() => Edn::Key(":something".to_string()),
            ":b".to_string() => Edn::Bool(false), ":c".to_string() => Edn::Nil},
        ));
        assert_eq!(edn.to_string(), "{:a :something, :b false, :c nil, }");
    }

    #[test]
    fn edn_to_debug() {
        let edn = Edn::Map(Map::new(
            map! {":a".to_string() => Edn::Key(":something".to_string()),
            ":b".to_string() => Edn::Bool(false), ":c".to_string() => Edn::Nil},
        ));
        let expected = "Map(Map({\":a\": Key(\":something\"), \":b\": Bool(false), \":c\": Nil}))";
        assert_eq!(edn.to_debug(), expected);
    }

    #[test]
    fn negative_isize_to_usize() {
        let neg_i = Edn::Int(-10);

        assert_eq!(neg_i.to_uint(), None);
    }

    #[test]
    fn max_usize_to_uint() {
        let max_u = Edn::UInt(usize::MAX);

        assert_eq!(max_u.to_int(), None);
    }

    #[test]
    fn positive_isize_to_usize() {
        let max_i = Edn::Int(isize::MAX);

        assert_eq!(max_i.to_uint(), Some(isize::MAX as usize));
    }

    #[test]
    fn small_usize_to_isize() {
        let small_u = Edn::UInt(10);

        assert_eq!(small_u.to_int(), Some(10));
    }

    #[test]
    fn regression_to_vec() {
        let expected = vec!["true", ":b", "test"];
        let edn = Edn::Vector(Vector(vec![
            Edn::Bool(true),
            Edn::Key(":b".to_string()),
            Edn::Str("test".to_string()),
        ]));
        let edn_vec = edn.to_vec().unwrap();

        assert_eq!(edn_vec, expected);
    }

    #[test]
    fn namespaced_map_to_string() {
        assert_eq!(
            ":abc{0 :val, 1 :value, }",
            Edn::NamespacedMap(
                "abc".to_string(),
                Map::new(map! {
                    "0".to_string() => Edn::Key(":val".to_string()),
                    "1".to_string() => Edn::Key(":value".to_string())
                })
            )
            .to_string()
        );
    }

    #[test]
    fn inst_to_string() {
        let inst = Edn::Inst("2020-09-18T01:16:25.909-00:00".to_string());

        assert_eq!(inst.to_string(), "#inst \"2020-09-18T01:16:25.909-00:00\"");
        let str_inst: String = crate::deserialize::from_edn(&inst).unwrap();
        assert_eq!(str_inst, "#inst \"2020-09-18T01:16:25.909-00:00\"");
    }

    #[test]
    fn inst_to_str_inst() {
        let inst = Edn::Inst("2020-09-18T01:16:25.909-00:00".to_string());

        assert_eq!(inst.to_str_inst().unwrap(), "2020-09-18T01:16:25.909-00:00");

        let uuid = Edn::Uuid("af6d8699-f442-4dfd-8b26-37d80543186b".to_string());
        assert_eq!(uuid.to_str_inst(), None);
    }

    #[test]
    fn uuid_to_string() {
        let uuid = Edn::Uuid("af6d8699-f442-4dfd-8b26-37d80543186b".to_string());

        assert_eq!(
            uuid.to_string(),
            "#uuid \"af6d8699-f442-4dfd-8b26-37d80543186b\""
        );
        let str_uuid: String = crate::deserialize::from_edn(&uuid).unwrap();
        assert_eq!(str_uuid, "#uuid \"af6d8699-f442-4dfd-8b26-37d80543186b\"");
    }

    #[test]
    fn uuid_to_str_uuid() {
        let uuid = Edn::Uuid("af6d8699-f442-4dfd-8b26-37d80543186b".to_string());

        assert_eq!(
            uuid.to_str_uuid().unwrap(),
            "af6d8699-f442-4dfd-8b26-37d80543186b"
        );

        let inst = Edn::Inst("2020-09-18T01:16:25.909-00:00".to_string());
        assert_eq!(inst.to_str_uuid(), None);
    }

    #[test]
    fn get_vec_at() {
        let expected = &Edn::Key(":b".to_string());
        let edn = Edn::Vector(Vector(vec![
            Edn::Bool(true),
            Edn::Key(":b".to_string()),
            Edn::Str("test".to_string()),
        ]));
        let val = &edn[Edn::UInt(1)];

        assert_eq!(val, expected);
    }

    #[test]
    fn get_list_at() {
        let expected = &Edn::Str("test".to_string());
        let edn = Edn::Vector(Vector(vec![
            Edn::Bool(true),
            Edn::Key(":b".to_string()),
            Edn::Str("test".to_string()),
        ]));
        let val = &edn[Edn::Int(2)];

        assert_eq!(val, expected);
    }

    #[test]
    fn get_namespaced_map() {
        let expected = &Edn::Key(":val".to_string());
        let ns_map = Edn::NamespacedMap(
            "abc".to_string(),
            Map::new(map! {
                "0".to_string() => Edn::Key(":val".to_string()),
                "1".to_string() => Edn::Key(":value".to_string())
            }),
        );

        let val = &ns_map[Edn::UInt(0)];
        assert_eq!(expected, val);
    }

    #[test]
    fn get_map() {
        let expected = &Edn::Key(":val".to_string());
        let map = Edn::Map(Map::new(map! {
            ":key".to_string() => Edn::Key(":val".to_string()),
            "1".to_string() => Edn::Key(":value".to_string())
        }));

        let val = &map[Edn::Key(":key".to_owned())];
        assert_eq!(expected, val);
    }
}
