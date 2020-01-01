#![recursion_limit="512"]

#[macro_use] extern crate edn_rs;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::edn_rs::{
        edn::{Edn, Set, Vector, List, Map},
    };

    #[test]
    fn parse_primitive_types() {
        assert_eq!(edn!(1), Edn::Int(1));
        assert_eq!(edn!(12.5), Edn::Double(12.5));
        assert_eq!(edn!(:key), Edn::Key("key".to_string()));
        assert_eq!(edn!("this is a string"), Edn::Str("this is a string".to_string()));
        assert_eq!(edn!(3/4), Edn::Rational("3/4".to_string()));
        assert_eq!(edn!(true), Edn::Bool(true));
        assert_eq!(edn!(false), Edn::Bool(false));
        assert_eq!(edn!(nil), Edn::Nil);
    }

    #[test]
    fn parse_empty_structures() {
        assert_eq!(edn!([]), Edn::Vector(Vector::new(Vec::new())));
        assert_eq!(edn!(()), Edn::List(List::new(Vec::new())));
        assert_eq!(edn!(#{}), Edn::Set(Set::new(Vec::new())));
        assert_eq!(edn!({}), Edn::Map(Map::new(HashMap::new())));
    }

    #[test]
    fn parse_simple_vector() {
        let expected = Edn::Vector(
            Vector::new(
                vec![
                    Edn::Int(1),
                    Edn::Double(1.2),
                    Edn::Int(3),
                    Edn::Bool(false),
                    Edn::Key("f".to_string()),
                    Edn::Nil,
                    Edn::Rational("3/4".to_string())
                ]
            )
        );

        assert_eq!(edn!([ 1 1.2 3 false :f nil 3/4]), expected);
    }

    #[test]
    fn parse_simple_list() {
        let expected = Edn::List(
            List::new(
                vec![
                    Edn::Int(1),
                    Edn::Double(1.2),
                    Edn::Int(3),
                    Edn::Bool(false),
                    Edn::Key("f".to_string()),
                    Edn::Nil,
                    Edn::Rational("3/4".to_string())
                ]
            )
        );

        assert_eq!(edn!((1 1.2 3 false :f nil 3/4)), expected);
    }

    #[test]
    fn parse_simple_set() {
        let expected = Edn::Set(
            Set::new(
                vec![
                    Edn::Int(1),
                    Edn::Double(1.2),
                    Edn::Int(3),
                    Edn::Bool(false),
                    Edn::Key("f".to_string()),
                    Edn::Nil,
                    Edn::Rational("3/4".to_string())
                ]
            )
        );

        assert_eq!(edn!(#{1 1.2 3 false :f nil 3/4}), expected);
    }

    #[test]
    fn parse_simple_map() {
        let expected = Edn::Map(
            Map::new(
                map!{
                    String::from("1.2") => Edn::Bool(false),
                    String::from("b") => Edn::Rational(String::from("3/4"))
                }
            )
        );

        assert_eq!(edn!({1.2 false, :b 3/4}), expected);
    }
}