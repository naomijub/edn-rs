use crate::edn::Error;
use crate::edn::{Edn, List, Map, Set, Vector};
use std::str::FromStr;
/// public trait to be used to `Deserialize` structs
///
/// Example:
/// ```
/// use crate::edn_rs::{Edn, EdnError, Deserialize};
///
/// #[derive(Debug, PartialEq)]
/// struct Person {
///     name: String,
///     age: usize,
/// }
///
/// impl Deserialize for Person {
///     fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
///         Ok(Self {
///             name: Deserialize::deserialize(&edn[":name"])?,
///             age: Deserialize::deserialize(&edn[":age"])?,
///         })
///     }
/// }
///
/// let edn_str = "{:name \"rose\" :age 66}";
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
/// let bad_edn_str = "{:name \"rose\" :age \"some text\"}";
/// let person: Result<Person, EdnError> = edn_rs::from_str(bad_edn_str);
///
/// assert_eq!(
///     person,
///     Err(EdnError::Deserialize(
///         "couldn't convert `some text` into `uint`".to_string()
///     ))
/// );
/// ```
pub trait Deserialize: Sized {
    fn deserialize(edn: &Edn) -> Result<Self, Error>;
}

fn build_deserialize_error(edn: Edn, type_: &str) -> Error {
    Error::Deserialize(format!("couldn't convert `{}` into `{}`", edn, type_))
}

macro_rules! impl_deserialize_float {
    ( $( $name:ty ),+ ) => {
        $(
            impl Deserialize for $name
            {
                fn deserialize(edn: &Edn) -> Result<Self, Error> {
                    edn
                        .to_float()
                        .ok_or_else(|| build_deserialize_error(edn.clone(), "float"))
                        .map(|u| u as $name)
                }
            }
        )+
    };
}

impl_deserialize_float!(f32, f64);

macro_rules! impl_deserialize_int {
    ( $( $name:ty ),+ ) => {
        $(
            impl Deserialize for $name
            {
                fn deserialize(edn: &Edn) -> Result<Self, Error> {
                    edn
                        .to_int()
                        .ok_or_else(|| build_deserialize_error(edn.clone(), "int"))
                        .map(|u| u as $name)
                }
            }
        )+
    };
}

impl_deserialize_int!(isize, i8, i16, i32, i64);

macro_rules! impl_deserialize_uint {
    ( $( $name:ty ),+ ) => {
        $(
            impl Deserialize for $name
            {
                fn deserialize(edn: &Edn) -> Result<Self, Error> {
                    edn
                        .to_uint()
                        .ok_or_else(|| build_deserialize_error(edn.clone(), "uint"))
                        .map(|u| u as $name)
                }
            }
        )+
    };
}

impl_deserialize_uint!(usize, u8, u16, u32, u64);

impl Deserialize for bool {
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        edn.to_bool()
            .ok_or_else(|| build_deserialize_error(edn.clone(), "bool"))
    }
}

impl Deserialize for String {
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        Ok(edn.to_string())
    }
}

impl Deserialize for char {
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        edn.to_char()
            .ok_or_else(|| build_deserialize_error(edn.clone(), "char"))
    }
}

impl<T> Deserialize for Vec<T>
where
    T: Deserialize,
{
    fn deserialize(edn: &Edn) -> Result<Self, Error> {
        match edn {
            Edn::Vector(_) => Ok(edn
                .iter()
                .unwrap()
                .map(|e| Deserialize::deserialize(e))
                .collect::<Result<Vec<T>, Error>>()?),
            Edn::List(_) => Ok(edn
                .iter()
                .unwrap()
                .map(|e| Deserialize::deserialize(e))
                .collect::<Result<Vec<T>, Error>>()?),
            Edn::Set(_) => Ok(edn
                .iter()
                .unwrap()
                .map(|e| Deserialize::deserialize(e))
                .collect::<Result<Vec<T>, Error>>()?),
            _ => Err(build_deserialize_error(
                edn.clone(),
                std::any::type_name::<Vec<T>>(),
            )),
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
            _ => Ok(Some(Deserialize::deserialize(&edn)?)),
        }
    }
}

/// `from_str` parses a EDN String into something that implements `TryFrom<Edn, Error = EdnError>`
pub fn from_str<T: Deserialize>(s: &str) -> Result<T, Error> {
    let edn = Edn::from_str(s)?;
    T::deserialize(&edn)
}

pub(crate) fn tokenize(edn: &str) -> Vec<String> {
    edn.replace("}", " } ")
        .replace("#{", " @ ")
        .replace("{", " { ")
        .replace("[", " [ ")
        .replace("]", " ] ")
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("\n", " ")
        .replace(",", " ")
        .replace("\"", " \" ")
        .replace("#inst", "")
        .trim()
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| String::from(s))
        .collect()
}

pub(crate) fn parse<'a>(tokens: &'a [String]) -> Result<(Edn, &'a [String]), Error> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(Error::from("Could not understand tokens".to_string()))?;

    match &token[..] {
        "[" => read_vec(rest),
        "]" => Err("Unexpected Token `]`".to_string().into()),
        "(" => read_list(rest),
        ")" => Err("Unexpected Token `)`".to_string().into()),
        "@" => read_set(rest),
        "{" => read_map(rest),
        "}" => Err("Unexpected Token `}`".to_string().into()),
        "\"" => read_str(rest),
        _ => Ok((Edn::parse_word(token.to_string()), rest)),
    }
}

fn read_vec<'a>(tokens: &'a [String]) -> Result<(Edn, &'a [String]), Error> {
    let mut res: Vec<Edn> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or("Could not find closing `]` for Vector".to_string())?;
        if next_token == "]" {
            return Ok((Edn::Vector(Vector::new(res)), rest));
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn read_list<'a>(tokens: &'a [String]) -> Result<(Edn, &'a [String]), Error> {
    let mut res: Vec<Edn> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or("Could not find closing `)` for List".to_string())?;
        if next_token == ")" {
            return Ok((Edn::List(List::new(res)), rest));
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn read_set<'a>(tokens: &'a [String]) -> Result<(Edn, &'a [String]), Error> {
    use std::collections::BTreeSet;
    let mut res: BTreeSet<Edn> = BTreeSet::new();
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or("Could not find closing `}` for Set".to_string())?;
        if next_token == "}" {
            return Ok((Edn::Set(Set::new(res)), rest));
        }
        let (exp, new_xs) = parse(&xs)?;
        res.insert(exp);
        xs = new_xs;
    }
}

fn read_map<'a>(tokens: &'a [String]) -> Result<(Edn, &'a [String]), Error> {
    use std::collections::BTreeMap;
    let mut res = BTreeMap::new();
    let mut xs = tokens;
    loop {
        let (first_token, rest) = xs
            .split_first()
            .ok_or("Could not find closing `}` for Map".to_string())?;
        if first_token == "}" {
            return Ok((Edn::Map(Map::new(res)), rest));
        }

        let (exp1, new_xs1) = parse(&xs)?;
        let (exp2, new_xs2) = parse(&new_xs1)?;

        res.insert(exp1.to_string(), exp2);
        xs = new_xs2;
    }
}

fn read_str<'a>(tokens: &'a [String]) -> Result<(Edn, &'a [String]), Error> {
    let mut res = String::new();
    let mut xs = tokens;
    let mut counter = 0;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or("Could not find closing `\"` for Str".to_string())?;
        if next_token == "\"" {
            return Ok((Edn::Str(res), rest));
        }
        let (exp, new_xs) = xs
            .split_first()
            .ok_or("Could not find closing `\"` for Str".to_string())?;
        if counter != 0 {
            res.push_str(" ");
        }
        res.push_str(exp);
        xs = new_xs;
        counter += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::edn::{Map, Set};
    use crate::{map, set};

    #[test]
    fn tokenize_edn() {
        let edn = "[1 \"2\" 3.3 :b [true \\c]]";

        assert_eq!(
            tokenize(edn),
            vec![
                "[".to_string(),
                "1".to_string(),
                "\"".to_string(),
                "2".to_string(),
                "\"".to_string(),
                "3.3".to_string(),
                ":b".to_string(),
                "[".to_string(),
                "true".to_string(),
                "\\c".to_string(),
                "]".to_string(),
                "]".to_string()
            ]
        );
    }

    #[test]
    fn parse_simple_vec() {
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
    fn parse_list_with_vec() {
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
    fn parse_list_with_set() {
        let edn = "(1 \"2\" 3.3 :b #{true \\c})";

        assert_eq!(
            Edn::from_str(edn),
            Ok(Edn::List(List::new(vec![
                Edn::UInt(1),
                Edn::Str("2".to_string()),
                Edn::Double(3.3.into()),
                Edn::Key(":b".to_string()),
                Edn::Set(Set::new(set![Edn::Bool(true), Edn::Char('c')]))
            ])))
        );
    }

    #[test]
    fn parse_simple_map() {
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
    fn parse_complex_map() {
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
    fn parse_wordy_str() {
        let edn = "[\"hello brave new world\"]";

        assert_eq!(
            Edn::from_str(edn).unwrap(),
            Edn::Vector(Vector::new(vec![Edn::Str(
                "hello brave new world".to_string()
            )]))
        )
    }
}
