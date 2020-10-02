use crate::edn::{rational_to_double, Edn, Vector};

pub(crate) fn to_json(edn: Edn) -> String {
    String::new()
}

pub(crate) fn display_as_json(edn: &Edn) -> String {
    match edn {
        Edn::Vector(v) => vec_to_json(v.to_owned().to_vec()),
        Edn::Set(s) => set_to_json_vec(s.to_owned().to_set()),
        Edn::Map(_) => unimplemented!(),
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
        Edn::NamespacedMap(_, _) => unimplemented!(),
        Edn::Nil => String::from("null"),
        Edn::Empty => String::from(""),
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

    keywrod.trim().replace("-", "").replace(".", "")
}

fn vec_to_json(vec: Vec<Edn>) -> String {
    let vec_str = vec
        .iter()
        .map(|e| display_as_json(e))
        .collect::<Vec<String>>()
        .join(",");
    let mut s = String::from("[");
    s.push_str(&vec_str);
    s.push_str("]");
    s
}

fn set_to_json_vec(set: std::collections::BTreeSet<Edn>) -> String {
    let set_str = set
        .iter()
        .map(|e| display_as_json(e))
        .collect::<Vec<String>>()
        .join(",");
    let mut s = String::from("[");
    s.push_str(&set_str);
    s.push_str("]");
    s
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::edn::{Edn, List, Set, Vector};
    use crate::set;

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
            "[true,\"b\",\"test\",\'4\',-0.75,4.5,4]".to_string()
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
            "[true,\"b\",\"test\",\'4\',-0.75,4.5,4]".to_string()
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
}
