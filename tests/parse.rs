#[macro_use] extern crate edn_rs;

// TODO: 
// 1. Sequences with rational
// 2. chars
// 3. keywords
// 4. symbols
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::edn_rs::{
        edn::{Edn, Set, Vector, List, Map},
        macros::*,
    };

    #[test]
    fn parse_primitive_types() {
        assert_eq!(edn!(1), Edn::Int(1));
        assert_eq!(edn!(12.5), Edn::Double(12.5));
        // assert_eq!(edn!(\\c), Edn::Char('c'));
        // assert_eq!(edn!(:key-word), Edn::Key(":key-word".to_string()));
        assert_eq!(edn!("this is a string"), Edn::Str("\"this is a string\"".to_string()));
        // assert_eq!(edn!(my-symbol), Edn::Symbol("my-symbol".to_string()));
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
    fn parse_simple_list() {
        let expected = Edn::List(
            List::new(
                vec![
                    Edn::Nil,
                    Edn::Bool(true),
                    Edn::Int(2),
                ]
            )
        );

        assert_eq!(edn!((nil true 2)), expected);
    }

    #[test]
    fn parse_simple_vector() {
        let expected = Edn::Vector(
            Vector::new(
                vec![
                    Edn::Int(1),
                    Edn::Bool(false),
                    Edn::Double(1.2),
                    Edn::Int(4),
                ]
            )
        );

        assert_eq!(edn!([ 1 false 1.2 4 ]), expected);
    }

//     #[test]
//     fn parse_simple_set() {
//         let list = String::from("#{3/4 false 1.2 :bb-8 my-symbol}");
//         let expected = Edn::Set(
//             Set::new(
//                 vec![
//                     Edn::Rational("3/4".to_string()),
//                     Edn::Bool(false),
//                     Edn::Double(1.2),
//                     Edn::Key(":bb-8".to_string()),
//                     Edn::Symbol("my-symbol".to_string())
//                 ]
//             )
//         );

//         assert_eq!(parse_edn(list), expected);
//     }

//     #[test]
//     fn parse_simple_map() {
//         let list = String::from("{3/4 false 1.2 :bb-8}");
//         let mut hm = HashMap::new();
//         hm.insert("3/4".to_string(), Edn::Bool(false));
//         hm.insert("1.2".to_string(), Edn::Key(":bb-8".to_string()));
//         let expected = Edn::Map(
//             Map::new(hm)
//         );

//         assert_eq!(parse_edn(list), expected);
//     }
}