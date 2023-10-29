use crate::edn::{Edn, Error};
use std::collections::{BTreeMap, HashMap};
#[cfg(feature = "sets")]
use std::collections::{BTreeSet, HashSet};
use std::convert::TryFrom;
use std::str::FromStr;

pub mod parse;

#[cfg(feature = "sets")]
use ordered_float::OrderedFloat;

/// public trait to be used to `Deserialize` structs.
///
/// # Errors
///
/// Error will be like `EdnError::Deserialize("couldn't convert <value> into <type>")`
///
/// ```
/// use crate::edn_rs::{Edn, EdnError, Deserialize};
///
/// #[derive(Debug, PartialEq)]
/// struct Person {
///     name: String,
///     age: u64,
/// }
///
/// impl Deserialize for Person {
///     fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
///         Ok(Self {
///             name: edn_rs::from_edn(&edn[":name"])?,
///             age: edn_rs::from_edn(&edn[":age"])?,
///         })
///     }
/// }
///
/// let edn_str = "{:name \"rose\" :age 66 }";
/// let person: Person = edn_rs::from_str(edn_str).unwrap();
///
/// assert_eq!(
///     person,
///     Person {
///         name: "rose".to_string(),
///         age: 66,
///     }
/// );
///
/// println!("{:?}", person);
/// // Person { name: "rose", age: 66 }
///
/// let bad_edn_str = "{:name \"rose\" :age \"some text\" }";
/// let person: Result<Person, EdnError> = edn_rs::from_str(bad_edn_str);
///
/// assert_eq!(
///     person,
///     Err(EdnError::Deserialize(
///         "couldn't convert `\"some text\"` into `uint`".to_string()
///     ))
/// );
/// ```
#[allow(clippy::missing_errors_doc)]
pub trait Deserialize: Sized {
    fn deserialize(edn: &Edn) -> Result<Self, Error>;
}

fn build_deserialize_error(edn: &Edn, type_: &str) -> Error {
    Error::Deserialize(format!("couldn't convert `{edn}` into `{type_}`"))
}

impl Deserialize for () {
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        match edn {
            Edn::Nil => Ok(()),
            _ => Err(build_deserialize_error(edn, "unit")),
        }
    }
}

#[cfg(feature = "sets")]
impl Deserialize for OrderedFloat<f64> {
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        edn.to_float()
            .ok_or_else(|| build_deserialize_error(edn, "edn_rs::Double"))
            .map(std::convert::Into::into)
    }
}

impl Deserialize for f64 {
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        edn.to_float()
            .ok_or_else(|| build_deserialize_error(edn, "edn_rs::Double"))
            .map(std::convert::Into::into)
    }
}

macro_rules! impl_deserialize_int {
    ( $( $name:ty ),+ ) => {
        $(
            impl Deserialize for $name {
                fn deserialize(edn: &Edn) -> Result<Self, Error> {
                    let int = edn
                        .to_int()
                        .ok_or_else(|| build_deserialize_error(edn, "int"))?;
                    Ok(Self::try_from(int)?)
                }
            }
        )+
    };
}

impl_deserialize_int!(i8, i16, i32, i64);

macro_rules! impl_deserialize_uint {
    ( $( $name:ty ),+ ) => {
        $(
            impl Deserialize for $name {
                fn deserialize(edn: &Edn) -> Result<Self, Error> {
                    let uint = edn
                        .to_uint()
                        .ok_or_else(|| build_deserialize_error(edn, "uint"))?;
                    Ok(Self::try_from(uint)?)
                }
            }
        )+
    };
}

impl_deserialize_uint!(u8, u16, u32, u64);

impl Deserialize for bool {
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        edn.to_bool()
            .ok_or_else(|| build_deserialize_error(edn, "bool"))
    }
}

impl Deserialize for String {
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        match edn {
            Edn::Str(s) => {
                if s.starts_with('\"') {
                    Ok(s.replace('\"', ""))
                } else {
                    Ok(s.clone())
                }
            }
            e => Ok(e.to_string()),
        }
    }
}

impl Deserialize for char {
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        edn.to_char()
            .ok_or_else(|| build_deserialize_error(edn, "char"))
    }
}

impl<T> Deserialize for Vec<T>
where
    T: Deserialize,
{
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        match edn {
            Edn::Vector(_) => Ok(edn
                .iter_some()
                .ok_or_else(|| Error::Iter(format!("Could not create iter from {edn:?}")))?
                .map(|e| Deserialize::deserialize(e))
                .collect::<Result<Self, Error>>()?),
            Edn::List(_) => Ok(edn
                .iter_some()
                .ok_or_else(|| Error::Iter(format!("Could not create iter from {edn:?}")))?
                .map(|e| Deserialize::deserialize(e))
                .collect::<Result<Self, Error>>()?),
            #[cfg(feature = "sets")]
            Edn::Set(_) => Ok(edn
                .iter_some()
                .ok_or_else(|| Error::Iter(format!("Could not create iter from {edn:?}")))?
                .map(|e| Deserialize::deserialize(e))
                .collect::<Result<Self, Error>>()?),
            _ => Err(build_deserialize_error(edn, std::any::type_name::<Self>())),
        }
    }
}

#[allow(clippy::implicit_hasher)]
impl<T> Deserialize for HashMap<String, T>
where
    T: Deserialize,
{
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        match edn {
            Edn::Map(_) => edn
                .map_iter()
                .ok_or_else(|| Error::Iter(format!("Could not create iter from {edn:?}")))?
                .map(|(key, e)| {
                    Ok((
                        key.to_string(),
                        Deserialize::deserialize(e).map_err(|_| {
                            Error::Deserialize(format!(
                                "Cannot safely deserialize {:?} to {}",
                                edn, "HashMap"
                            ))
                        })?,
                    ))
                })
                .collect::<Result<Self, Error>>(),
            Edn::NamespacedMap(ns, _) => edn
                .map_iter()
                .ok_or_else(|| Error::Iter(format!("Could not create iter from {edn:?}")))?
                .map(|(key, e)| {
                    let deser_element = Deserialize::deserialize(e).map_err(|_| {
                        Error::Deserialize(format!(
                            "Cannot safely deserialize {:?} to {}",
                            edn, "HashMap"
                        ))
                    });

                    if ns.starts_with(':') {
                        Ok((ns.to_string() + "/" + key, deser_element?))
                    } else {
                        Ok((String::from(':') + ns + "/" + key, deser_element?))
                    }
                })
                .collect::<Result<Self, Error>>(),
            _ => Err(build_deserialize_error(edn, std::any::type_name::<Self>())),
        }
    }
}

impl<T> Deserialize for BTreeMap<String, T>
where
    T: Deserialize,
{
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        match edn {
            Edn::Map(_) => edn
                .map_iter()
                .ok_or_else(|| Error::Iter(format!("Could not create iter from {edn:?}")))?
                .map(|(key, e)| {
                    Ok((
                        key.to_string(),
                        Deserialize::deserialize(e).map_err(|_| {
                            Error::Deserialize(format!(
                                "Cannot safely deserialize {:?} to {}",
                                edn, "BTreeMap"
                            ))
                        })?,
                    ))
                })
                .collect::<Result<Self, Error>>(),
            Edn::NamespacedMap(ns, _) => edn
                .map_iter()
                .ok_or_else(|| Error::Iter(format!("Could not create iter from {edn:?}")))?
                .map(|(key, e)| {
                    let deser_element = Deserialize::deserialize(e).map_err(|_| {
                        Error::Deserialize(format!(
                            "Cannot safely deserialize {:?} to {}",
                            edn, "BTreeMap"
                        ))
                    });

                    if ns.starts_with(':') {
                        Ok((ns.to_string() + "/" + key, deser_element?))
                    } else {
                        Ok((String::from(':') + ns + "/" + key, deser_element?))
                    }
                })
                .collect::<Result<Self, Error>>(),
            _ => Err(build_deserialize_error(edn, std::any::type_name::<Self>())),
        }
    }
}

#[allow(clippy::implicit_hasher)]
#[cfg(feature = "sets")]
impl<T> Deserialize for HashSet<T>
where
    T: std::cmp::Eq + std::hash::Hash + Deserialize,
{
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        match edn {
            Edn::Set(_) => edn
                .set_iter()
                .ok_or_else(|| Error::Iter(format!("Could not create iter from {edn:?}")))?
                .map(|e| {
                    Deserialize::deserialize(e).map_err(|_| {
                        Error::Deserialize(format!(
                            "Cannot safely deserialize {:?} to {}",
                            edn, "HashSet"
                        ))
                    })
                })
                .collect::<Result<Self, Error>>(),
            _ => Err(build_deserialize_error(edn, std::any::type_name::<Self>())),
        }
    }
}

#[cfg(feature = "sets")]
impl<T> Deserialize for BTreeSet<T>
where
    T: std::cmp::Eq + std::hash::Hash + std::cmp::Ord + Deserialize,
{
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        match edn {
            Edn::Set(_) => edn
                .set_iter()
                .ok_or_else(|| Error::Iter(format!("Could not create iter from {edn:?}")))?
                .map(|e| {
                    Deserialize::deserialize(e).map_err(|_| {
                        Error::Deserialize(format!(
                            "Cannot safely deserialize {:?} to {}",
                            edn, "BTreeSet"
                        ))
                    })
                })
                .collect::<Result<Self, Error>>(),
            _ => Err(build_deserialize_error(edn, std::any::type_name::<Self>())),
        }
    }
}

impl<T> Deserialize for Option<T>
where
    T: Deserialize,
{
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        match edn {
            Edn::Nil => Ok(None),
            _ => Ok(Some(from_edn(edn)?)),
        }
    }
}

/// `from_str` deserializes an EDN String into type `T` that implements `Deserialize`. Response is `Result<T, EdnError>`
///
/// # Errors
///
/// Error will be like `EdnError::Deserialize("couldn't convert <value> into <type>")`
///
/// ```
/// use edn_rs::{Deserialize, Edn, EdnError};
///
/// #[derive(Debug, PartialEq)]
/// struct Person {
///     name: String,
///     age: u64,
/// }
///
/// impl Deserialize for Person {
///     fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
///         Ok(Self {
///             name: edn_rs::from_edn(&edn[":name"])?,
///             age: edn_rs::from_edn(&edn[":age"])?,
///         })
///     }
/// }
///
/// let edn_str = "  {:name \"rose\" :age 66  }  ";
/// let person: Person = edn_rs::from_str(edn_str).unwrap();
///
/// println!("{:?}", person);
/// // Person { name: "rose", age: 66 }
///
/// assert_eq!(
///     person,
///     Person {
///         name: "rose".to_string(),
///         age: 66,
///     }
/// );
///
/// let bad_edn_str = "{:name \"rose\" :age \"some text\" }";
/// let person: Result<Person, EdnError> = edn_rs::from_str(bad_edn_str);
///
/// assert_eq!(
///     person,
///     Err(EdnError::Deserialize(
///             "couldn't convert `\"some text\"` into `uint`".to_string()
///     ))
/// );
/// ```
pub fn from_str<T: Deserialize>(s: &str) -> Result<T, Error> {
    let edn = Edn::from_str(s)?;
    from_edn(&edn)
}

/// `from_edn` deserializes an EDN type into a `T` type that implements `Deserialize`. Response is `Result<T, EdnError>`
///
/// # Errors
///
/// Error will be like `EdnError::Deserialize("couldn't convert <value> into <type>")`
///
/// ```
/// use edn_rs::{map, Deserialize, Edn, EdnError, Map};
///
/// #[derive(Debug, PartialEq)]
/// struct Person {
///     name: String,
///     age: u64,
/// }
///
/// impl Deserialize for Person {
///     fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
///         Ok(Self {
///             name: edn_rs::from_edn(&edn[":name"])?,
///             age: edn_rs::from_edn(&edn[":age"])?,
///         })
///     }
/// }
///
/// let edn = Edn::Map(Map::new(map! {
///     ":name".to_string() => Edn::Str("rose".to_string()),
///     ":age".to_string() => Edn::UInt(66)
/// }));
/// let person: Person = edn_rs::from_edn(&edn).unwrap();
///
/// println!("{:?}", person);
/// // Person { name: "rose", age: 66 }
///
/// assert_eq!(
///     person,
///     Person {
///         name: "rose".to_string(),
///         age: 66,
///     }
/// );
///
/// let bad_edn = Edn::Map(Map::new(map! {
///     ":name".to_string() => Edn::Str("rose".to_string()),
///     ":age".to_string() => Edn::Str("some text".to_string())
/// }));
/// let person: Result<Person, EdnError> = edn_rs::from_edn(&bad_edn);
///
/// assert_eq!(
///     person,
///     Err(EdnError::Deserialize(
///         "couldn't convert `\"some text\"` into `uint`".to_string()
///     ))
/// );
/// ```
pub fn from_edn<T: Deserialize>(edn: &Edn) -> Result<T, Error> {
    T::deserialize(edn)
}

#[cfg(test)]
mod test {
    use super::*;
    #[cfg(feature = "sets")]
    use crate::edn::Set;
    use crate::edn::{List, Map, Vector};
    use crate::{hmap, hset, map, set};

    #[test]
    fn unit() {
        let nil = "nil";
        let unit: () = from_str(nil).unwrap();

        assert_eq!(unit, ())
    }

    #[test]
    #[cfg(feature = "sets")]
    fn deser_btreeset_with_error() {
        let edn = "#{\"a\", 5, \"b\"}";
        let err: Result<BTreeSet<u64>, Error> = from_str(edn);
        assert_eq!(
            err,
            Err(Error::Deserialize(
                "Cannot safely deserialize Set(Set({Str(\"a\"), Str(\"b\"), UInt(5)})) to BTreeSet"
                    .to_string()
            ))
        )
    }

    #[test]
    fn from_str_simple_vec() {
        let edn = "[1 \"2\" 3.3 :b true \\c]";

        assert_eq!(
            Edn::from_str(edn),
            Ok(Edn::Vector(Vector::new(vec![
                Edn::UInt(1),
                Edn::Str("2".to_string()),
                Edn::Double(3.3.into()),
                Edn::Key(":b".to_string()),
                Edn::Bool(true),
                Edn::Char('c')
            ])))
        );
    }

    #[test]
    fn from_str_list_with_vec() {
        let edn = "(1 \"2\" 3.3 :b [true \\c])";

        assert_eq!(
            Edn::from_str(edn),
            Ok(Edn::List(List::new(vec![
                Edn::UInt(1),
                Edn::Str("2".to_string()),
                Edn::Double(3.3.into()),
                Edn::Key(":b".to_string()),
                Edn::Vector(Vector::new(vec![Edn::Bool(true), Edn::Char('c')]))
            ])))
        );
    }

    #[test]
    #[cfg(feature = "sets")]
    fn from_str_list_with_set() {
        let edn = "(1 -10 \"2\" 3.3 :b #{true \\c})";

        assert_eq!(
            Edn::from_str(edn),
            Ok(Edn::List(List::new(vec![
                Edn::UInt(1),
                Edn::Int(-10),
                Edn::Str("2".to_string()),
                Edn::Double(3.3.into()),
                Edn::Key(":b".to_string()),
                Edn::Set(Set::new(set![Edn::Bool(true), Edn::Char('c')]))
            ])))
        );
    }

    #[test]
    fn from_str_simple_map() {
        let edn = "{:a \"2\" :b true :c nil}";

        assert_eq!(
            Edn::from_str(edn),
            Ok(Edn::Map(Map::new(
                map! {":a".to_string() => Edn::Str("2".to_string()),
                ":b".to_string() => Edn::Bool(true), ":c".to_string() => Edn::Nil}
            )))
        );
    }

    #[test]
    #[cfg(feature = "sets")]
    fn from_str_complex_map() {
        let edn = "{:a \"2\" :b [true false] :c #{:A {:a :b} nil}}";

        assert_eq!(
            Edn::from_str(edn),
            Ok(Edn::Map(Map::new(map! {
            ":a".to_string() =>Edn::Str("2".to_string()),
            ":b".to_string() => Edn::Vector(Vector::new(vec![Edn::Bool(true), Edn::Bool(false)])),
            ":c".to_string() => Edn::Set(Set::new(
                set!{
                    Edn::Map(Map::new(map!{":a".to_string() => Edn::Key(":b".to_string())})),
                    Edn::Key(":A".to_string()),
                    Edn::Nil}))})))
        );
    }

    #[test]
    fn from_str_wordy_str() {
        let edn = "[\"hello brave new world\"]";

        assert_eq!(
            Edn::from_str(edn).unwrap(),
            Edn::Vector(Vector::new(vec![Edn::Str(
                "hello brave new world".to_string()
            )]))
        )
    }

    #[test]
    fn namespaced_maps() {
        let edn = ":abc{ 0 :val 1 :value}";

        assert_eq!(
            Edn::from_str(edn).unwrap(),
            Edn::NamespacedMap(
                "abc".to_string(),
                Map::new(map! {
                    "0".to_string() => Edn::Key(":val".to_string()),
                    "1".to_string() => Edn::Key(":value".to_string())
                })
            )
        );
    }

    #[test]
    fn uuid() {
        let uuid = "#uuid \"af6d8699-f442-4dfd-8b26-37d80543186b\"";
        let edn: Edn = Edn::from_str(uuid).unwrap();

        assert_eq!(
            edn,
            Edn::Uuid("af6d8699-f442-4dfd-8b26-37d80543186b".to_string())
        )
    }

    #[test]
    fn deserialize_struct_with_vec() {
        #[derive(PartialEq, Debug)]
        struct Foo {
            bar: Vec<Option<u64>>,
        }
        impl Deserialize for Foo {
            fn deserialize(edn: &Edn) -> Result<Self, Error> {
                Ok(Foo {
                    bar: from_edn(&edn[":bar"])?,
                })
            }
        }
        let edn_foo = "{:bar [1 nil 3]}";
        let foo: Foo = from_str(edn_foo).unwrap();

        assert_eq!(
            foo,
            Foo {
                bar: vec![Some(1), None, Some(3)],
            }
        );
    }

    #[test]
    fn test_sym() {
        let edn: Edn = Edn::from_str("(a b c your-hair!-is+_parsed?)").unwrap();
        let expected = Edn::List(List::new(vec![
            Edn::Symbol("a".to_string()),
            Edn::Symbol("b".to_string()),
            Edn::Symbol("c".to_string()),
            Edn::Symbol("your-hair!-is+_parsed?".to_string()),
        ]));
        assert_eq!(edn, expected);
    }

    #[test]
    fn test_nft() {
        let t: Edn = Edn::from_str("tTEST").unwrap();
        let f: Edn = Edn::from_str("fTEST").unwrap();
        let n: Edn = Edn::from_str("nTEST").unwrap();
        let err: Edn = Edn::from_str("fTE").unwrap();

        assert_eq!(n, Edn::Symbol("nTEST".to_string()));
        assert_eq!(f, Edn::Symbol("fTEST".to_string()));
        assert_eq!(t, Edn::Symbol("tTEST".to_string()));
        assert_eq!(err, Edn::Symbol("fTE".to_string()));
    }

    #[test]
    #[cfg(feature = "sets")]
    fn test_more_sym() {
        let edn: Edn = Edn::from_str("(a \\b \"c\" 5 #{hello world})").unwrap();
        let expected = Edn::List(List::new(vec![
            Edn::Symbol("a".to_string()),
            Edn::Char('b'),
            Edn::Str("c".to_string()),
            Edn::UInt(5u64),
            Edn::Set(Set::new(
                set! { Edn::Symbol("hello".to_string()), Edn::Symbol("world".to_string()) },
            )),
        ]));
        assert_eq!(edn, expected);
    }

    #[test]
    fn namespaced_maps_navigation() {
        let edn_str = ":abc{ 0 :val 1 :value}";

        let edn = Edn::from_str(edn_str).unwrap();

        assert_eq!(edn[0], Edn::Key(":val".to_string()));
        assert_eq!(edn["0"], Edn::Key(":val".to_string()));
        assert_eq!(edn[1], Edn::Key(":value".to_string()));
        assert_eq!(edn["1"], Edn::Key(":value".to_string()));
    }

    #[test]
    fn deser_namespaced_btreemap() {
        let ns_map = Edn::NamespacedMap(
            "abc".to_string(),
            Map::new(map! {
                "0".to_string() => Edn::Key(":val".to_string()),
                "1".to_string() => Edn::Key(":value".to_string())
            }),
        );
        let expected = map! {
            ":abc/0".to_string() => ":val".to_string(),
            ":abc/1".to_string() => ":value".to_string()
        };
        let map: std::collections::BTreeMap<String, String> = from_edn(&ns_map).unwrap();
        assert_eq!(map, expected);
    }

    #[test]
    fn deser_namespaced_hashmap() {
        let ns_map = Edn::NamespacedMap(
            "abc".to_string(),
            Map::new(map! {
                "0".to_string() => Edn::Key(":val".to_string()),
                "1".to_string() => Edn::Key(":value".to_string())
            }),
        );
        let expected = hmap! {
            ":abc/0".to_string() => ":val".to_string(),
            ":abc/1".to_string() => ":value".to_string()
        };
        let map: std::collections::HashMap<String, String> = from_edn(&ns_map).unwrap();
        assert_eq!(map, expected);
    }

    #[test]
    fn deser_btreemap() {
        let ns_map = Edn::Map(Map::new(map! {
            ":a".to_string() => Edn::Vector(Vector::new(vec![Edn::Key(":val".to_string())])),
            ":b".to_string() => Edn::Vector(Vector::new(vec![Edn::Key(":value".to_string())]))
        }));
        let expected = map! {
            ":a".to_string() => vec![":val".to_string()],
            ":b".to_string() => vec![":value".to_string()]
        };
        let map: std::collections::BTreeMap<String, Vec<String>> = from_edn(&ns_map).unwrap();
        assert_eq!(map, expected);
    }

    #[test]
    fn deser_hashmap() {
        let ns_map = Edn::Map(Map::new(map! {
            ":a".to_string() => Edn::Bool(true),
            ":b".to_string() => Edn::Bool(false)
        }));
        let expected = hmap! {
            ":a".to_string() => true,
            ":b".to_string() => false
        };
        let map: std::collections::HashMap<String, bool> = from_edn(&ns_map).unwrap();
        assert_eq!(map, expected);
    }

    #[test]
    #[cfg(feature = "sets")]
    fn deser_btreeset() {
        let set = Edn::Set(Set::new(set! {
            Edn::UInt(4),
            Edn::UInt(5),
            Edn::UInt(6)
        }));
        let expected = set! {
            4,
            5,
            6,
        };
        let deser_set: std::collections::BTreeSet<u64> = from_edn(&set).unwrap();
        assert_eq!(deser_set, expected);
    }

    #[cfg(feature = "sets")]
    #[test]
    fn deser_hashset() {
        use ordered_float::OrderedFloat;

        let set = Edn::Set(Set::new(set! {
            Edn::Double(4.6.into()),
            Edn::Double(5.6.into()),
            Edn::Double(6.6.into())
        }));
        let expected = hset! {
            OrderedFloat(4.6f64),
            OrderedFloat(5.6f64),
            OrderedFloat(6.6f64),
        };
        let deser_set: std::collections::HashSet<OrderedFloat<f64>> = from_edn(&set).unwrap();
        assert_eq!(deser_set, expected);
    }

    #[test]
    fn weird_input() {
        let edn = "{:a]";

        assert_eq!(
            Edn::from_str(edn),
            Err(Error::ParseEdn(
                "Could not identify symbol index".to_string()
            ))
        );
    }

    #[test]
    fn string_with_empty_set() {
        assert_eq!("\"#{}\"", format!("{}", Edn::from_str("\"#{}\"").unwrap()));
    }
}
