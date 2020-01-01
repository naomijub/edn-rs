use std::fmt;
use std::ops;
use crate::edn::{Edn, Map};

/// This is a Copy of [Serde_json::index](https://docs.serde.rs/src/serde_json/value/index.rs.html)
pub trait Index: private::Sealed {
    #[doc(hidden)]
    fn index_into<'v>(&self, v: &'v Edn) -> Option<&'v Edn>;

    #[doc(hidden)]
    fn index_into_mut<'v>(&self, v: &'v mut Edn) -> Option<&'v mut Edn>;

    #[doc(hidden)]
    fn index_or_insert<'v>(&self, v: &'v mut Edn) -> &'v mut Edn;
}

impl Index for usize {
    fn index_into<'v>(&self, v: &'v Edn) -> Option<&'v Edn> {
        match *v {
            Edn::Vector(ref vec) => vec.0.get(*self),
            Edn::List(ref vec) => vec.0.get(*self),
            Edn::Set(ref vec) => vec.0.get(*self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Edn) -> Option<&'v mut Edn> {
        match *v {
            Edn::Vector(ref mut vec) => vec.0.get_mut(*self),
            Edn::List(ref mut vec) => vec.0.get_mut(*self),
            Edn::Set(ref mut vec) => vec.0.get_mut(*self),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, v: &'v mut Edn) -> &'v mut Edn {
        match *v {
            Edn::Vector(ref mut vec) => {
                let len = vec.0.len();
                vec.0.get_mut(*self).unwrap_or_else(|| {
                    panic!(
                        "cannot access index {} of EDN array of length {}",
                        self, len
                    )
                })
            }
            _ => panic!("cannot access index {} of EDN {}", self, Type(v)),
        }
    }
}

impl Index for str {
    fn index_into<'v>(&self, v: &'v Edn) -> Option<&'v Edn> {
        match *v {
            Edn::Map(ref map) => map.0.get(self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Edn) -> Option<&'v mut Edn> {
        match *v {
            Edn::Map(ref mut map) => map.0.get_mut(self),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, v: &'v mut Edn) -> &'v mut Edn {
        if let Edn::Nil = *v {
            *v = Edn::Map(Map::new(std::collections::HashMap::new()));
        }
        match *v {
            Edn::Map(ref mut map) => map.0.entry(self.to_owned()).or_insert(Edn::Nil),
            _ => panic!("cannot access key {:?} in EDN {}", self, Type(v)),
        }
    }
}

impl Index for String {
    fn index_into<'v>(&self, v: &'v Edn) -> Option<&'v Edn> {
        self[..].index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Edn) -> Option<&'v mut Edn> {
        self[..].index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Edn) -> &'v mut Edn {
        self[..].index_or_insert(v)
    }
}

impl<'a, T: ?Sized> Index for &'a T
where
    T: Index,
{
    fn index_into<'v>(&self, v: &'v Edn) -> Option<&'v Edn> {
        (**self).index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Edn) -> Option<&'v mut Edn> {
        (**self).index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Edn) -> &'v mut Edn {
        (**self).index_or_insert(v)
    }
}

// Prevent users from implementing the Index trait.
mod private {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for String {}
    impl<'a, T: ?Sized> Sealed for &'a T where T: Sealed {}
}
struct Type<'a>(&'a Edn);

impl<'a> fmt::Display for Type<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            Edn::Nil => formatter.write_str("null"),
            Edn::Bool(_) => formatter.write_str("boolean"),
            Edn::Int(_) => formatter.write_str("integer"),
            Edn::UInt(_) => formatter.write_str("integer"),
            Edn::Str(_) => formatter.write_str("string"),
            Edn::Vector(_) => formatter.write_str("vector"),
            Edn::Set(_) => formatter.write_str("set"),
            Edn::List(_) => formatter.write_str("list"),
            Edn::Map(_) => formatter.write_str("map"),
            Edn::Key(_) => formatter.write_str("key"),
            Edn::Symbol(_) => formatter.write_str("symbol"),
            Edn::Double(_) => formatter.write_str("double"),
            Edn::Rational(_) => formatter.write_str("rational"),
        }
    }
}

impl<I> ops::Index<I> for Edn
where
    I: Index,
{
    type Output = Edn;
    fn index(&self, index: I) -> &Edn {
        static NULL: Edn = Edn::Nil;
        index.index_into(self).unwrap_or(&NULL)
    }
}

impl<I> ops::IndexMut<I> for Edn
where
    I: Index,
{
    fn index_mut(&mut self, index: I) -> &mut Edn {
        index.index_or_insert(self)
    }
}