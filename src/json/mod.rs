use crate::edn::{rational_to_double, Edn};

pub(crate) fn display_as_json(edn: &Edn) -> String {
    match edn {
        Edn::Vector(v) => vec_to_json(v.to_owned().to_vec()),
        Edn::Set(s) => set_to_json_vec(s.to_owned().to_set()),
        Edn::Map(map) => map_to_json(map.to_owned().to_map()),
        Edn::List(l) => vec_to_json(l.to_owned().to_vec()),
        Edn::Key(key) => format!("{:?}", kebab_to_camel(key)),
        Edn::Symbol(s) => format!("{:?}", s),
        Edn::Str(s) => format!("{:?}", s),
        Edn::Int(n) => format!("{}", n),
        Edn::UInt(n) => format!("{}", n),
        Edn::Double(n) => format!("{}", n),
        Edn::Rational(r) => format!("{}", rational_to_double(r).unwrap()),
        Edn::Char(c) => format!("'{}'", c),
        Edn::Bool(b) => format!("{}", b),
        Edn::Inst(inst) => format!("{:?}", inst),
        Edn::Uuid(uuid) => format!("{:?}", uuid),
        Edn::NamespacedMap(ns, map) => nsmap_to_json(ns, map.to_owned().to_map()),
        Edn::Nil => String::from("null"),
        Edn::Empty => String::from(""),
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

fn vec_to_json(vec: Vec<Edn>) -> String {
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

fn set_to_json_vec(set: std::collections::BTreeSet<Edn>) -> String {
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

fn map_to_json(map: std::collections::BTreeMap<String, Edn>) -> String {
    let map_str = map
        .iter()
        .map(|(k, e)| {
            let key = if k.starts_with(':') {
                kebab_to_camel(k)
            } else {
                k.to_string()
            };
            let edn = display_as_json(e);

            format!("{:?}: {}", key, edn)
        })
        .collect::<Vec<String>>()
        .join(", ");
    let mut s = String::from("{");
    s.push_str(&map_str);
    s.push('}');
    s
}

fn nsmap_to_json(ns: &str, map: std::collections::BTreeMap<String, Edn>) -> String {
    let mut s = String::from("{");
    let map_str = map_to_json(map);
    s.push_str(&format!("{:?}: ", kebab_to_camel(ns)));
    s.push_str(&map_str);
    s.push('}');
    s
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::edn::{Edn, List, Map, Set, Vector};
    use crate::{map, set};

    #[test]
    fn nil_and_empty_edns() {
        assert_eq!(display_as_json(&Edn::Nil), String::from("null"));
        assert_eq!(display_as_json(&Edn::Empty), String::from(""));
    }

    #[test]
    fn numbers() {
        assert_eq!(display_as_json(&Edn::UInt(34usize)), String::from("34"));
        assert_eq!(display_as_json(&Edn::Int(-25isize)), String::from("-25"));
        assert_eq!(
            display_as_json(&Edn::Double(3.14f64.into())),
            String::from("3.14")
        );
    }

    #[test]
    fn rational_numbers() {
        assert_eq!(
            display_as_json(&Edn::Rational("3/4".to_string())),
            String::from("0.75")
        );
        assert_eq!(
            display_as_json(&Edn::Rational("-3/9".to_string())),
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
    fn inst_and_uuid() {
        let inst = Edn::Inst("2020-09-18T01:16:25.909-00:00".to_string());
        let uuid = Edn::Uuid("af6d8699-f442-4dfd-8b26-37d80543186b".to_string());

        assert_eq!(
            display_as_json(&inst),
            "\"2020-09-18T01:16:25.909-00:00\"".to_string()
        );
        assert_eq!(
            display_as_json(&uuid),
            "\"af6d8699-f442-4dfd-8b26-37d80543186b\"".to_string()
        );
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
            Edn::Rational("-3/4".to_string()),
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
            Edn::Rational("-3/4".to_string()),
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
            Edn::Rational("-3/4".to_string()),
            Edn::Double(4.5f64.into()),
            Edn::UInt(4),
        ]));
        let set = display_as_json(&edn);
        assert!(set.contains("["));
        assert!(set.contains("]"));
        assert!(set.contains("-0.75"));
        assert!(set.contains("\"myBestie\""));
    }

    #[test]
    fn simple_map() {
        let map = Edn::Map(Map::new(map! {
            String::from("1.2") => Edn::Bool(false),
            String::from(":belo-monte") => Edn::Rational(String::from("3/4")),
            String::from("true") => Edn::Char('d')
        }));

        assert_eq!(
            display_as_json(&map),
            "{\"1.2\": false, \"beloMonte\": 0.75, \"true\": \'d\'}"
        )
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
                Edn::Rational("3/4".to_string()),
                Edn::Set(Set::new(set! {
                    Edn::Rational("3/4".to_string())
                })),
            ])),
            Edn::Map(Map::new(map![
                    String::from("false") => Edn::Key(":f".to_string()),
                    String::from("nil") => Edn::Rational("3/4".to_string()),
                    String::from(":my-crazy-map") => Edn::Map(Map::new(map![
                        String::from("false") => Edn::Map(
                            Map::new( map![
                                String::from(":f") => Edn::Key(String::from(":b"))
                            ])),
                        String::from("nil") => Edn::Vector(
                            Vector::new( vec![
                                Edn::Rational("3/4".to_string()),
                                Edn::Int(1isize)
                            ]))
                ]))
            ])),
        ]));

        assert_eq!(display_as_json(&edn),
            "[1, 1.2, 3, [false, \"f\", null, 0.75, [0.75]], {\"myCrazyMap\": {\"false\": {\"f\": \"b\"}, \"nil\": [0.75, 1]}, \"false\": \"f\", \"nil\": 0.75}]");
    }

    #[test]
    fn simple_namespaced_map() {
        let map = Edn::NamespacedMap(
            String::from("this-is-a-namespace"),
            Map::new(map! {
                String::from("1.2") => Edn::Bool(false),
                String::from(":belo-monte") => Edn::Rational(String::from("3/4")),
                String::from("true") => Edn::Char('d')
            }),
        );

        assert_eq!(
            display_as_json(&map),
            "{\"thisIsANamespace\": {\"1.2\": false, \"beloMonte\": 0.75, \"true\": \'d\'}}"
        )
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
                Edn::Rational("-3/4".to_string()),
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
