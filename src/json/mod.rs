use crate::edn::{rational_to_double, Edn};

pub(crate) fn to_json(edn: Edn) -> String {
    String::new()
}

pub(crate) fn display_as_json(edn: &Edn) -> String {
    match edn {
        Edn::Vector(_) => unimplemented!(),
        Edn::Set(_) => unimplemented!(),
        Edn::Map(_) => unimplemented!(),
        Edn::List(_) => unimplemented!(),
        Edn::Key(_) => unimplemented!(),
        Edn::Symbol(_) => unimplemented!(),
        Edn::Str(_) => unimplemented!(),
        Edn::Int(n) => format!("{}", n),
        Edn::UInt(n) => format!("{}", n),
        Edn::Double(n) => format!("{}", n),
        Edn::Rational(r) => format!("{}", rational_to_double(r).unwrap()),
        Edn::Char(_) => unimplemented!(),
        Edn::Bool(_) => unimplemented!(),
        Edn::Inst(_) => unimplemented!(),
        Edn::Uuid(_) => unimplemented!(),
        Edn::NamespacedMap(_, _) => unimplemented!(),
        Edn::Nil => String::from("null"),
        Edn::Empty => String::from(""),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::edn::Edn;

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
}
