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
                .ok_or(Error::Iter(format!("Could not create iter from {:?}", edn)))?
                .map(|e| Deserialize::deserialize(e))
                .collect::<Result<Vec<T>, Error>>()?),
            Edn::List(_) => Ok(edn
                .iter()
                .ok_or(Error::Iter(format!("Could not create iter from {:?}", edn)))?
                .map(|e| Deserialize::deserialize(e))
                .collect::<Result<Vec<T>, Error>>()?),
            Edn::Set(_) => Ok(edn
                .iter()
                .ok_or(Error::Iter(format!("Could not create iter from {:?}", edn)))?
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

/// `from_str` deserializes an EDN String into type `T` that implements `Deserialize`. Response is `Result<T, EdnError>`
pub fn from_str<T: Deserialize>(s: &str) -> Result<T, Error> {
    let edn = Edn::from_str(&s.trim())?;
    T::deserialize(&edn)
}

pub(crate) fn tokenize(edn: &str) -> std::str::Chars {
    edn.chars()
}

pub(crate) fn parse(c: Option<char>, chars: &mut std::str::Chars) -> Result<Edn, Error> {
    Ok(match c {
        Some('[') => read_vec(chars)?,
        Some('(') => read_list(chars)?,
        Some('#') => read_set(chars)?,
        Some('{') => read_map(chars)?,
        edn => parse_edn(edn, chars)?,
    })
}

pub(crate) fn parse_edn(c: Option<char>, chars: &mut std::str::Chars) -> Result<Edn, Error> {
    match c {
        Some('\"') => Ok(read_str(chars)),
        Some(':') => Ok(read_key(chars)),
        Some(n) if n.is_numeric() => Ok(read_number(n, chars)?),
        Some('-') => Ok(read_number('-', chars)?),
        Some('\\') => Ok(read_char(chars)?),
        Some(b) if b == 't' || b == 'f' || b == 'n' => Ok(read_bool_or_nil(b, chars)?),
        a => Err(Error::ParseEdn(format!(
            "{} could not be parsed",
            a.unwrap().to_string()
        ))),
    }
}

fn read_key(chars: &mut std::str::Chars) -> Edn {
    let mut key = String::from(":");
    let key_chars = chars
        .take_while(|c| !c.is_whitespace() && c != &')' && c != &']' && c != &'}')
        .collect::<String>();
    key.push_str(&key_chars);
    Edn::Key(key)
}

fn read_str(chars: &mut std::str::Chars) -> Edn {
    let string = chars.take_while(|c| c != &'\"').collect::<String>();
    Edn::Str(string)
}

fn read_number(n: char, chars: &mut std::str::Chars) -> Result<Edn, Error> {
    let mut number = String::new();
    let string = chars
        .take_while(|c| c.is_numeric() || c == &'.' || c == &'/')
        .collect::<String>();
    number.push(n);
    number.push_str(&string);

    match number {
        n if n.parse::<usize>().is_ok() => Ok(Edn::UInt(n.parse::<usize>()?)),
        n if n.parse::<isize>().is_ok() => Ok(Edn::Int(n.parse::<isize>()?)),
        n if n.parse::<f64>().is_ok() => Ok(Edn::Double(n.parse::<f64>()?.into())),
        n if n.contains("/") && n.split("/").all(|d| d.parse::<f64>().is_ok()) => {
            Ok(Edn::Rational(n))
        }
        _ => Err(Error::ParseEdn(format!("{} could not be parsed", number))),
    }
}

fn read_char(chars: &mut std::str::Chars) -> Result<Edn, Error> {
    let c = chars.next();
    c.ok_or(format!("{:?} could not be parsed", c))
        .map(|c| Edn::Char(c))
        .map_err(|e| Error::ParseEdn(e))
}

fn read_bool_or_nil(c: char, chars: &mut std::str::Chars) -> Result<Edn, Error> {
    match c {
        't' => {
            let mut string = String::new();
            let t = chars
                .take_while(|e| e == &'r' || e == &'u' || e == &'e')
                .collect::<String>();
            string.push(c);
            string.push_str(&t);
            Ok(Edn::Bool(string.parse::<bool>()?))
        }
        'f' => {
            let mut string = String::new();
            let f = chars
                .take_while(|e| e == &'a' || e == &'l' || e == &'s' || e == &'e')
                .collect::<String>();
            string.push(c);
            string.push_str(&f);
            Ok(Edn::Bool(string.parse::<bool>()?))
        }
        'n' => {
            let mut string = String::new();
            let n = chars
                .take_while(|e| e == &'i' || e == &'l')
                .collect::<String>();
            string.push(c);
            string.push_str(&n);
            match &string[..] {
                "nil" => Ok(Edn::Nil),
                _ => Err(Error::ParseEdn(format!("{} cound not be parsed", string))),
            }
        }
        _ => Err(Error::ParseEdn(
            "Nullable boolean cound not be parsed".to_string(),
        )),
    }
}

fn read_vec(chars: &mut std::str::Chars) -> Result<Edn, Error> {
    let mut res: Vec<Edn> = vec![];
    loop {
        match chars.next() {
            Some(']') => return Ok(Edn::Vector(Vector::new(res))),
            Some(c) if !c.is_whitespace() && c != ',' => {
                res.push(parse(Some(c), chars)?);
            }
            Some(c) if c.is_whitespace() || c == ',' => (),
            err => return Err(Error::ParseEdn(format!("{:?} could not be parsed", err))),
        }
    }
}

fn read_list(chars: &mut std::str::Chars) -> Result<Edn, Error> {
    let mut res: Vec<Edn> = vec![];
    loop {
        match chars.next() {
            Some(')') => return Ok(Edn::List(List::new(res))),
            Some(c) if !c.is_whitespace() && c != ',' => {
                res.push(parse(Some(c), chars)?);
            }
            Some(c) if c.is_whitespace() || c == ',' => (),
            err => return Err(Error::ParseEdn(format!("{:?} could not be parsed", err))),
        }
    }
}

fn read_set(chars: &mut std::str::Chars) -> Result<Edn, Error> {
    use std::collections::BTreeSet;
    let mut res: BTreeSet<Edn> = BTreeSet::new();
    loop {
        match chars.next() {
            Some('}') => return Ok(Edn::Set(Set::new(res))),
            Some(c) if !c.is_whitespace() && c != ',' && c != '{' => {
                res.insert(parse(Some(c), chars)?);
            }
            Some(c) if c.is_whitespace() || c == ',' || c == '{' => (),
            err => return Err(Error::ParseEdn(format!("{:?} could not be parsed", err))),
        }
    }
}

fn read_map(chars: &mut std::str::Chars) -> Result<Edn, Error> {
    use std::collections::BTreeMap;
    let mut res: BTreeMap<String, Edn> = BTreeMap::new();
    loop {
        match chars.next() {
            Some('}') => return Ok(Edn::Map(Map::new(res))),
            Some(c) if !c.is_whitespace() && c != ',' => {
                res.insert(
                    parse(Some(c), chars)?.to_string(),
                    parse(chars.next(), chars)?,
                );
            }
            Some(c) if c.is_whitespace() || c == ',' => (),
            err => return Err(Error::ParseEdn(format!("{:?} could not be parsed", err))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::edn::{Double, Map, Set};
    use crate::{map, set};

    #[test]
    fn parse_keyword() {
        let mut key = ":keyword".chars();

        assert_eq!(
            parse_edn(key.next(), &mut key).unwrap(),
            Edn::Key(":keyword".to_string())
        )
    }

    #[test]
    fn parse_str() {
        let mut string = "\"hello world, from      RUST\"".chars();

        assert_eq!(
            parse_edn(string.next(), &mut string).unwrap(),
            Edn::Str("hello world, from      RUST".to_string())
        )
    }

    #[test]
    fn parse_number() {
        let mut uint = "143".chars();
        let mut int = "-435143".chars();
        let mut f = "-43.5143".chars();
        let mut r = "43/5143".chars();
        assert_eq!(parse_edn(uint.next(), &mut uint).unwrap(), Edn::UInt(143));
        assert_eq!(parse_edn(int.next(), &mut int).unwrap(), Edn::Int(-435143));
        assert_eq!(
            parse_edn(f.next(), &mut f).unwrap(),
            Edn::Double(Double::from(-43.5143))
        );
        assert_eq!(
            parse_edn(r.next(), &mut r).unwrap(),
            Edn::Rational("43/5143".to_string())
        );
    }

    #[test]
    fn parse_char() {
        let mut c = "\\k".chars();

        assert_eq!(parse_edn(c.next(), &mut c).unwrap(), Edn::Char('k'))
    }

    #[test]
    fn parse_bool_or_nil() {
        let mut t = "true".chars();
        let mut f = "false".chars();
        let mut n = "nil".chars();
        assert_eq!(parse_edn(t.next(), &mut t).unwrap(), Edn::Bool(true));
        assert_eq!(parse_edn(f.next(), &mut f).unwrap(), Edn::Bool(false));
        assert_eq!(parse_edn(n.next(), &mut n).unwrap(), Edn::Nil);
    }

    #[test]
    fn parse_simple_vec() {
        let mut edn = "[11 \"2\" 3.3 :b true \\c]".chars();

        assert_eq!(
            parse(edn.next(), &mut edn).unwrap(),
            Edn::Vector(Vector::new(vec![
                Edn::UInt(11),
                Edn::Str("2".to_string()),
                Edn::Double(3.3.into()),
                Edn::Key(":b".to_string()),
                Edn::Bool(true),
                Edn::Char('c')
            ]))
        );
    }

    #[test]
    fn parse_list() {
        let mut edn = "(1 \"2\" 3.3 :b )".chars();

        assert_eq!(
            parse(edn.next(), &mut edn).unwrap(),
            Edn::List(List::new(vec![
                Edn::UInt(1),
                Edn::Str("2".to_string()),
                Edn::Double(3.3.into()),
                Edn::Key(":b".to_string()),
            ]))
        );
    }

    #[test]
    fn parse_set() {
        let mut edn = "#{true \\c 3 }".chars();

        assert_eq!(
            parse(edn.next(), &mut edn).unwrap(),
            Edn::Set(Set::new(set![
                Edn::Bool(true),
                Edn::Char('c'),
                Edn::UInt(3)
            ]))
        )
    }

    #[test]
    fn parse_complex() {
        let mut edn = "[:b ( 5 \\c #{true \\c 3 } ) ]".chars();

        assert_eq!(
            parse(edn.next(), &mut edn).unwrap(),
            Edn::Vector(Vector::new(vec![
                Edn::Key(":b".to_string()),
                Edn::List(List::new(vec![
                    Edn::UInt(5),
                    Edn::Char('c'),
                    Edn::Set(Set::new(set![
                        Edn::Bool(true),
                        Edn::Char('c'),
                        Edn::UInt(3)
                    ]))
                ]))
            ]))
        )
    }

    #[test]
    fn parse_simple_map() {
        let mut edn = "{:a \"2\" :b false :c nil }".chars();

        assert_eq!(
            parse(edn.next(), &mut edn).unwrap(),
            Edn::Map(Map::new(
                map! {":a".to_string() => Edn::Str("2".to_string()),
                ":b".to_string() => Edn::Bool(false), ":c".to_string() => Edn::Nil}
            ))
        );
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
    fn from_str_list_with_set() {
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

    // #[test]
    // fn from_str_complex_map() {
    //     let edn = "{:a \"2\" :b [true false] :c #{:A {:a :b} nil}}";

    //     assert_eq!(
    //         Edn::from_str(edn),
    //         Ok(Edn::Map(Map::new(map! {
    //         ":a".to_string() =>Edn::Str("2".to_string()),
    //         ":b".to_string() => Edn::Vector(Vector::new(vec![Edn::Bool(true), Edn::Bool(false)])),
    //         ":c".to_string() => Edn::Set(Set::new(
    //             set!{
    //                 Edn::Map(Map::new(map!{":a".to_string() => Edn::Key(":b".to_string())})),
    //                 Edn::Key(":A".to_string()),
    //                 Edn::Nil}))})))
    //     );
    // }

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
}
