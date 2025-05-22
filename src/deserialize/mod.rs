use alloc::collections::BTreeMap;
use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::any;
use core::convert::{Into, TryFrom};
use core::str::FromStr;
#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "std")]
use std::collections::HashSet;

use crate::edn::{Edn, Error};

pub mod parse;

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
    /// Deserializes an EDN type into a `T` type that implements `Deserialize`.
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

impl Deserialize for OrderedFloat<f64> {
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        edn.to_float()
            .ok_or_else(|| build_deserialize_error(edn, "edn_rs::Double"))
            .map(Into::into)
    }
}

impl Deserialize for f64 {
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        edn.to_float()
            .ok_or_else(|| build_deserialize_error(edn, "edn_rs::Double"))
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

impl_deserialize_int!(i8, i16, i32, i64, isize);

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

impl_deserialize_uint!(u8, u16, u32, u64, usize);

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
            Edn::Set(_) => Ok(edn
                .iter_some()
                .ok_or_else(|| Error::Iter(format!("Could not create iter from {edn:?}")))?
                .map(|e| Deserialize::deserialize(e))
                .collect::<Result<Self, Error>>()?),
            _ => Err(build_deserialize_error(edn, any::type_name::<Self>())),
        }
    }
}

#[cfg(feature = "std")]
impl<T, H> Deserialize for HashMap<String, T, H>
where
    T: Deserialize,
    H: std::hash::BuildHasher + std::default::Default,
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
            _ => Err(build_deserialize_error(edn, any::type_name::<Self>())),
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
            _ => Err(build_deserialize_error(edn, any::type_name::<Self>())),
        }
    }
}

#[cfg(feature = "std")]
impl<T, H> Deserialize for HashSet<T, H>
where
    T: std::cmp::Eq + std::hash::Hash + Deserialize,
    H: std::hash::BuildHasher + std::default::Default,
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
            _ => Err(build_deserialize_error(edn, any::type_name::<Self>())),
        }
    }
}

impl<T> Deserialize for BTreeSet<T>
where
    T: core::cmp::Eq + core::hash::Hash + core::cmp::Ord + Deserialize,
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
            _ => Err(build_deserialize_error(edn, any::type_name::<Self>())),
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
