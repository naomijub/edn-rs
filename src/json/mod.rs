use alloc::collections::BTreeMap;
#[cfg(feature = "sets")]
use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::edn::{rational_to_double, Edn};

#[allow(clippy::module_name_repetitions)]
pub fn display_as_json(edn: &Edn) -> String {
    match edn {
        Edn::Vector(v) => vec_to_json(&v.clone().to_vec()),
        #[cfg(feature = "sets")]
        Edn::Set(s) => set_to_json_vec(&s.clone().to_set()),
        Edn::Map(map) => map_to_json(&map.clone().to_map()),
        Edn::List(l) => vec_to_json(&l.clone().to_vec()),
        Edn::Key(key) => format!("{:?}", kebab_to_camel(key)),
        Edn::Symbol(s) | Edn::Str(s) => format!("{s:?}"),
        Edn::Int(n) => format!("{n}"),
        Edn::UInt(n) => format!("{n}"),
        Edn::Double(n) => {
            // Rust formats an f64 with a value of 2^5 as "32".
            // We do this to ensure all precision is printed if available, but still adds a decimal point for json.
            let mut s = format!("{n}");
            if !s.contains('.') {
                s.push_str(".0");
            }
            s
        }
        Edn::Rational(r) => format!("{}", rational_to_double(*r)),
        Edn::Char(c) => format!("'{c}'"),
        Edn::Bool(b) => format!("{b}"),
        Edn::Nil => String::from("null"),
        Edn::Empty => String::new(),
        Edn::Tagged(tag, content) => format!("{{ \"{}\": {}}}", tag, display_as_json(content)),
    }
}

fn kebab_to_camel(key: &str) -> String {
    let keywrod = key
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if c == ':' {
                ' '
            } else if i > 0 && i != 1 {
                if &key[i - 1..i] == ":" || &key[i - 1..i] == "-" || &key[i - 1..i] == "." {
                    c.to_uppercase()
                        .collect::<String>()
                        .chars()
                        .take(1)
                        .next()
                        .unwrap()
                } else {
                    c
                }
            } else {
                c
            }
        })
        .collect::<String>();

    keywrod.trim().replace(['-', '.'], "")
}

fn vec_to_json(vec: &[Edn]) -> String {
    let vec_str = vec
        .iter()
        .map(display_as_json)
        .collect::<Vec<String>>()
        .join(", ");
    let mut s = String::from("[");
    s.push_str(&vec_str);
    s.push(']');
    s
}

#[cfg(feature = "sets")]
fn set_to_json_vec(set: &BTreeSet<Edn>) -> String {
    let set_str = set
        .iter()
        .map(display_as_json)
        .collect::<Vec<String>>()
        .join(", ");
    let mut s = String::from("[");
    s.push_str(&set_str);
    s.push(']');
    s
}

fn map_to_json(map: &BTreeMap<String, Edn>) -> String {
    let map_str = map
        .iter()
        .map(|(k, e)| {
            let key = if k.starts_with(':') {
                kebab_to_camel(k)
            } else {
                k.to_string()
            };
            let edn = display_as_json(e);

            format!("{key:?}: {edn}")
        })
        .collect::<Vec<String>>()
        .join(", ");
    let mut s = String::from("{");
    s.push_str(&map_str);
    s.push('}');
    s
}

#[cfg(test)]
mod test {
    use alloc::boxed::Box;
    use alloc::vec;

    use super::*;
    use crate::edn::{Edn, List, Map, Set, Vector};
    use crate::{map, set};

    #[test]
    fn nil_and_empty_edns() {
        assert_eq!(display_as_json(&Edn::Nil), String::from("null"));
        assert_eq!(display_as_json(&Edn::Empty), String::new());
    }

    #[test]
    fn numbers() {
        assert_eq!(display_as_json(&Edn::UInt(34u64)), String::from("34"));
        assert_eq!(display_as_json(&Edn::Int(-25i64)), String::from("-25"));
        assert_eq!(
            display_as_json(&Edn::Double(3.14f64.into())),
            String::from("3.14")
        );
        assert_eq!(
            display_as_json(&Edn::Double(32f64.into())),
            String::from("32.0")
        );
    }

    #[test]
    fn rational_numbers() {
        assert_eq!(
            display_as_json(&Edn::Rational((3, 4))),
            String::from("0.75")
        );
        assert_eq!(
            display_as_json(&Edn::Rational((-3, 9))),
            String::from("-0.3333333333333333")
        );
    }

    #[test]
    fn bools() {
        assert_eq!(display_as_json(&Edn::Bool(true)), String::from("true"));
        assert_eq!(display_as_json(&Edn::Bool(false)), String::from("false"));
    }

    #[test]
    fn chars() {
        assert_eq!(display_as_json(&Edn::Char('e')), String::from("'e'"));
        assert_eq!(display_as_json(&Edn::Char('5')), String::from("'5'"));
    }

    #[test]
    fn strings() {
        let edn = Edn::Str("Hello World".to_string());
        assert_eq!(display_as_json(&edn), "\"Hello World\"".to_string());
    }

    #[test]
    fn symbols() {
        let edn = Edn::Symbol("Hello World".to_string());
        assert_eq!(display_as_json(&edn), "\"Hello World\"".to_string());
    }

    #[test]
    fn keyword() {
        // Don't know what to do with '/'. maybe whitespace?
        let edn = Edn::Key(":hellow-world/again.id".to_string());
        assert_eq!(display_as_json(&edn), "\"hellowWorld/againId\"".to_string());
    }

    #[test]
    fn vector() {
        let edn = Edn::Vector(Vector::new(vec![
            Edn::Bool(true),
            Edn::Key(":b".to_string()),
            Edn::Str("test".to_string()),
            Edn::Char('4'),
            Edn::Rational((-3, 4)),
            Edn::Double(4.5f64.into()),
            Edn::UInt(4),
        ]));
        assert_eq!(
            display_as_json(&edn),
            "[true, \"b\", \"test\", \'4\', -0.75, 4.5, 4]".to_string()
        );
    }

    #[test]
    fn list() {
        let edn = Edn::List(List::new(vec![
            Edn::Bool(true),
            Edn::Key(":b".to_string()),
            Edn::Str("test".to_string()),
            Edn::Char('4'),
            Edn::Rational((-3, 4)),
            Edn::Double(4.5f64.into()),
            Edn::UInt(4),
        ]));
        assert_eq!(
            display_as_json(&edn),
            "[true, \"b\", \"test\", \'4\', -0.75, 4.5, 4]".to_string()
        );
    }

    #[test]
    fn set_test() {
        let edn = Edn::Set(Set::new(set![
            Edn::Bool(true),
            Edn::Key(":my-bestie".to_string()),
            Edn::Str("test".to_string()),
            Edn::Char('4'),
            Edn::Rational((-3, 4)),
            Edn::Double(4.5f64.into()),
            Edn::UInt(4),
        ]));
        let set = display_as_json(&edn);
        assert!(set.contains('['));
        assert!(set.contains(']'));
        assert!(set.contains("-0.75"));
        assert!(set.contains("\"myBestie\""));
    }

    #[test]
    fn simple_map() {
        let map = Edn::Map(Map::new(map! {
            String::from("1.2") => Edn::Bool(false),
            String::from(":belo-monte") => Edn::Rational((3, 4)),
            String::from("true") => Edn::Char('d')
        }));

        assert_eq!(
            display_as_json(&map),
            "{\"1.2\": false, \"beloMonte\": 0.75, \"true\": \'d\'}"
        );
    }

    #[test]
    fn complex_structure() {
        let edn = Edn::Vector(Vector::new(vec![
            Edn::Int(1),
            Edn::Double(1.2.into()),
            Edn::UInt(3),
            Edn::List(List::new(vec![
                Edn::Bool(false),
                Edn::Key(":f".to_string()),
                Edn::Nil,
                Edn::Rational((3, 4)),
                Edn::Set(Set::new(set! {
                    Edn::Rational((3, 4))
                })),
            ])),
            Edn::Map(Map::new(map![
                    String::from("false") => Edn::Key(":f".to_string()),
                    String::from("nil") => Edn::Rational((3, 4)),
                    String::from(":my-crazy-map") => Edn::Map(Map::new(map![
                        String::from("false") => Edn::Map(
                            Map::new( map![
                                String::from(":f") => Edn::Key(String::from(":b"))
                            ])),
                        String::from("nil") => Edn::Vector(
                            Vector::new( vec![
                                Edn::Rational((3, 4)),
                                Edn::Int(1i64)
                            ]))
                ]))
            ])),
        ]));

        assert_eq!(display_as_json(&edn),
            "[1, 1.2, 3, [false, \"f\", null, 0.75, [0.75]], {\"myCrazyMap\": {\"false\": {\"f\": \"b\"}, \"nil\": [0.75, 1]}, \"false\": \"f\", \"nil\": 0.75}]");
    }

    #[test]
    fn tagged_vector() {
        let edn = Edn::Tagged(
            String::from("random/tag"),
            Box::new(Edn::Vector(Vector::new(vec![
                Edn::Bool(true),
                Edn::Key(":b".to_string()),
                Edn::Str("test".to_string()),
                Edn::Char('4'),
                Edn::Rational((-3, 4)),
                Edn::Double(4.5f64.into()),
                Edn::UInt(4),
            ]))),
        );
        assert_eq!(
            display_as_json(&edn),
            "{ \"random/tag\": [true, \"b\", \"test\", \'4\', -0.75, 4.5, 4]}".to_string()
        );
    }
}
