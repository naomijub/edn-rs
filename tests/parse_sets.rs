#[cfg(feature = "sets")]
#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::collections::{BTreeMap, BTreeSet};

    use edn_rs::{edn, set, Edn, List, Map, Set, Vector};

    #[test]
    fn parse_empty_structures() {
        assert_eq!(edn!([]), Edn::Vector(Vector::new(Vec::new())));
        assert_eq!(edn!(()), Edn::List(List::new(Vec::new())));
        assert_eq!(edn!(#{}), Edn::Set(Set::new(BTreeSet::new())));
        assert_eq!(edn!({}), Edn::Map(Map::new(BTreeMap::new())));
    }

    #[test]
    fn parse_simple_set() {
        let expected = Edn::Set(Set::new(set! {
            Edn::Int(1),
            Edn::Double(1.2.into()),
            Edn::Int(3),
            Edn::Bool(false),
            Edn::Key(":f".to_string()),
            Edn::Nil,
            Edn::Rational((3, 4))
        }));

        assert_eq!(edn!(#{1 1.2 3 false :f nil 3/4}), expected);
    }

    #[test]
    fn parse_complex_set() {
        let expected = Edn::Set(Set::new(set! {
            Edn::Int(1),
            Edn::Double(1.2.into()),
            Edn::Int(3),
            Edn::List(
                List::new( vec![
                    Edn::Bool(false),
                    Edn::Key(":f".to_string()),
                    Edn::Nil,
                    Edn::Rational((3, 4))
            ])),
            Edn::Vector(
                Vector::new( vec![
                    Edn::Bool(true),
                    Edn::Key(":b".to_string()),
                    Edn::Rational((12, 5))
            ]))
        }));

        assert_eq!(
            edn!(#{ 1 1.2 3 (false :f nil 3/4) [true :b 12/5]}),
            expected
        );
    }
}
