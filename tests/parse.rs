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
        assert_eq!(edn!(shsadc-has), Edn::Symbol(String::from("shsadc-has")));
        assert_eq!(edn!(sym), Edn::Symbol(String::from("sym")));
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
                    Edn::Symbol("sym".to_string()),
                    Edn::Double(1.2),
                    Edn::Int(3),
                    Edn::Bool(false),
                    Edn::Key("f".to_string()),
                    Edn::Nil,
                    Edn::Rational("3/4".to_string())
                ]
            )
        );

        assert_eq!(edn!([ sym 1.2 3 false :f nil 3/4]), expected);
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

    #[test]
    fn parse_complex_vector() {
        let expected = Edn::Vector(
            Vector::new(
                vec![
                    Edn::Int(1),
                    Edn::Double(1.2),
                    Edn::Int(3),
                    Edn::Vector(
                        Vector::new( vec![
                            Edn::Bool(false),
                            Edn::Key("f".to_string()),
                            Edn::Nil,
                            Edn::Rational("3/4".to_string())
                    ]))
                ]
            )
        );

        assert_eq!(edn!([ 1 1.2 3 [false :f nil 3/4]]), expected);
    }

    #[test]
    fn parse_complex_vector_with_list() {
        let expected = Edn::Vector(
            Vector::new(
                vec![
                    Edn::Int(1),
                    Edn::Double(1.2),
                    Edn::Int(3),
                    Edn::List(
                        List::new( vec![
                            Edn::Bool(false),
                            Edn::Key("f".to_string()),
                            Edn::Nil,
                            Edn::Rational("3/4".to_string())
                    ]))
                ]
            )
        );

        assert_eq!(edn!([ 1 1.2 3 (false :f nil 3/4)]), expected);
    }

    #[test]
    fn parse_complex_vector_with_map() {
        let expected = Edn::Vector(
            Vector::new(
                vec![
                    Edn::Int(1),
                    Edn::Double(1.2),
                    Edn::Int(3),
                    Edn::Map(
                        Map::new( map![
                            String::from("false") => Edn::Key("f".to_string()),
                            String::from("nil") => Edn::Rational("3/4".to_string())
                    ]))
                ]
            )
        );

        assert_eq!(edn!([ 1 1.2 3 {false :f nil 3/4}]), expected);
    }

    #[test]
    fn parse_complex_set() {
        let expected = Edn::Set(
            Set::new(
                vec![
                    Edn::Int(1),
                    Edn::Double(1.2),
                    Edn::Int(3),
                    Edn::List(
                        List::new( vec![
                            Edn::Bool(false),
                            Edn::Key("f".to_string()),
                            Edn::Nil,
                            Edn::Rational("3/4".to_string())
                    ])),
                    Edn::Vector(
                        Vector::new( vec![
                            Edn::Bool(true),
                            Edn::Key("b".to_string()),
                            Edn::Rational("12/5".to_string())
                    ]))
                ]
            )
        );

        assert_eq!(edn!(#{ 1 1.2 3 (false :f nil 3/4) [true :b 12/5]}), expected);
    }

    #[test]
    fn parse_complex_list_with_map() {
        let expected = Edn::List(
            List::new(
                vec![
                    Edn::Int(1),
                    Edn::Double(1.2),
                    Edn::Int(3),
                    Edn::Map(
                        Map::new( map![
                            String::from("false") => Edn::Map(
                                Map::new( map![ 
                                    String::from("f") => Edn::Key(String::from("b"))
                                ])),
                            String::from("nil") => Edn::Vector(
                                Vector::new( vec![
                                    Edn::Rational("3/4".to_string()),
                                    Edn::Int(1isize)
                                ]))
                    ]))
                ]
            )
        );

        assert_eq!(edn!(( 1 1.2 3 {false {:f :b} nil [3/4 1]})), expected);
    }
}