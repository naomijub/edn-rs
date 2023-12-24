use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use core::convert::TryFrom;
use core::{fmt, ops};

use crate::edn::{Edn, Map};

/// This is a Copy of [`Serde_json::index`](https://docs.serde.rs/src/serde_json/value/index.rs.html)
pub trait Index: private::Sealed {
    fn index_into<'v>(&self, v: &'v Edn) -> Option<&'v Edn>;

    fn index_into_mut<'v>(&self, v: &'v mut Edn) -> Option<&'v mut Edn>;

    fn index_or_insert<'v>(&self, v: &'v mut Edn) -> &'v mut Edn;
}

impl Index for usize {
    fn index_into<'v>(&self, v: &'v Edn) -> Option<&'v Edn> {
        return match *v {
            Edn::Vector(ref vec) => vec.0.get(*self),
            Edn::List(ref vec) => vec.0.get(*self),
            Edn::Map(ref map) => map.0.get(&self.to_string()),
            _ => None,
        };
    }
    fn index_into_mut<'v>(&self, v: &'v mut Edn) -> Option<&'v mut Edn> {
        return match *v {
            Edn::Vector(ref mut vec) => vec.0.get_mut(*self),
            Edn::List(ref mut vec) => vec.0.get_mut(*self),
            Edn::Map(ref mut map) => map.0.get_mut(&self.to_string()),
            _ => None,
        };
    }
    fn index_or_insert<'v>(&self, v: &'v mut Edn) -> &'v mut Edn {
        match *v {
            Edn::Vector(ref mut vec) => {
                let len = vec.0.len();
                vec.0.get_mut(*self).unwrap_or_else(|| {
                    panic!("cannot access index {self} of EDN array of length {len}")
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
        if *v == Edn::Nil {
            *v = Edn::Map(Map::new(alloc::collections::BTreeMap::new()));
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

impl Index for Edn {
    fn index_into<'v>(&self, v: &'v Edn) -> Option<&'v Edn> {
        let key = self.to_string();
        let index = self.to_uint();

        match (v, index) {
            (Self::Map(ref map), _) => map.0.get(&key),
            (Self::List(_) | Self::Vector(_), Some(idx)) => {
                // A panic is expected behavior when trying to index beyond usize
                let idx = usize::try_from(idx).unwrap();
                idx.index_into(v)
            }
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, _: &'v mut Edn) -> Option<&'v mut Edn> {
        panic!("index_into_mut not implemented for edn")
    }
    fn index_or_insert<'v>(&self, _: &'v mut Edn) -> &'v mut Edn {
        panic!("index_or_insert not implemented for edn")
    }
}

impl<'a, T> Index for &'a T
where
    T: ?Sized + Index,
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
    use alloc::string::String;

    use crate::Edn;

    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for String {}
    impl Sealed for Edn {}
    impl<'a, T> Sealed for &'a T where T: ?Sized + Sealed {}
}
struct Type<'a>(&'a Edn);

impl<'a> fmt::Display for Type<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self.0 {
            Edn::Empty => formatter.write_str("empty"),
            Edn::Nil => formatter.write_str("null"),
            Edn::Bool(_) => formatter.write_str("boolean"),
            Edn::Int(_) | Edn::UInt(_) => formatter.write_str("integer"),
            Edn::Str(_) => formatter.write_str("string"),
            Edn::Vector(_) => formatter.write_str("vector"),
            #[cfg(feature = "sets")]
            Edn::Set(_) => formatter.write_str("set"),
            Edn::List(_) => formatter.write_str("list"),
            Edn::Map(_) => formatter.write_str("map"),
            Edn::Key(_) => formatter.write_str("key"),
            Edn::Char(_) => formatter.write_str("char"),
            Edn::Symbol(_) => formatter.write_str("symbol"),
            Edn::Double(_) => formatter.write_str("double"),
            Edn::Rational(_) => formatter.write_str("rational"),
            Edn::Tagged(_, _) => formatter.write_str("tagged-element"),
        }
    }
}

impl<I> ops::Index<I> for Edn
where
    I: Index,
{
    type Output = Self;
    fn index(&self, index: I) -> &Self {
        static NULL: Edn = Edn::Nil;
        index.index_into(self).unwrap_or(&NULL)
    }
}

impl<I> ops::IndexMut<I> for Edn
where
    I: Index,
{
    fn index_mut(&mut self, index: I) -> &mut Self {
        index.index_or_insert(self)
    }
}
