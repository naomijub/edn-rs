extern crate edn_rs;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::edn_rs::{
        edn::{Edn, Set, Vector, List, Map},
        parse_edn,
    };

    #[test]
    fn parse_primitive_types() {
        assert_eq!(parse_edn("1".to_string()), Edn::Int(1));
        assert_eq!(parse_edn("12.5".to_string()), Edn::Double(12.5));
        assert_eq!(parse_edn("\\c".to_string()), Edn::Char('c'));
        assert_eq!(parse_edn(":key-word".to_string()), Edn::Key(":key-word".to_string()));
        assert_eq!(parse_edn("\"this is a string\"".to_string()), Edn::Str("\"this is a string\"".to_string()));
        assert_eq!(parse_edn("my-symbol".to_string()), Edn::Symbol("my-symbol".to_string()));
        assert_eq!(parse_edn("3/4".to_string()), Edn::Rational("3/4".to_string()));
        assert_eq!(parse_edn("true".to_string()), Edn::Bool(true));
        assert_eq!(parse_edn("".to_string()), Edn::Nil);
    }

    #[test]
    fn parse_empty_structures() {
        assert_eq!(parse_edn(String::from("[]")), Edn::Vector(Vector::new(Vec::new())));
        assert_eq!(parse_edn(String::from("()")), Edn::List(List::new(Vec::new())));
        assert_eq!(parse_edn(String::from("#{}")), Edn::Set(Set::new(Vec::new())));
        assert_eq!(parse_edn(String::from("{}")), Edn::Map(Map::new(HashMap::new())));
    }

    #[test]
    fn parse_simple_list() {
        let list = String::from("(nil true 1 :b \\c)");
        let expected = Edn::List(
            List::new(
                vec![
                    Edn::Nil,
                    Edn::Bool(true),
                    Edn::Int(1),
                    Edn::Key(":b".to_string()),
                    Edn::Char('c')
                ]
            )
        );

        assert_eq!(parse_edn(list), expected);
    }

    #[test]
    fn parse_simple_vector() {
        let list = String::from("[3/4 false 1.2 :bb-8 my-symbol]");
        let expected = Edn::Vector(
            Vector::new(
                vec![
                    Edn::Rational("3/4".to_string()),
                    Edn::Bool(false),
                    Edn::Double(1.2),
                    Edn::Key(":bb-8".to_string()),
                    Edn::Symbol("my-symbol".to_string())
                ]
            )
        );

        assert_eq!(parse_edn(list), expected);
    }

    #[test]
    fn parse_simple_set() {
        let list = String::from("#{3/4 false 1.2 :bb-8 my-symbol}");
        let expected = Edn::Set(
            Set::new(
                vec![
                    Edn::Rational("3/4".to_string()),
                    Edn::Bool(false),
                    Edn::Double(1.2),
                    Edn::Key(":bb-8".to_string()),
                    Edn::Symbol("my-symbol".to_string())
                ]
            )
        );

        assert_eq!(parse_edn(list), expected);
    }

    #[test]
    fn parse_simple_map() {
        let list = String::from("{3/4 false 1.2 :bb-8}");
        let mut hm = HashMap::new();
        hm.insert("3/4".to_string(), Edn::Bool(false));
        hm.insert("1.2".to_string(), Edn::Key(":bb-8".to_string()));
        let expected = Edn::Map(
            Map::new(hm)
        );

        assert_eq!(parse_edn(list), expected);
    }
}